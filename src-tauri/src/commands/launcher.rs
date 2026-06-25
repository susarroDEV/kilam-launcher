use std::sync::Arc;

use tauri::State;

use crate::business::auth::AuthProvider;
use crate::business::config::ConfigStore;
use crate::business::client_provisioner::ClientProvisioner;
use crate::business::event_store::EventStore;
use crate::business::launcher::{LaunchError, Launcher};
use crate::error::Result;

#[tauri::command]
pub async fn launch_event(
  event_id: String,
  launcher: State<'_, Arc<dyn Launcher + Send + Sync>>,
  auth: State<'_, Arc<dyn AuthProvider + Send + Sync>>,
  event_store: State<'_, Arc<dyn EventStore + Send + Sync>>,
  config_store: State<'_, Arc<dyn ConfigStore + Send + Sync>>,
  client_provisioner: State<'_, Arc<dyn ClientProvisioner + Send + Sync>>,
) -> Result<()> {
  let config = config_store.read_config().await?;
  
  let user = auth.current_session().await?
    .ok_or(LaunchError::NotReady)?;

  let events = event_store.get_active_events(user.uuid.clone(), config.install_dir.clone()).await?;
  let event = events
    .into_iter()
    .find(|e| e.event.id == event_id)
    .ok_or(LaunchError::NotReady)?;

  if !matches!(event.status, crate::business::event_store::EventStatus::Ready) {
    return Err(LaunchError::NotReady.into());
  }

  client_provisioner.provision(&event.event, config.install_dir.clone()).await?;

  launcher.launch(&event.event, &user, &config).await?;

  Ok(())
}
