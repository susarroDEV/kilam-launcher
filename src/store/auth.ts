import { create } from "zustand"
import { AuthStore } from "../types/auth"

export const useAuth = create<AuthStore>((set) => ({
  profile: null,
  setProfile : (p) => set({profile : p}),
  clearProfile : () => set({profile : null})
}))
