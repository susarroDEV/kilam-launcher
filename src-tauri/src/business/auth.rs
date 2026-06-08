use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
  Offline,
  Microsoft
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
  pub username: String,
  pub uuid: String,
  pub token: Option<String>,
  pub auth_type: AuthType
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
  MissingSession
}
