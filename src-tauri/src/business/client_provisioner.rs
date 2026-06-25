use crate::business::event_store::Event;
use crate::error::Result;
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProvisionProgress {
  pub percentage: u8,
  pub message: String,
}

#[async_trait]
pub trait ClientProvisioner: Send + Sync {
  async fn provision(&self, event: &Event, install_dir: String) -> Result<()>;
  async fn is_provisioned(&self, event: &Event, install_dir: String) -> bool;
}
