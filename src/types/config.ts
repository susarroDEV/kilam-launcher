export type LauncherConfig = {
  java_path?: string,
  install_dir: string,
  close_on_launch: boolean
}

export type ConfigStore = {
  config: LauncherConfig | null,
  setConfig: (newConfig: LauncherConfig) => void
}
