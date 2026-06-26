use async_trait::async_trait;
use std::path::PathBuf;

use crate::business::auth::UserProfile;
use crate::business::config::LauncherConfig;
use crate::business::event_store::Event;
use crate::business::launcher::{LaunchError, Launcher};
use crate::error::Result;

pub struct ProcessLauncher;

impl ProcessLauncher {
  pub fn new() -> Self {
    ProcessLauncher
  }

  fn collect_libraries(dir: &PathBuf) -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = std::fs::read_dir(dir) {
      for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
          entries.extend(Self::collect_libraries(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("jar") {
          if let Some(s) = path.to_str() {
            entries.push(s.to_string());
          }
        }
      }
    }
    entries
  }
}

#[async_trait]
impl Launcher for ProcessLauncher {
  async fn launch(&self, event: &Event, user: &UserProfile, config: &LauncherConfig) -> Result<()> {
    let java = if let Some(path) = &config.java_path {
      path.clone()
    } else if let Ok(java_home) = std::env::var("JAVA_HOME") {
      format!("{}/bin/java", java_home)
    } else {
      "java".to_string()
    };

    if (config.java_path.is_some() || std::env::var("JAVA_HOME").is_ok())
      && !std::path::Path::new(&java).exists()
    {
      return Err(LaunchError::JavaNotFound.into());
    }

    let sep = if cfg!(target_os = "windows") {
      ";"
    } else {
      ":"
    };

    let instance_dir = format!("{}/{}", config.install_dir, event.id);
    let libraries_dir = PathBuf::from(format!("{}/libraries", instance_dir));
    let natives_dir = format!("{}/natives", instance_dir);
    let client_jar = format!("{}/client.jar", instance_dir);
    let game_dir = instance_dir.clone();

    let mut classpath_entries = Self::collect_libraries(&libraries_dir);
    classpath_entries.push(client_jar);
    let classpath = classpath_entries.join(sep);

    std::process::Command::new(&java)
      .arg(format!("-Djava.library.path={}", natives_dir))
      .arg("-cp")
      .arg(&classpath)
      .arg("net.minecraft.client.main.Main")
      .arg("--username")
      .arg(&user.username)
      .arg("--uuid")
      .arg(&user.uuid)
      .arg("--accessToken")
      .arg("0")
      .arg("--userType")
      .arg("legacy")
      .arg("--gameDir")
      .arg(&game_dir)
      .arg("--version")
      .arg(&event.minecraft_version)
      .arg(format!("-Xms{}m", config.min_memory_mb))
      .arg(format!("-Xmx{}m", config.max_memory_mb))
      .spawn()
      .map_err(|e| LaunchError::ProcessFailed(e.to_string()))?;

    Ok(())
  }
}
