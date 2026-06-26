export type LauncherConfig = {
  java_path?: string,
  install_dir: string,
  close_on_launch: boolean,
  min_memory?: number,
  max_memory?: number
}

export type ConfigStore = {
  config: LauncherConfig | null,
  setConfig: (newConfig: LauncherConfig) => void
}
