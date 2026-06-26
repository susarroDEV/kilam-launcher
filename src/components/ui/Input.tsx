import '../../styles/components/input.css'

interface InputProps {
  label?: string
  value: string
  onChange: (value: string) => void
  error?: string
  placeholder?: string
  hint?: string
  type?: string
  disabled?: boolean
  id?: string
}

export default function Input({
  label,
  value,
  onChange,
  error,
  placeholder,
  hint,
  type = 'text',
  disabled = false,
  id,
}: InputProps) {
  const inputId = id ?? label?.toLowerCase().replace(/\s+/g, '-')

  return (
    <div className="field">
      {label && (
        <label htmlFor={inputId} className="field__label">
          {label}
        </label>
      )}
      <input
        id={inputId}
        type={type}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        className={['field__input', error ? 'field__input--error' : ''].filter(Boolean).join(' ')}
      />
      {error && <span className="field__error">{error}</span>}
      {hint && !error && <span className="field__hint">{hint}</span>}
    </div>
  )
}
