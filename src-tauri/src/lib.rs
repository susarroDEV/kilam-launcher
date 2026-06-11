use reqwest::Client;
use tauri::Manager;

pub use error::{LauncherError, Result};
mod business;
mod commands;
mod error;
mod infra;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(|app| {
      app.manage(Client::new());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      crate::commands::config::get_config,
      crate::commands::auth::login,
      crate::commands::auth::logout,
      crate::commands::auth::current_session,
      crate::commands::event_store::get_active_events
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
