import '../../styles/components/tag.css'

export type TagStatus = 'listo' | 'descargando' | 'no-instalado' | 'proximamente' | 'desactualizado'

interface TagProps {
  status: TagStatus
}

const labels: Record<TagStatus, string> = {
  listo:          '● Listo',
  descargando:    '↓ Descargando',
  'no-instalado': '○ No instalado',
  proximamente:   '◌ Próximamente',
  desactualizado: '↑ Actualizar',
}

export default function Tag({ status }: TagProps) {
  return (
    <span className={`tag tag--${status}`}>
      {labels[status]}
    </span>
  )
}
