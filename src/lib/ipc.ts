import { invoke } from "@tauri-apps/api/core"
import { LauncherConfig } from "../types/config"
import { UserProfile } from "../types/auth"
import { EventDTO } from "../types/event_store"

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

export async function getActiveEvents(uuid: string) : Promise<EventDTO[]>   {
  return invoke<EventDTO[]>("get_active_events", {uuid})
}
