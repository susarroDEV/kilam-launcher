use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventIndex {
  pub active_events: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
  pub id: String,
  pub url: String,
  pub sha256: String,
  pub path: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ModLoader {
  Fabric,
  Forge,
  Vanilla
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
  NotInstalled,
  Outdated,
  Ready
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
  pub assets: Vec<Asset>
}

pub trait EventStore {
  async fn get_active_events(&self, uuid: String) -> Result<Vec<EventDTO>>;
}

#[derive(Error, Debug)]
pub enum EventError {
  #[error("Fetch has failed: {0}")]
  FetchFailed(#[from] reqwest::Error),
  #[error("Parse has failed")]
  ParseFailed,
  #[error("Event [{0}] was not found")]
  EventNotFound(String),
  #[error("No available events for [{0}] player")]
  EmptyAccesibleEvents(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventDTO {
  pub event: Event,
  pub status: EventStatus
}

pub fn calculate_status(event: &Event, disk_hashes: HashMap<String, String>) -> EventStatus {
  for asset in &event.assets {
    match disk_hashes.get(&asset.id) {
      None => return EventStatus::NotInstalled,
      Some(hash) 
        if hash != &asset.sha256 => {
          return EventStatus::Outdated;
      }
      _ => {}
    }
  }

  return EventStatus::Ready
}
