import { useEffect, useState } from 'react'
import '../styles/shared.css'
import '../styles/pages/download.css'
import { useNavigation } from '../store/navigation'
import { useConfig } from '../store/config'
import {
  downloadEvent,
  onDownloadProgress,
  onDownloadComplete,
  onProvisionProgress,
  ProvisionProgress,
} from '../lib/ipc'
import { DownloadProgress } from '../types/downloader'
import Button from '../components/ui/Button'
import ProgressBar from '../components/ui/ProgressBar'

function formatBytes(bytes: number): string {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

interface ErrorModalProps {
  eventName: string
  message: string
  onRetry: () => void
  onCancel: () => void
}

function ErrorModal({ eventName, message, onRetry, onCancel }: ErrorModalProps) {
  return (
    <div className="error-modal-overlay" role="dialog" aria-modal="true">
      <article className="error-modal">
        <header>
          <p className="error-modal__eyebrow">× ERROR</p>
          <h2 className="error-modal__title">{eventName}</h2>
        </header>

        <p className="error-modal__message">{message}</p>

        <div className="error-modal__actions">
          <Button variant="secondary" size="sm" onClick={onCancel}>Cancelar</Button>
          <Button variant="primary"   size="sm" onClick={onRetry}>↺ Reintentar</Button>
        </div>
      </article>
    </div>
  )
}

export default function DownloadScreen() {
  const dto            = useNavigation((s) => s.downloadTarget)
  const finishDownload = useNavigation((s) => s.finishDownload)
  const config         = useConfig((s) => s.config)

  const [progress, setProgress]             = useState<DownloadProgress | null>(null)
  const [provisionMsg, setProvisionMsg]     = useState('')
  const [provisionPct, setProvisionPct]     = useState(0)
  const [failed, setFailed]                 = useState(false)
  const [failureMessage, setFailureMessage] = useState('')
  const [retryCount, setRetryCount]         = useState(0)

  const event      = dto?.event
  const installDir = config?.install_dir

  const pct = progress
    ? Math.round((progress.downloaded_bytes / Math.max(1, progress.total_bytes)) * 100)
    : provisionPct

  const handleRetry = () => {
    setFailed(false)
    setFailureMessage('')
    setProgress(null)
    setProvisionMsg('')
    setProvisionPct(0)
    setRetryCount((c) => c + 1)
  }

  useEffect(() => {
    if (!event || !installDir) return

    let cancelled = false
    const unlisteners: Array<() => void> = []

    const cleanup = () => {
      cancelled = true
      unlisteners.forEach((u) => u())
      unlisteners.length = 0
    }

    const run = async () => {
      const up = await onDownloadProgress((p) => {
        if (!cancelled && p.event_id === event.id) setProgress(p)
      })
      unlisteners.push(up)

      const uc = await onDownloadComplete((result) => {
        if (cancelled) return
        if (result.outcome === 'success') {
          finishDownload()
        } else {
          const msg =
            typeof result.outcome === 'object' && 'failure' in result.outcome
              ? (result.outcome as { failure: string }).failure
              : 'Error desconocido'
          setFailureMessage(msg)
          setFailed(true)
        }
        cleanup()
      })
      unlisteners.push(uc)

      const uprov = await onProvisionProgress((p: ProvisionProgress) => {
        if (cancelled) return
        setProvisionPct(p.percentage)
        setProvisionMsg(p.message)
      })
      unlisteners.push(uprov)

      try {
        await downloadEvent(event, installDir)
      } catch (e) {
        if (!cancelled) {
          setFailureMessage(String(e))
          setFailed(true)
        }
        cleanup()
      }
    }

    run()
    return cleanup
  }, [retryCount, event, installDir, finishDownload])

  if (!event) return null

  const totalMB        = progress ? formatBytes(progress.total_bytes)     : '—'
  const doneBytes      = progress ? formatBytes(progress.downloaded_bytes) : '—'
  const modloaderLabel = event.modloader !== 'vanilla'
    ? `${event.modloader} ${event.modloader_version}`
    : null

  return (
    <div className="download-screen">
      <header className="app-header">
        <div className="app-header__brand">
          <div className="app-header__icon">
            <img
              src="/logos/kilam-rounded.avif"
              alt="KIL/AM"
              width="26"
              height="26"
              onError={(e) => {
                e.currentTarget.style.display = 'none'
                const p = e.currentTarget.parentElement
                if (p) p.innerHTML = `<span style="color:var(--color-secondary);font-weight:900;font-size:8px;line-height:1.1;text-align:center">KIL<br/>AM</span>`
              }}
            />
          </div>
          <span className="app-header__name">LAUNCHER</span>
        </div>
      </header>

      <main className="download-content">
        <div aria-hidden="true" className="download__grid" />

        <div className="download-layout">
          <div className="download-left">
            <p className="download-eyebrow">▶ PREPARANDO EVENTO</p>
            <h1 className="download-title">{event.name}</h1>
            <p className="download-subtitle">Provisionando cliente de Minecraft</p>

            <div className="download-progress">
              <ProgressBar value={pct} />
              <p className="download-meta">
                {[modloaderLabel, `${doneBytes} / ${totalMB}`].filter(Boolean).join(' · ')}
              </p>
            </div>

            <div className="provision-steps">
              <div className="provision-step provision-step--current">
                <span className="provision-step__icon provision-step__icon--current spinner" aria-hidden="true" />
                <span className="provision-step__label">
                  {provisionMsg || 'Iniciando descarga...'}
                </span>
              </div>
            </div>
          </div>

          <div className="download-right" aria-hidden="true">
            <span className="download-pct">
              {pct}<span className="download-pct__sym">%</span>
            </span>
          </div>
        </div>

        <div className="download-footer">
          <Button variant="ghost" size="sm" onClick={finishDownload}>
            Cancelar
          </Button>
        </div>
      </main>

      {failed && (
        <ErrorModal
          eventName={event.name}
          message={failureMessage || 'La verificación SHA256 no coincide. El archivo descargado está corrupto o cambió en el servidor.'}
          onRetry={handleRetry}
          onCancel={finishDownload}
        />
      )}
    </div>
  )
}
