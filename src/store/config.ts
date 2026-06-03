import { create } from "zustand"
import { ConfigStore } from "../types/config"

export const useConfig = create<ConfigStore>((set) => ({
  config: null,
  setConfig : (newConfig) => set({config : newConfig})
}))
