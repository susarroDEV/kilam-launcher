use crate::business::config::{ConfigStore, LauncherConfig};
use crate::error::Result;
use async_trait::async_trait;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

pub struct LocalConfigStore {
  app_handle: tauri::AppHandle,
}

impl LocalConfigStore {
  pub fn new(app_handle: tauri::AppHandle) -> Self {
    Self { app_handle }
  }
}

#[async_trait]
impl ConfigStore for LocalConfigStore {
  async fn read_config(&self) -> Result<LauncherConfig> {
    let store = self.app_handle.store_builder("config.json").build()?;
    let config = store.get("config");

    match config {
      Some(config) => {
        let res = serde_json::from_value(config)?;

        Ok(res)
      }
      None => {
        let default_config = LauncherConfig {
          java_path: None,
          install_dir: self
            .app_handle
            .path()
            .app_data_dir()?
            .to_string_lossy()
            .to_string(),
          close_on_launch: true,
          min_memory_mb: 512,
          max_memory_mb: 2048,
        };

        Ok(default_config)
      }
    }
  }

  async fn update_config(&self, new_config: LauncherConfig) -> Result<()> {
    let store = self.app_handle.store_builder("config.json").build()?;
    store.set("config", serde_json::to_value(&new_config)?);
    Ok(())
  }
}
