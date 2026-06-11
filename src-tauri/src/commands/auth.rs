use crate::business::auth::{AuthProvider, UserProfile};
use crate::error::Result;
use crate::infra::auth::OfflineAuthProvider;

#[tauri::command]
pub async fn login(app_handle: tauri::AppHandle, username: String) -> Result<UserProfile> {
  let provider = OfflineAuthProvider::new(app_handle);

  let profile = provider.login(&username).await?;

  Ok(profile)
}

#[tauri::command]
pub async fn logout(app_handle: tauri::AppHandle) -> Result<()> {
  let provider = OfflineAuthProvider::new(app_handle);

  provider.logout().await?;

  Ok(())
}

#[tauri::command]
pub async fn current_session(app_handle: tauri::AppHandle) -> Result<Option<UserProfile>> {
  let provider = OfflineAuthProvider::new(app_handle);

  let profile = provider.current_session().await?;

  Ok(profile)
}
