use crate::business::auth::{AuthError, AuthProvider, AuthType, UserProfile, validate_username};
use crate::error::Result;
use md5::{Md5, Digest};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

pub struct OfflineAuthProvider {
  app_handle: tauri::AppHandle
}

impl OfflineAuthProvider {
  pub fn new(app_handle: tauri::AppHandle) -> Self {
    Self {
      app_handle : app_handle
    }
  }
}

impl AuthProvider for OfflineAuthProvider {
  async fn login(&self, username: &str) -> Result<UserProfile> {
    let user = String::from(username);

    validate_username(username)?;

    let hash = Md5::digest(format!("OfflinePlayer:{}", user));
    let mut bytes: [u8; 16] = hash.into();

    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    let uuid = Uuid::from_bytes(bytes);
    
    let profile = UserProfile {
      username: user,
      uuid: String::from(uuid),
      auth_type: AuthType::Offline,
      token: None
    };

    let store = self.app_handle.store_builder("session.json").build()?;
    store.set("profile", serde_json::to_value(&profile)?);

    Ok(profile)
  }
  
  async fn logout(&self) -> Result<()> {
    let store = self.app_handle.store_builder("session.json").build()?;
    if let Some(_) = store.get("profile") {
      store.delete("profile");
    }
    Ok(())
  }

  async fn current_session(&self) -> Result<Option<UserProfile>> {
    let store = self.app_handle.store_builder("session.json").build()?;
    let profile = store.get("profile");

    match profile {
      Some(p) => {
        Ok(Some(serde_json::from_value(p)?))
      }
      None => {
        Ok(None)
      }
    }
  }
}
