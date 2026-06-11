use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LauncherConfig {
  pub java_path: Option<String>,
  pub install_dir: String,
  pub close_on_launch: bool,
}
