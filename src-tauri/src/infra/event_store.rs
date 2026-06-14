use std::collections::HashMap;
use std::fs::read;

use crate::business::event_store::{
  calculate_status, Event, EventDTO, EventError, EventIndex, EventStore,
};
use crate::error::Result;
use async_trait::async_trait;
use md5::Digest;
use reqwest::Client;
use sha2::Sha256;

const MANIFEST_URL: &str = "https://gist.githubusercontent.com/susarroDEV/c110ce866f2cfec390d03f117b2c54b6/raw/gistfile1.txt";

pub struct RemoteEventStore {
  client: Client,
}

impl RemoteEventStore {
  pub fn new(client: Client) -> Self {
    Self {
      client
    }
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
impl EventStore for RemoteEventStore {
  async fn get_active_events(&self, uuid: String, install_dir: String) -> Result<Vec<EventDTO>> {
    let index = self
      .client
      .get(MANIFEST_URL)
      .send()
      .await
      .map_err(EventError::FetchFailed)?
      .json::<EventIndex>()
      .await
      .map_err(EventError::FetchFailed)?;

    let mut events: Vec<Event> = Vec::new();

    for url in &index.active_events {
      events.push(
        self
          .client
          .get(url)
          .send()
          .await
          .map_err(EventError::FetchFailed)?
          .json::<Event>()
          .await
          .map_err(EventError::FetchFailed)?,
      );
    }

    events.retain(|e| e.whitelist.contains(&uuid));

    let mut events_dtos: Vec<EventDTO> = Vec::new();

    for event in events {
      let disk_hashes =
        RemoteEventStore::build_disk_hashes(install_dir.clone(), event.clone()).await;
      let status = calculate_status(&event, disk_hashes);
      let event_dto = EventDTO { event, status };

      events_dtos.push(event_dto);
    }

    Ok(events_dtos)
  }
}
