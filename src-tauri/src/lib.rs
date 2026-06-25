use crate::business::auth::AuthProvider;
use crate::business::client_provisioner::ClientProvisioner;
use crate::business::config::ConfigStore;
use crate::business::downloader::Downloader;
use crate::business::event_store::EventStore;
use crate::infra::auth::OfflineAuthProvider;
use crate::infra::client_provisioner::MojangClientProvisioner;
use crate::infra::config::LocalConfigStore;
use crate::infra::downloader::HttpDownloader;
use crate::infra::event_store::RemoteEventStore;
use reqwest::Client;
use std::sync::Arc;
use tauri::Manager;

pub use error::LauncherError;

mod business;
mod commands;
mod error;
mod infra;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tracing_subscriber::fmt::init();

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(setup)
    .invoke_handler(tauri::generate_handler![
      crate::commands::config::get_config,
      crate::commands::config::update_config,
      crate::commands::auth::login,
      crate::commands::auth::logout,
      crate::commands::auth::current_session,
      crate::commands::event_store::get_active_events,
      crate::commands::downloader::download_event
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
  let auth_provider: Arc<dyn AuthProvider + Send + Sync> =
    Arc::new(OfflineAuthProvider::new(app.handle().clone()));

  app.manage(auth_provider);

  let client = Client::new();
  let event_store: Arc<dyn EventStore + Send + Sync> =
    Arc::new(RemoteEventStore::new(client.clone(), app.handle().clone()));

  app.manage(event_store);

  let config_store: Arc<dyn ConfigStore + Send + Sync> =
    Arc::new(LocalConfigStore::new(app.handle().clone()));

  app.manage(config_store);

  let downloader: Arc<dyn Downloader + Send + Sync> =
    Arc::new(HttpDownloader::new(app.handle().clone(), client.clone()));

  app.manage(downloader);

  let provisioner: Arc<dyn ClientProvisioner + Send + Sync> =
    Arc::new(MojangClientProvisioner::new(client.clone()));

  app.manage(provisioner);

  app.manage(client);

  Ok(())
}
