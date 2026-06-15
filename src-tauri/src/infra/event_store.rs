use std::sync::Arc;
use crate::business::downloader::Downloader;
use crate::business::event_store::{
  calculate_status, Event, EventDTO, EventError, EventIndex, EventStore,
};
use crate::error::Result;
use async_trait::async_trait;
use reqwest::Client;
use tauri::{AppHandle, Manager};

const MANIFEST_URL: &str = "https://gist.githubusercontent.com/susarroDEV/c110ce866f2cfec390d03f117b2c54b6/raw/gistfile1.txt";

pub struct RemoteEventStore {
  client: Client,
  app_handle: tauri::AppHandle
}

impl RemoteEventStore {
  pub fn new(client: Client, app_handle: AppHandle) -> Self {
    Self { client, app_handle }
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

    let downloader = self.app_handle.state::<Arc<dyn Downloader + Send + Sync>>();

    for event in events {
      let status = calculate_status(&event, downloader.as_ref(), &install_dir).await;
      let event_dto = EventDTO { event, status };

      events_dtos.push(event_dto);
    }

    Ok(events_dtos)
  }
}
