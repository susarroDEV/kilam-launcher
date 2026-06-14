use std::sync::Arc;

use crate::business::downloader::Downloader;
use crate::business::event_store::Event;

use crate::error::Result;

#[tauri::command]
pub async fn download_event(
  downloader: tauri::State<'_, Arc<dyn Downloader + Send + Sync>>,
  event: Event,
  install_dir: String,
) -> Result<()> {
  let downloader = Arc::clone(&downloader);

  tauri::async_runtime::spawn(async move {
    let _ = downloader.download_event(event, install_dir).await;
  });

  Ok(())
}
