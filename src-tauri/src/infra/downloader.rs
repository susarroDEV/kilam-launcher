use std::collections::HashMap;
use std::fs::{read, remove_file, rename};
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use reqwest::Client;
use sha2::{Digest, Sha256};
use tauri::Emitter;

use crate::business::downloader::{
  DownloadOutcome, DownloadProgress, DownloadResult, DownloadStatus, Downloader, DownloaderError,
};
use crate::business::event_store::{Asset, Event};
use crate::error::Result;

fn verify_sha256(path: &PathBuf, expected_hash: String) -> Result<()> {
  match read(path) {
    Ok(bytes) => {
      let hash = Sha256::digest(&bytes);
      let hex_string = hex::encode(hash);

      if hex_string != expected_hash {
        Err(
          DownloaderError::VerificationFailed(format!(
            "Hash mismatch: expected {}, got {}",
            expected_hash, hex_string
          ))
          .into(),
        )
      } else {
        Ok(())
      }
    }
    Err(e) => Err(DownloaderError::Io(e).into()),
  }
}

pub struct HttpDownloader {
  app_handle: tauri::AppHandle,
  client: Client,
}

impl HttpDownloader {
  pub fn new(app_handle: tauri::AppHandle, client: Client) -> Self {
    Self { app_handle, client }
  }

  async fn download_asset(&self, asset: &Asset, event_id: &str, install_dir: &str) -> Result<()> {
    let mut response = self
      .client
      .get(asset.url.clone())
      .send()
      .await
      .map_err(DownloaderError::DownloadFailed)?;

    let total_bytes = response.content_length().unwrap_or(0);
    let tmp_path = std::path::PathBuf::from(format!("{}/{}.tmp", install_dir, asset.path));
    let path = std::path::PathBuf::from(format!("{}/{}", install_dir, asset.path));
    let dir = tmp_path.parent();

    if let Some(dir) = dir {
      std::fs::create_dir_all(dir).map_err(DownloaderError::Io)?;
    }

    let mut file = std::fs::File::create(&tmp_path).map_err(DownloaderError::Io)?;
    let mut downloaded_bytes: u64 = 0;

    while let Some(chunk) = response
      .chunk()
      .await
      .map_err(DownloaderError::DownloadFailed)?
    {
      file.write_all(&chunk).map_err(DownloaderError::Io)?;
      downloaded_bytes += chunk.len() as u64;
      self.app_handle.emit(
        "download:progress",
        DownloadProgress {
          event_id: event_id.to_string(),
          asset_id: asset.id.clone(),
          downloaded_bytes,
          total_bytes,
          status: DownloadStatus::Downloading,
        },
      )?;
    }

    match verify_sha256(&tmp_path, asset.sha256.clone()) {
      Ok(_) => {
        rename(tmp_path, path).map_err(DownloaderError::Io)?;
      }
      Err(e) => {
        self.app_handle.emit(
          "download:complete",
          DownloadResult {
            event_id: event_id.to_string(),
            outcome: DownloadOutcome::Failure(e.to_string()),
          },
        )?;
        remove_file(tmp_path).map_err(DownloaderError::Io)?;
        return Err(e);
      }
    }

    Ok(())
  }

  async fn build_disk_hashes(install_dir: String, event: Event) -> HashMap<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
      let mut assets: HashMap<String, String> = HashMap::new();

      for asset in &event.assets {
        let complete_path = format!("{}/{}", install_dir, asset.path);
        match read(complete_path) {
          Err(_) => {
            continue;
          }
          Ok(bytes) => {
            let hash = Sha256::digest(&bytes);
            let hex_string = hex::encode(hash);

            assets.insert(asset.id.clone(), hex_string);
          }
        }
      }

      assets
    })
    .await
    .unwrap_or_default()
  }
}

#[async_trait]
impl Downloader for HttpDownloader {
  async fn download_event(&self, event: Event, install_dir: String) -> Result<()> {
    for asset in &event.assets {
      self.download_asset(asset, &event.id, &install_dir).await?;
    }

    self.app_handle.emit(
      "download:complete",
      DownloadResult {
        event_id: event.id.clone(),
        outcome: DownloadOutcome::Success,
      },
    )?;

    Ok(())
  }

  async fn is_ready(&self, event: &Event, install_dir: &str) -> bool {
    let disk_hashes = HttpDownloader::build_disk_hashes(install_dir.to_string(), event.clone()).await;

    for asset in &event.assets {
      match disk_hashes.get(&asset.id) {
        None => return false,
        Some(hash) if hash != &asset.sha256 => {
          return false
        }
        _ => {}
      }
    }
    true
  }

}
