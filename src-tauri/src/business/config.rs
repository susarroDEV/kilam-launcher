use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LauncherConfig {
  pub java_path: Option<String>,
  pub install_dir: String,
  pub close_on_launch: bool,
  pub min_memory_mb: u32,
  pub max_memory_mb: u32
}

#[async_trait]
pub trait ConfigStore {
  async fn read_config(&self) -> Result<LauncherConfig>;
  async fn update_config(&self, new_config: LauncherConfig) -> Result<()>;
}
