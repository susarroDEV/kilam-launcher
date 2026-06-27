import React, { useState } from 'react'
import '../styles/pages/login.css'
import { openUrl } from '@tauri-apps/plugin-opener'
import { login, loginMicrosoft } from '../lib/ipc'
import { useAuth } from '../store/auth'
import { useNavigation } from '../store/navigation'
import Button from '../components/ui/Button'
import Input from '../components/ui/Input'

const USERNAME_RE = /^[A-Za-z0-9_]{4,16}$/

const MS_ICON = (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
    <rect x="0" y="0" width="7" height="7" fill="#F25022" />
    <rect x="9" y="0" width="7" height="7" fill="#7FBA00" />
    <rect x="0" y="9" width="7" height="7" fill="#00A4EF" />
    <rect x="9" y="9" width="7" height="7" fill="#FFB900" />
  </svg>
)

export default function LoginScreen() {
  const [username, setUsername] = useState('')
  const [loading, setLoading]   = useState(false)
  const [msLoading, setMsLoading] = useState(false)
  const [error, setError]       = useState('')
  const setProfile = useAuth((s) => s.setProfile)
  const navigate   = useNavigation((s) => s.navigate)

  const touched    = username.length > 0
  const isValid    = USERNAME_RE.test(username)
  const inputError = touched && !isValid
    ? '× Nombre inválido. Usa 4-16 caracteres [A-Z a-z 0-9 _].'
    : undefined

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault()
    if (!isValid) return
    setLoading(true)
    setError('')
    try {
      const profile = await login(username)
      navigate('main')
      setProfile(profile)
    } catch (err) {
      const e = err as { message?: string }
      setError(e.message ?? 'Error al iniciar sesión')
    } finally {
      setLoading(false)
    }
  }

  async function handleMicrosoftLogin() {
    setMsLoading(true)
    setError('')
    try {
      const profile = await loginMicrosoft()
      setProfile(profile)
      navigate('main')
    } catch (err) {
      const e = err as { message?: string }
      setError(e.message ?? 'Error al iniciar sesión con Microsoft')
    } finally {
      setMsLoading(false)
    }
  }

  return (
    <main className="login-screen">
      <section className="login-card">
        <div className="login-logo">
          <div className="login-logo__circle">
            <img
              src="/logos/kilam-rounded.avif"
              alt="KILAM"
              width="56"
              height="56"
              onError={(e) => {
                e.currentTarget.style.display = 'none'
                const parent = e.currentTarget.parentElement
                if (parent) {
                  parent.innerHTML = `<span style="color:var(--color-secondary);font-weight:900;font-size:13px;line-height:1.1;text-align:center">KIL<br/>AM</span>`
                }
              }}
            />
          </div>
          <span className="login-logo__wordmark">LAUNCHER</span>
        </div>

        <form onSubmit={handleLogin} className="login-form">
          <Input
            label="Nombre de usuario"
            value={username}
            onChange={setUsername}
            error={inputError}
            hint="4-16 caracteres [A-Z a-z 0-9 _]"
            placeholder="Steve"
          />

          <Button
            type="submit"
            variant="primary"
            disabled={!isValid}
            loading={loading}
            className="btn--block"
          >
            ▶ Entrar en modo offline
          </Button>

          {touched && !isValid && (
            <p className="login-hint--error">
              El botón se habilitará al ingresar el nombre.
            </p>
          )}

          {error && <p className="login-hint--error">{error}</p>}
        </form>

        <div className="login-separator" role="separator">
          <div className="login-separator__line" />
          <span className="login-separator__label">o</span>
          <div className="login-separator__line" />
        </div>

        <Button
          variant="secondary"
          loading={msLoading}
          disabled={msLoading}
          onClick={handleMicrosoftLogin}
          className="btn--block"
        >
          {MS_ICON}
          Iniciar sesión con Microsoft
        </Button>

        <footer className="login-footer">
          <span className="login-footer__item">v0.1.0</span>
          <button
            type="button"
            className="login-footer__site-link"
            onClick={() => openUrl('https://kilam.net')}
          >
            kilam.net
          </button>
        </footer>
      </section>
    </main>
  )
}
