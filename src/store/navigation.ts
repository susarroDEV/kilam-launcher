import { create } from 'zustand'
import { EventDTO } from '../types/event_store'

export type Screen = 'main' | 'download' | 'settings'

interface NavigationStore {
  screen: Screen
  downloadTarget: EventDTO | null
  navigate: (screen: Screen) => void
  startDownload: (dto: EventDTO) => void
  finishDownload: () => void
}

export const useNavigation = create<NavigationStore>((set) => ({
  screen: 'main',
  downloadTarget: null,
  navigate: (screen) => set({ screen }),
  startDownload: (dto) => set({ screen: 'download', downloadTarget: dto }),
  finishDownload: () => set({ screen: 'main', downloadTarget: null }),
}))
