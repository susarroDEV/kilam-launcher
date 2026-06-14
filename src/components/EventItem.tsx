import { useConfig } from "../store/config"
import { EventDTO, EventStatus } from "../types/event_store"
import { DownloadProgress } from "../types/downloader"
import { downloadEvent, onDownloadComplete, onDownloadProgress } from "../lib/ipc"
import { useEffect, useState } from "react"

function EventItem({dto}: {dto: EventDTO}) {
  const [status, setStatus] = useState(dto.status)
  const [downloading, setDownloading] = useState(false)
  const [error, setError] = useState(false)
  const [progress, setProgress] = useState<DownloadProgress | null>(null)

  const config = useConfig((state) => state.config)
  
  const handleDownload = async () => {
    if (!config) return
    
    setDownloading(true)
    try {
      await downloadEvent(dto.event, config.install_dir)
    } catch (e) {console.error(e)
      setDownloading(false)
      setError(true)
    }
    
  }

  const eventId = dto.event.id

  useEffect(() => {
    let unlistenProgress: (() => void) | null = null
    let unlistenComplete: (() => void) | null = null

    const setup = async () => {
      unlistenProgress = await onDownloadProgress((progress) => {
        if (progress.event_id == eventId) setProgress(progress)
      })
      unlistenComplete = await onDownloadComplete((result) => {
        console.log("download complete", result)
        if (result.outcome === "success") {
          console.log("setting status to ready")
          setStatus(EventStatus.Ready)
        }
        setDownloading(false)
      })
    }

    setup()

    return () => {
      unlistenProgress?.()
      unlistenComplete?.()
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
      {(
        status == EventStatus.Ready 
        ?
        <button>
          Play
        </button>
        :
        <button
          disabled={downloading}
          onClick={handleDownload}
        >
          Download
        </button>
      )}
      {(
        progress && 
        status != EventStatus.Ready &&
        <progress value={progress.downloaded_bytes} max={progress.total_bytes} />
      )}
      {(
        error &&
        <p>
          Ha habido un error...
        </p>
      )}
    </li>
  )
}

export default EventItem
