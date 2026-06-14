export type DownloadOutcome =
  | { kind: "success" }
  | { kind: "failure"; message: string }

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
