use tauri::Manager;
use crate::business::event_store::{EventDTO, EventStore};
use crate::infra::config::read_config;
use crate::infra::event_store::RemoteEventStore;
use reqwest::Client;
use crate::error::Result;

#[tauri::command]
pub async fn get_active_events(app_handle: tauri::AppHandle, uuid: String) -> Result<Vec<EventDTO>> {
  let config = read_config(&app_handle)?;
  let client = app_handle.state::<Client>().inner().clone();

  let remote_event_store = RemoteEventStore::new(client, config.install_dir);

  let event_dtos = remote_event_store.get_active_events(uuid).await?; 

  Ok(event_dtos)
}
