use serde::Serialize;
use thiserror::Error;

use crate::business;

#[derive(Error, Debug)]
pub enum LauncherError {
  #[error("Error in Plugin Store: {0}")]
  Store(#[from] tauri_plugin_store::Error),
  #[error("Error in Serde JSON: {0}")]
  SerdeJson(#[from] serde_json::Error),
  #[error("Error in Authentication: {0}")]
  Auth(#[from] business::auth::AuthError),
  #[error("Error in Event Store {0}")]
  EventStore(#[from] business::event_store::EventError),
  #[error("Error in Tauri {0}")]
  Tauri(#[from] tauri::Error),
  #[error("Error in Downloader {0}")]
  Downloader(#[from] business::downloader::DownloaderError),
  #[error("Error in Client Provisioner {0}")]
  ClientProvisioner(#[from] business::client_provisioner::ClientProvisionerError),
}

impl Serialize for LauncherError {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

pub type Result<T> = std::result::Result<T, LauncherError>;
