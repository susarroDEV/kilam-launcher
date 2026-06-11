use tauri::Manager;
use tauri_plugin_store::StoreExt;
use crate::business::config::LauncherConfig;
use crate::error::Result;

pub fn read_config (app_handle: &tauri::AppHandle) -> Result<LauncherConfig> {
  let store = app_handle.store_builder("config.json").build()?;
  let config = store.get("config");

  match config {
    Some(config) => {
      let res = serde_json::from_value(config)?;

      Ok(res)
    },
    None => {
      let default_config = LauncherConfig {
        java_path: None,
        install_dir: String::from(app_handle.path().app_data_dir()?.to_string_lossy().to_string()),
        close_on_launch: true
      };

      Ok(default_config)
    }
  }
}
