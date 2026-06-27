use crate::business::auth::UserProfile;
use crate::error::Result;
use crate::{MicrosoftAuth, OfflineAuth};

#[tauri::command]
pub async fn login(
  provider: tauri::State<'_, OfflineAuth>,
  username: String,
) -> Result<UserProfile> {
  provider.0.login(&username).await
}

#[tauri::command]
pub async fn login_microsoft(
  provider: tauri::State<'_, MicrosoftAuth>,
) -> Result<UserProfile> {
  provider.0.login("").await
}

#[tauri::command]
pub async fn logout(
  offline: tauri::State<'_, OfflineAuth>,
  microsoft: tauri::State<'_, MicrosoftAuth>,
) -> Result<()> {
  offline.0.logout().await?;
  microsoft.0.logout().await?;
  Ok(())
}

#[tauri::command]
pub async fn current_session(
  offline: tauri::State<'_, OfflineAuth>,
  microsoft: tauri::State<'_, MicrosoftAuth>,
) -> Result<Option<UserProfile>> {
  if let Some(p) = microsoft.0.current_session().await? {
    return Ok(Some(p));
  }
  offline.0.current_session().await
}
