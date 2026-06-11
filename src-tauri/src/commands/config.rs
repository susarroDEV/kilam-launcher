use crate::business::config::LauncherConfig;
use crate::error::Result;
use crate::infra::config::read_config;

#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle) -> Result<LauncherConfig> {
  let config = read_config(&app_handle)?;
  Ok(config)
}
