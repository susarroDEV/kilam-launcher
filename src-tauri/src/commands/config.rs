use std::sync::Arc;

use crate::business::config::{ConfigStore, LauncherConfig};
use crate::error::Result;

#[tauri::command]
pub async fn get_config(
    config_store: tauri::State<'_, Arc<dyn ConfigStore + Send + Sync>>
  ) -> Result<LauncherConfig> {
  let config = config_store.read_config().await?;
  Ok(config)
}
