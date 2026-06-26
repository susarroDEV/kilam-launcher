import '../../styles/components/event-card.css'
import { Event, EventStatus } from '../../types/event_store'
import Tag, { TagStatus } from './Tag'

interface EventCardProps {
  event: Event
  status: EventStatus
  onClick?: () => void
  active?: boolean
}

function eventStatusToTag(s: EventStatus): TagStatus {
  switch (s) {
    case EventStatus.Ready:        return 'listo'
    case EventStatus.NotInstalled: return 'no-instalado'
    case EventStatus.Outdated:     return 'desactualizado'
  }
}

function statusModifier(s: EventStatus): string {
  switch (s) {
    case EventStatus.Ready:        return 'event-card--ready'
    case EventStatus.NotInstalled: return 'event-card--notinstalled'
    case EventStatus.Outdated:     return 'event-card--outdated'
  }
}

export default function EventCard({ event, status, onClick, active = false }: EventCardProps) {
  const clickable = onClick != null

  const classes = [
    'event-card',
    clickable ? 'event-card--clickable' : '',
    active ? 'event-card--active' : statusModifier(status),
  ].filter(Boolean).join(' ')

  return (
    <article onClick={clickable ? onClick : undefined} className={classes}>
      <div className="event-card__icon" aria-hidden="true">⛏</div>
      <div className="event-card__info">
        <p className="event-card__name">{event.name}</p>
        <p className="event-card__version">{event.minecraft_version}</p>
      </div>
      <Tag status={eventStatusToTag(status)} />
    </article>
  )
}
