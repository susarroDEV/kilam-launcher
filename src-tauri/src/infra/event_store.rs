use std::collections::HashMap;
use std::fs::read;

use md5::Digest;
use reqwest::{Client};
use sha2::Sha256;
use crate::error::Result;
use crate::business::event_store::{Event, EventDTO, EventError, EventIndex, EventStore, calculate_status};

const MANIFEST_URL: &str = "https://gist.githubusercontent.com/susarroDEV/c110ce866f2cfec390d03f117b2c54b6/raw/d80d4d36558e6311c77299e4d625bd637d4d3487/gistfile1.txt";

pub struct RemoteEventStore {
  client: Client,
  install_dir: String
}

impl RemoteEventStore {
  pub fn new(client: Client, install_dir: String) -> Self {
    Self {
      client: client,
      install_dir: install_dir
    }
  }

  fn build_disk_hashes(&self, event: &Event) -> HashMap<String, String> {
    let mut assets: HashMap<String, String> = HashMap::new();

    for asset in &event.assets {
      let complete_path = format!("{}/{}", self.install_dir, asset.path);
      match read(complete_path) {
        Err(_) => {continue;}
        Ok(bytes) => {
          let hash = Sha256::digest(&bytes);
          let hex_string = hex::encode(hash);
          
          assets.insert(asset.id.clone(), hex_string);
        }
      }
    }

    assets
  }

}

impl EventStore for RemoteEventStore {
  async fn get_active_events(&self, uuid: String) -> Result<Vec<EventDTO>> {
    let index = self.client
    .get(MANIFEST_URL)
    .send()
    .await
    .map_err(EventError::FetchFailed)?
    .json::<EventIndex>()
    .await
    .map_err(EventError::FetchFailed)?;

    let mut events : Vec<Event> = Vec::new();

    for url in &index.active_events {
      events.push(
        self.client
          .get(url)
          .send()
          .await
          .map_err(EventError::FetchFailed)?
          .json::<Event>()
          .await
          .map_err(EventError::FetchFailed)?
      );
    }

    events.retain(
      |e| e.whitelist.contains(&uuid)
    );

    let mut events_dtos : Vec<EventDTO> = Vec::new();

    for event in events  {
      let disk_hashes = self.build_disk_hashes(&event);
      let status = calculate_status(&event, disk_hashes);
      let event_dto = EventDTO {
        event: event,
        status: status
      };

      events_dtos.push(event_dto);
    }

    Ok(events_dtos)
  }
}
