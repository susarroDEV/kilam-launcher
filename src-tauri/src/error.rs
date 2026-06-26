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
  #[error("Error in Launcher {0}")]
  Launcher(#[from] business::launcher::LaunchError),
}

impl LauncherError {
  fn kind(&self) -> &'static str {
    match self {
      Self::Store(_) => "Store",
      Self::SerdeJson(_) => "SerdeJson",
      Self::Auth(_) => "Auth",
      Self::EventStore(_) => "EventStore",
      Self::Tauri(_) => "Tauri",
      Self::Downloader(_) => "Downloader",
      Self::ClientProvisioner(_) => "ClientProvisioner",
      Self::Launcher(_) => "Launcher",
    }
  }
}

#[derive(Serialize)]
struct ErrorPayload<'a> {
  kind: &'a str,
  message: String,
}

impl Serialize for LauncherError {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    ErrorPayload {
      kind: self.kind(),
      message: self.to_string(),
    }
    .serialize(serializer)
  }
}

pub type Result<T> = std::result::Result<T, LauncherError>;
