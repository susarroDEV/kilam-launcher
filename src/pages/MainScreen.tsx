import { useEffect, useState } from 'react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import '../styles/shared.css'
import '../styles/pages/main.css'
import { useAuth } from '../store/auth'
import { useConfig } from '../store/config'
import { useNavigation } from '../store/navigation'
import { getActiveEvents, launchEvent, onProvisionProgress, ProvisionProgress } from '../lib/ipc'
import { EventDTO, EventStatus } from '../types/event_store'
import Button from '../components/ui/Button'
import EventCard from '../components/ui/EventCard'
import ProgressBar from '../components/ui/ProgressBar'

function AppHeader({ username }: { username: string }) {
  const navigate = useNavigation((s) => s.navigate)
  return (
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

      <nav className="app-header__nav">
        <Button variant="ghost" size="sm" onClick={() => navigate('settings')} aria-label="Ajustes">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
        </Button>
        <div className="app-header__user">
          <div className="app-header__avatar" aria-hidden="true">
            {username[0]?.toUpperCase()}
          </div>
          <span className="app-header__username">{username}</span>
        </div>
      </nav>
    </header>
  )
}

function MetaChip({ icon, label }: { icon: string; label: string }) {
  return (
    <span className="meta-chip">
      <span aria-hidden="true">{icon}</span>
      <span>{label}</span>
    </span>
  )
}

interface FeaturedProps {
  dto: EventDTO
  onDownload: (dto: EventDTO) => void
  onLaunch: (eventId: string) => void
  launching: boolean
  provisionMsg: string
  provisionPct: number
}

function FeaturedEvent({ dto, onDownload, onLaunch, launching, provisionMsg, provisionPct }: FeaturedProps) {
  const { event, status } = dto
  const modloaderLabel = event.modloader !== 'vanilla'
    ? `${event.modloader} ${event.modloader_version}`
    : null

  return (
    <article className="featured-event">
      <div className="featured-event__grid" aria-hidden="true" />

      <div className="featured-event__body">
        
        <h1 className="featured-event__title">{event.name}</h1>

        <p className="featured-event__description">{event.description}</p>

        <div className="featured-event__meta">
          <MetaChip icon="⛏" label={`Minecraft ${event.minecraft_version}`} />
          {modloaderLabel && <MetaChip icon="🔧" label={modloaderLabel} />}
        </div>

        {status === EventStatus.Ready && (
          <div className="featured-event__actions">
            <div className="featured-event__play-row">
              <Button
                variant="primary"
                size="lg"
                onClick={() => onLaunch(event.id)}
                loading={launching}
                disabled={launching}
              >
                ▶ JUGAR
              </Button>
              {!launching && <span className="ready-badge">● Listo</span>}
            </div>

            {launching && (
              <div className="provision-status">
                <ProgressBar value={provisionPct} />
                {provisionMsg && (
                  <div className="provision-status__line">
                    <span className="spinner" aria-hidden="true" />
                    <span className="provision-status__msg">{provisionMsg}</span>
                  </div>
                )}
              </div>
            )}
          </div>
        )}

        {status === EventStatus.NotInstalled && (
          <Button variant="primary" size="lg" onClick={() => onDownload(dto)}>
            ⬇ DESCARGAR
          </Button>
        )}

        {status === EventStatus.Outdated && (
          <Button
            variant="primary"
            size="lg"
            onClick={() => onDownload(dto)}
            className="btn--update"
          >
            ↑ ACTUALIZAR
          </Button>
        )}
      </div>
    </article>
  )
}

interface EventsBarProps {
  events: EventDTO[]
  selectedId: string
  onSelect: (id: string) => void
}

function EventsBar({ events, selectedId, onSelect }: EventsBarProps) {
  const others = events.filter((d) => d.event.id !== selectedId)
  if (others.length === 0) return null

  return (
    <aside className="events-bar">
      <h2 className="events-bar__label">Otros eventos</h2>
      <div className="events-bar__list">
        {others.map((dto) => (
          <EventCard
            key={dto.event.id}
            event={dto.event}
            status={dto.status}
            onClick={() => onSelect(dto.event.id)}
            active={false}
          />
        ))}
      </div>
    </aside>
  )
}

export default function MainScreen() {
  const profile       = useAuth((s) => s.profile)
  const config        = useConfig((s) => s.config)
  const startDownload = useNavigation((s) => s.startDownload)

  const [events, setEvents]             = useState<EventDTO[]>([])
  const [loading, setLoading]           = useState(true)
  const [fetchError, setFetchError]     = useState<string | null>(null)
  const [selectedId, setSelectedId]     = useState<string | null>(null)
  const [launching, setLaunching]       = useState(false)
  const [provisionMsg, setProvisionMsg] = useState('')
  const [provisionPct, setProvisionPct] = useState(0)

  useEffect(() => {
    if (!profile || !config) return
    let cancelled = false
    const load = async () => {
      setLoading(true)
      try {
        const data = await getActiveEvents(profile.uuid, config.install_dir)
        if (!cancelled) {
          setEvents(data)
          if (data.length > 0) setSelectedId(data[0].event.id)
        }
      } catch (e) {
        if (!cancelled) setFetchError(String(e))
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    load()
    return () => { cancelled = true }
  }, [profile, config])

  const handleLaunch = async (eventId: string) => {
    setLaunching(true)
    setProvisionMsg('')
    setProvisionPct(0)

    const unlisten = await onProvisionProgress((p: ProvisionProgress) => {
      setProvisionPct(p.percentage)
      setProvisionMsg(p.message)
    })

    try {
      await launchEvent(eventId)
      if (config?.close_on_launch) await getCurrentWindow().close()
    } catch (e) {
      console.error('Launch error:', e)
    } finally {
      setLaunching(false)
      setProvisionMsg('')
      setProvisionPct(0)
      unlisten()
    }
  }

  const featuredDto = events.find((d) => d.event.id === selectedId) ?? events[0] ?? null

  return (
    <div className="main-screen">
      <AppHeader username={profile?.username ?? ''} />

      <main className="main-content">
        {loading && (
          <div className="status-center status-center--muted">Cargando eventos...</div>
        )}

        {!loading && fetchError && (
          <div className="status-center status-center--error">{fetchError}</div>
        )}

        {!loading && !fetchError && events.length === 0 && (
          <div className="status-center status-center--muted">No hay eventos disponibles.</div>
        )}

        {!loading && !fetchError && featuredDto && (
          <FeaturedEvent
            dto={featuredDto}
            onDownload={startDownload}
            onLaunch={handleLaunch}
            launching={launching}
            provisionMsg={provisionMsg}
            provisionPct={provisionPct}
          />
        )}
      </main>

      {!loading && !fetchError && featuredDto && (
        <EventsBar
          events={events}
          selectedId={featuredDto.event.id}
          onSelect={setSelectedId}
        />
      )}

      <footer className="main-footer">
        {['KILAM', 'Info', 'Crear', 'Hacer / actualizar', 'Sugerencias y quejas'].map((item, i) => (
          <span key={i} className="main-footer__link">
            {i > 0 && <span aria-hidden="true">· </span>}
            {item}
          </span>
        ))}
      </footer>
    </div>
  )
}
