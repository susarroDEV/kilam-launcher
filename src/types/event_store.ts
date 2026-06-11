export enum EventStatus {
  NotInstalled = "notinstalled",
  Outdated = "outdated",
  Ready = "ready"
}

export enum ModLoader {
  Fabric = "fabric",
  Forge = "forge",
  Vanilla = "vanilla"
}

export type Asset = {
  id: string,
  url: string,
  sha256: string,
  path: string
}

export type Event = {
  id: string,
  name: string,
  description: string,
  image_url: string,
  minecraft_version: string,
  modloader: ModLoader,
  modloader_version: string,
  server_ip: string,
  whitelist: string[],
  assets: Asset[]
}

export type EventDTO = {
  event: Event,
  status: EventStatus
}
