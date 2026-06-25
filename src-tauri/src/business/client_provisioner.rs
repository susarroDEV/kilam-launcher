use crate::business::event_store::Event;
use crate::error::Result;
use async_trait::async_trait;
use reqwest;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientProvisionerError {
  #[error("Fetch failed: {0}")]
  FetchFailed(#[from] reqwest::Error),
  #[error("Parse failed: {0}")]
  ParseFailed(String),
  #[error("Download failed: {0}")]
  DownloadFailed(String),
  #[error("Verification failed: {0}")]
  VerificationFailed(String),
  #[error("Extraction failed: {0}")]
  ExtractionFailed(#[from] io::Error),
}

#[async_trait]
pub trait ClientProvisioner: Send + Sync {
  async fn provision(&self, event: &Event, install_dir: String) -> Result<()>;
  async fn is_provisioned(&self, event: &Event, install_dir: String) -> bool;
}
