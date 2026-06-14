use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error::Result;
use std::io;

use crate::business::event_store::Event;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
  Pending,
  Downloading,
  Done,
  Failed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadProgress {
  pub event_id: String,
  pub asset_id: String,
  pub downloaded_bytes: u64,
  pub total_bytes: u64,
  pub status: DownloadStatus
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DownloadOutcome {
  Success,
  Failure(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadResult {
  pub event_id: String,
  pub outcome: DownloadOutcome
}

#[derive(Error, Debug)]
pub enum DownloaderError {
  #[error("Error in Input/Output: {0}")]
  Io(#[from] io::Error),
  #[error("The download has failed: {0}")]
  DownloadFailed(#[from] reqwest::Error),
  #[error("The verification has failed: {0}")]
  VerificationFailed(String)
}

#[async_trait]
pub trait Downloader {
  async fn download_event(&self, event: Event, install_dir: String) -> Result<()>;
}
