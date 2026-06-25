import { invoke } from "@tauri-apps/api/core"
import { LauncherConfig } from "../types/config"
import { UserProfile } from "../types/auth"
import { EventDTO, Event } from "../types/event_store"
import { DownloadProgress, DownloadResult } from "../types/downloader"
import { listen } from "@tauri-apps/api/event"

// CONFIG

export async function getConfig() : Promise<LauncherConfig> {
  return invoke<LauncherConfig>("get_config")
}

// AUTH

export async function login(username: string) : Promise<UserProfile> {
  return invoke<UserProfile>("login", { username })
}

export async function logout() {
  return invoke("logout")
}

export async function getSession() : Promise<UserProfile | null> {
  return invoke<UserProfile>("current_session")
}

// EVENT STORE

export async function getActiveEvents(uuid: string, install_dir: string) : Promise<EventDTO[]>   {
  return invoke<EventDTO[]>("get_active_events", {uuid, installDir: install_dir})
}

// DOWNLOADER

export async function downloadEvent(event: Event, install_dir: string) : Promise<void> {
  return invoke<void>("download_event", {event, installDir: install_dir})
}

export async function onDownloadProgress(
  callback: (progress: DownloadProgress) => void
) {
  return listen<DownloadProgress>("download:progress", (event) => {
    callback(event.payload)
  })
}

export async function onDownloadComplete(
  callback: (result: DownloadResult) => void
) {
  return listen<DownloadResult>("download:complete", (event) => {
    callback(event.payload)
  })
}

// PROVISIONER

export interface ProvisionProgress {
  percentage: number
  message: string
}

export async function onProvisionProgress(
  callback: (progress: ProvisionProgress) => void
) {
  return listen<ProvisionProgress>("provision:progress", (event) => {
    callback(event.payload)
  })
}

// LAUNCHER

export async function launchEvent(eventId: string) : Promise<void> {
  return invoke<void>("launch_event", { eventId })
}
