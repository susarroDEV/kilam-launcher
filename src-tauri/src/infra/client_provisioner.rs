use crate::LauncherError;
use crate::business::client_provisioner::{ClientProvisioner, ClientProvisionerError};
use crate::business::event_store::Event;
use crate::error::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangVersionEntry {
  id: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangVersionsIndex {
  versions: Vec<MojangVersionEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangClient {
  url: String,
  sha1: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangClientDownload {
  client: MojangClient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangArtifact {
  url: String,
  sha1: String,
  path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangNativeArtifact {
  url: String,
  sha1: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangLibraryDownload {
  artifact: MojangArtifact,
  #[serde(default)]
  classifiers: HashMap<String, MojangNativeArtifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangLibrary {
  name: String,
  downloads: MojangLibraryDownload,
  #[serde(default)]
  natives: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MojangVersionManifest {
  downloads: MojangClientDownload,
  libraries: Vec<MojangLibrary>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FabricLibrary {
  name: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FabricManifest {
  libraries: Vec<FabricLibrary>,
}

fn maven_name_to_path(name: &str) -> String {
  let fract: Vec<&str> = name.split(':').collect();
  format!(
    "{}/{}/{}/{}-{}.jar",
    fract[0].replace('.', "/"),
    fract[1],
    fract[2],
    fract[1],
    fract[2]
  )
}

pub struct MojangClientProvisioner {
  client: Client,
}

impl MojangClientProvisioner {
  pub fn new(client: Client) -> Self {
    Self { client }
  }

  async fn download_file(
    &self,
    url: &str,
    dest_path: &std::path::Path,
    sha1: &str,
  ) -> std::result::Result<(), ClientProvisionerError> {
    let tmp_path = dest_path.with_extension("tmp");

    if let Some(dir) = dest_path.parent() {
      std::fs::create_dir_all(dir).map_err(ClientProvisionerError::ExtractionFailed)?;
    }

    let mut response = self
      .client
      .get(url)
      .send()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?;

    let mut bytes = Vec::new();
    while let Some(chunk) = response
      .chunk()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?
    {
      bytes.extend_from_slice(&chunk);
    }

    let sha1_str = sha1.to_string();
    let tmp_path_clone = tmp_path.clone();
    let dest_path_buf = dest_path.to_path_buf();

    tauri::async_runtime::spawn_blocking(
      move || -> std::result::Result<(), ClientProvisionerError> {
        use sha1::{Digest, Sha1};

        std::fs::write(&tmp_path_clone, &bytes)
          .map_err(ClientProvisionerError::ExtractionFailed)?;

        if !sha1_str.is_empty() {
          let hash = hex::encode(Sha1::digest(&bytes));
          if hash != sha1_str {
            std::fs::remove_file(&tmp_path_clone)
              .map_err(ClientProvisionerError::ExtractionFailed)?;
            return Err(ClientProvisionerError::VerificationFailed(
              "SHA1 mismatch".to_string(),
            ));
          }
        }

        std::fs::rename(&tmp_path_clone, &dest_path_buf)
          .map_err(ClientProvisionerError::ExtractionFailed)?;

        Ok(())
      },
    )
    .await
    .map_err(|e| {
      ClientProvisionerError::ExtractionFailed(std::io::Error::new(std::io::ErrorKind::Other, e))
    })?
  }

  async fn extract_natives(
    &self,
    event: &Event,
    install_dir: &str,
    version: &MojangVersionManifest,
  ) -> std::result::Result<(), ClientProvisionerError> {
    let os_key = if cfg!(target_os = "windows") {
      "windows"
    } else if cfg!(target_os = "macos") {
      "osx"
    } else {
      "linux"
    };

    let natives_dir =
      std::path::PathBuf::from(format!("{}/{}/natives", install_dir, event.id));
    std::fs::create_dir_all(&natives_dir).map_err(ClientProvisionerError::ExtractionFailed)?;

    for library in &version.libraries {
      if let Some(classifier_key) = library.natives.get(os_key) {
        if let Some(native) = library.downloads.classifiers.get(classifier_key) {
          let mut response = self
            .client
            .get(&native.url)
            .send()
            .await
            .map_err(ClientProvisionerError::FetchFailed)?;

          let mut bytes = Vec::new();
          while let Some(chunk) = response
            .chunk()
            .await
            .map_err(ClientProvisionerError::FetchFailed)?
          {
            bytes.extend_from_slice(&chunk);
          }

          let native_sha1 = native.sha1.clone();
          let natives_dir_clone = natives_dir.clone();

          tauri::async_runtime::spawn_blocking(
            move || -> std::result::Result<(), ClientProvisionerError> {
              use sha1::{Digest, Sha1};

              let hash = hex::encode(Sha1::digest(&bytes));
              if hash != native_sha1 {
                return Err(ClientProvisionerError::VerificationFailed(
                  "Native SHA1 mismatch".to_string(),
                ));
              }

              let cursor = std::io::Cursor::new(&bytes);
              let mut archive = zip::ZipArchive::new(cursor).map_err(|e| {
                ClientProvisionerError::ExtractionFailed(std::io::Error::new(
                  std::io::ErrorKind::Other,
                  e,
                ))
              })?;

              for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|e| {
                  ClientProvisionerError::ExtractionFailed(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e,
                  ))
                })?;

                if file.name().ends_with('/') {
                  continue;
                }

                let out_path = natives_dir_clone.join(file.name());
                if let Some(parent) = out_path.parent() {
                  std::fs::create_dir_all(parent)
                    .map_err(ClientProvisionerError::ExtractionFailed)?;
                }

                let mut out_file = std::fs::File::create(&out_path)
                  .map_err(ClientProvisionerError::ExtractionFailed)?;
                std::io::copy(&mut file, &mut out_file)
                  .map_err(ClientProvisionerError::ExtractionFailed)?;
              }

              Ok(())
            },
          )
          .await
          .map_err(|e| {
            ClientProvisionerError::ExtractionFailed(std::io::Error::new(
              std::io::ErrorKind::Other,
              e,
            ))
          })??;
        }
      }
    }

    Ok(())
  }
}

#[async_trait]
impl ClientProvisioner for MojangClientProvisioner {
  async fn provision(&self, event: &Event, install_dir: String) -> Result<()> {
    let index = self
      .client
      .get(MANIFEST_URL)
      .send()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?
      .json::<MojangVersionsIndex>()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?;

    let version_entry = index
      .versions
      .iter()
      .find(|v| v.id == event.minecraft_version)
      .ok_or(ClientProvisionerError::ParseFailed(
        "Version not found".to_string(),
      ))?;

    let version = self
      .client
      .get(&version_entry.url)
      .send()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?
      .json::<MojangVersionManifest>()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?;

    let client_path =
      std::path::PathBuf::from(format!("{}/{}/client.jar", install_dir, event.id));
    self
      .download_file(
        &version.downloads.client.url,
        &client_path,
        &version.downloads.client.sha1,
      )
      .await
      .map_err(|e| LauncherError::ClientProvisioner(e))?;

    let libraries_path =
      std::path::PathBuf::from(format!("{}/{}/libraries", install_dir, event.id));

    for library in &version.libraries {
      let lib_path = libraries_path.join(&library.downloads.artifact.path);
      self
        .download_file(
          &library.downloads.artifact.url,
          &lib_path,
          &library.downloads.artifact.sha1,
        )
        .await
        .map_err(|e| LauncherError::ClientProvisioner(e))?;
    }

    let fabric_url = format!(
      "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
      &event.minecraft_version, &event.modloader_version
    );

    let fabric_manifest = self
      .client
      .get(fabric_url)
      .send()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?
      .json::<FabricManifest>()
      .await
      .map_err(ClientProvisionerError::FetchFailed)?;

    for fabric_library in &fabric_manifest.libraries {
      let lib_path = maven_name_to_path(&fabric_library.name);
      let full_path = libraries_path.join(&lib_path);
      let download_url = format!("{}{}", fabric_library.url, lib_path);
      self
        .download_file(&download_url, &full_path, "")
        .await
        .map_err(|e| LauncherError::ClientProvisioner(e))?;
    }

    self
      .extract_natives(event, &install_dir, &version)
      .await
      .map_err(|e| LauncherError::ClientProvisioner(e))?;

    Ok(())
  }

  async fn is_provisioned(&self, event: &Event, install_dir: String) -> bool {
    std::path::Path::new(&format!("{}/{}/client.jar", install_dir, event.id)).exists()
  }
}
