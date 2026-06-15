use crate::business::downloader::Downloader; 
use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventIndex {
  pub active_events: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
  pub id: String,
  pub url: String,
  pub sha256: String,
  pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ModLoader {
  Fabric,
  Forge,
  Vanilla,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
  NotInstalled,
  Outdated,
  Ready,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  pub id: String,
  pub name: String,
  pub description: String,
  pub image_url: String,
  pub minecraft_version: String,
  pub modloader: ModLoader,
  pub modloader_version: String,
  pub server_ip: String,
  pub whitelist: Vec<String>,
  pub assets: Vec<Asset>,
}

#[async_trait]
pub trait EventStore {
  async fn get_active_events(&self, uuid: String, install_dir: String) -> Result<Vec<EventDTO>>;
}

#[derive(Error, Debug)]
pub enum EventError {
  #[error("Fetch has failed: {0}")]
  FetchFailed(#[from] reqwest::Error),
  #[error("Parse has failed")]
  ParseFailed,
  #[error("Event [{0}] was not found")]
  EventNotFound(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventDTO {
  pub event: Event,
  pub status: EventStatus,
}

pub async fn calculate_status(event: &Event, downloader: &dyn Downloader, install_dir: &str) -> EventStatus {
  if downloader.is_ready(event, install_dir).await
  { EventStatus::Ready }
  else { EventStatus::NotInstalled }
}
