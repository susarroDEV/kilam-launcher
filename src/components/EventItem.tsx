import { useConfig } from "../store/config"
import { EventDTO, EventStatus } from "../types/event_store"
import { DownloadProgress } from "../types/downloader"
import { downloadEvent, launchEvent, onDownloadComplete, onDownloadProgress, onProvisionProgress, ProvisionProgress } from "../lib/ipc"
import { useEffect, useState } from "react"
import { getCurrentWindow } from "@tauri-apps/api/window"

function EventItem({dto}: {dto: EventDTO}) {
  const [status, setStatus] = useState(dto.status)
  const [downloading, setDownloading] = useState(false)
  const [launching, setLaunching] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [progress, setProgress] = useState<DownloadProgress | null>(null)
  const [provisionProgress, setProvisionProgress] = useState<ProvisionProgress | null>(null)

  const config = useConfig((state) => state.config)
  
  const handleDownload = async () => {
    if (!config) return
    setDownloading(true)
    setError(null)
    try {
      await downloadEvent(dto.event, config.install_dir)
    } catch (e) {
      console.error(e)
      setDownloading(false)
      setError("Error al descargar el evento")
    }
  }

  const handleLaunch = async () => {
    setLaunching(true)
    setError(null)
    setProvisionProgress(null)
    try {
      await launchEvent(dto.event.id)
      if (config?.close_on_launch) {
        await getCurrentWindow().close()
      }
    } catch (e) {
      console.error(e)
      setError("Error al lanzar el juego")
    } finally {
      setLaunching(false)
      setProvisionProgress(null)
    }
  }

  const eventId = dto.event.id

  useEffect(() => {
    let unlistenProgress: (() => void) | null = null
    let unlistenComplete: (() => void) | null = null
    let unlistenProvision: (() => void) | null = null

    const setup = async () => {
      unlistenProgress = await onDownloadProgress((progress) => {
        if (progress.event_id == eventId) setProgress(progress)
      })
      unlistenComplete = await onDownloadComplete((result) => {
        if (result.outcome === "success") {
          setStatus(EventStatus.Ready)
        }
        setDownloading(false)
      })
      unlistenProvision = await onProvisionProgress((progress) => {
        setProvisionProgress(progress)
      })
    }

    setup()

    return () => {
      unlistenProgress?.()
      unlistenComplete?.()
      unlistenProvision?.()
    }
  }, [eventId])

  return (
    <li>
      <header>
        {status}
      </header>
      <h1>
        {dto.event.name}
      </h1>
      <p>
        {dto.event.description}
      </p>
      {status === EventStatus.Ready ? (
        <button
          disabled={launching}
          onClick={handleLaunch}
        >
          {launching ? "Preparando..." : "Play"}
        </button>
      ) : (
        <button
          disabled={downloading}
          onClick={handleDownload}
        >
          {downloading ? "Descargando..." : "Download"}
        </button>
      )}
      {progress && status !== EventStatus.Ready && (
        <progress value={progress.downloaded_bytes} max={progress.total_bytes} />
      )}
      {launching && provisionProgress && (
        <div>
          <progress value={provisionProgress.percentage} max={100} />
          <p>{provisionProgress.message}</p>
        </div>
      )}
      {error && (
        <p>{error}</p>
      )}
    </li>
  )
}

export default EventItem
