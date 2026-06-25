use async_trait::async_trait;
use thiserror::Error;

use crate::business::auth::UserProfile;
use crate::business::config::LauncherConfig;
use crate::business::event_store::Event;
use crate::error::Result;

#[derive(Error, Debug)]
pub enum LaunchError {
  #[error("Not ready")]
  NotReady,
  #[error("Java has not been found")]
  JavaNotFound,
  #[error("The process has failed: {0}")]
  ProcessFailed(String),
}

#[async_trait]
pub trait Launcher: Send + Sync {
  async fn launch(&self, event: &Event, user: &UserProfile, config: &LauncherConfig) -> Result<()>;
}
