import '../../styles/components/progress-bar.css'

interface ProgressBarProps {
  value: number
  label?: string
  showPercentage?: boolean
}

export default function ProgressBar({ value, label, showPercentage = false }: ProgressBarProps) {
  const clamped = Math.min(100, Math.max(0, value))

  return (
    <div className="progress-bar">
      {(label || showPercentage) && (
        <div className="progress-bar__meta">
          {label && <span className="progress-bar__label">{label}</span>}
          {showPercentage && (
            <span className="progress-bar__pct">{Math.round(clamped)}%</span>
          )}
        </div>
      )}
      <div className="progress-bar__track">
        <div className="progress-bar__fill" style={{ width: `${clamped}%` }} />
      </div>
    </div>
  )
}
