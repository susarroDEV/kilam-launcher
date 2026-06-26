import '../../styles/components/button.css'
import React from 'react'

interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
  type?: 'button' | 'submit' | 'reset'
  onClick?: () => void
  children: React.ReactNode
  className?: string
  style?: React.CSSProperties
}

export default function Button({
  variant = 'primary',
  size = 'md',
  disabled = false,
  loading = false,
  type = 'button',
  onClick,
  children,
  className,
  style,
}: ButtonProps) {
  const isDisabled = disabled || loading

  const classes = [
    'btn',
    `btn--${variant}`,
    `btn--${size}`,
    className,
  ].filter(Boolean).join(' ')

  return (
    <button
      type={type}
      disabled={isDisabled}
      onClick={onClick}
      className={classes}
      style={style}
    >
      {loading && <span className="spinner" />}
      {children}
    </button>
  )
}
