use crate::error::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
  Offline,
  Microsoft,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
  pub username: String,
  pub uuid: String,
  pub token: Option<String>,
  pub auth_type: AuthType,
}

pub trait AuthProvider {
  async fn login(&self, username: &str) -> Result<UserProfile>;
  async fn logout(&self) -> Result<()>;
  async fn current_session(&self) -> Result<Option<UserProfile>>;
}

#[derive(Error, Debug)]
pub enum AuthError {
  #[error("Username: {0} is not a valid input")]
  InvalidUsername(String),
  #[error("There is no active session")]
  MissingSession,
}

pub fn validate_username(username: &str) -> Result<()> {
  let username = username.trim();

  if username.len() < 3 || username.len() > 16 {
    return Err(AuthError::InvalidUsername(username.to_string()).into());
  }

  if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
    return Err(AuthError::InvalidUsername(username.to_string()).into());
  }

  Ok(())
}
