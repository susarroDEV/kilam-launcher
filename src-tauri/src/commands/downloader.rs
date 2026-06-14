use std::sync::Arc;

use crate::business::event_store::Event;
use crate::business::downloader::{DownloadResult, Downloader};

use crate::error::Result;

#[tauri::command]
pub async fn download_event (
  downloader: tauri::State<'_, Arc<dyn Downloader + Send + Sync>>,
  event: Event,
  install_dir: String
  ) -> Result<DownloadResult> {

  let download_result = downloader.download_event(event, install_dir).await?;

  Ok(download_result)
}
