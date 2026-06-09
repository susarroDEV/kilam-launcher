export type UserProfile = {
  username: string,
  uuid: string,
  token?: string, 
  auth_type: AuthType
}

export enum AuthType  {
  Offline = "offline",
  Microsoft = "microsoft"
}

export type AuthStore = {
  profile: UserProfile | null,
  setProfile: (p: UserProfile) => void,
  clearProfile: () => void
}
