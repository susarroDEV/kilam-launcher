export type DownloadOutcome = "success" | { failure: string }

export enum DownloadStatus {
  Pending = "pending",
  Downloading = "downloading",
  Done = "done",
  Failed = "failed"
}

export type DownloadProgress = {
  event_id: string,
  asset_id: string,
  downloaded_bytes: number,
  total_bytes: number,
  status: DownloadStatus
}

export type DownloadResult = {
  event_id: string,
  outcome: DownloadOutcome
}
