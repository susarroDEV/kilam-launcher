use std::sync::Arc;

use crate::business::event_store::{EventDTO, EventStore};
use crate::error::Result;

#[tauri::command]
pub async fn get_active_events(
  event_store: tauri::State<'_, Arc<dyn EventStore + Send + Sync>>,
  uuid: String,
  install_dir: String,
) -> Result<Vec<EventDTO>> {
  let event_dtos = event_store.get_active_events(uuid, install_dir).await?;

  Ok(event_dtos)
}
