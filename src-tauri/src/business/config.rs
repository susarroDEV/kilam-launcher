use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LauncherConfig {
  pub java_path: Option<String>,
  pub install_dir: String,
  pub close_on_launch: bool,
}

#[async_trait]
pub trait ConfigStore {
  async fn read_config(&self) -> Result<LauncherConfig>;
  async fn update_config(&self, new_config: LauncherConfig) -> Result<()>;
}
