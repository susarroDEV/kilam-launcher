import { invoke } from "@tauri-apps/api/core"
import { LauncherConfig } from "../types/config"

export async function getConfig() : Promise<LauncherConfig> {
  return invoke<LauncherConfig>("get_config")
}
