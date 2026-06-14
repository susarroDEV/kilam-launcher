use std::sync::Arc;

use crate::business::auth::{AuthProvider, UserProfile};
use crate::error::Result;

#[tauri::command]
pub async fn login(
  provider: tauri::State<'_, Arc<dyn AuthProvider + Send + Sync>>,
  username: String,
) -> Result<UserProfile> {
  let profile = provider.login(&username).await?;

  Ok(profile)
}

#[tauri::command]
pub async fn logout(provider: tauri::State<'_, Arc<dyn AuthProvider + Send + Sync>>) -> Result<()> {
  provider.logout().await?;

  Ok(())
}

#[tauri::command]
pub async fn current_session(
  provider: tauri::State<'_, Arc<dyn AuthProvider + Send + Sync>>,
) -> Result<Option<UserProfile>> {
  let profile = provider.current_session().await?;

  Ok(profile)
}
