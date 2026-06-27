import { useRef, useState } from 'react'
import '../styles/pages/settings.css'
import { useConfig } from '../store/config'
import { useAuth } from '../store/auth'
import { useNavigation } from '../store/navigation'
import { updateConfig, logout } from '../lib/ipc'
import { LauncherConfig } from '../types/config'
import Button from '../components/ui/Button'
import Input from '../components/ui/Input'

function Toggle({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={`toggle ${checked ? 'toggle--on' : 'toggle--off'}`}
    >
      <span className={`toggle__thumb ${checked ? 'toggle__thumb--on' : 'toggle__thumb--off'}`} />
    </button>
  )
}

function RamSlider({
  label, value, min, max, step, onChange,
}: {
  label: string; value: number; min: number; max: number; step: number; onChange: (v: number) => void
}) {
  return (
    <div className="memory-slider-group">
      <div className="memory-slider-header">
        <span className="memory-slider-label">{label}</span>
        <span className="memory-slider-value">{value} MB</span>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
        className="memory-slider"
      />
    </div>
  )
}

export default function SettingsScreen() {
  const navigate     = useNavigation((s) => s.navigate)
  const config       = useConfig((s) => s.config)
  const setConfig    = useConfig((s) => s.setConfig)
  const profile      = useAuth((s) => s.profile)
  const clearProfile = useAuth((s) => s.clearProfile)

  const [javaPath,      setJavaPath]      = useState(() => config?.java_path ?? '')
  const [installDir,    setInstallDir]    = useState(() => config?.install_dir ?? '')
  const [closeOnLaunch, setCloseOnLaunch] = useState(() => config?.close_on_launch ?? false)
  const [minMemory,     setMinMemory]     = useState(() => config?.min_memory_mb ?? 512)
  const [maxMemory,     setMaxMemory]     = useState(() => config?.max_memory_mb ?? 2048)
  const [saving,        setSaving]        = useState(false)
  const [saved,         setSaved]         = useState(false)
  const [saveError,     setSaveError]     = useState<string | null>(null)
  const [loggingOut,    setLoggingOut]    = useState(false)

  const javaInputRef = useRef<HTMLInputElement>(null)
  const dirInputRef  = useRef<HTMLInputElement>(null)

  const handleSave = async () => {
    if (!config) return
    setSaving(true)
    setSaveError(null)
    const newConfig: LauncherConfig = {
      ...config,
      java_path: javaPath || undefined,
      install_dir: installDir,
      close_on_launch: closeOnLaunch,
      min_memory_mb: minMemory,
      max_memory_mb: maxMemory,
    }
    try {
      await updateConfig(newConfig)
      setConfig(newConfig)
      setSaved(true)
      setTimeout(() => setSaved(false), 2000)
    } catch (e) {
      const err = e as { message?: string }
      setSaveError(err.message ?? 'Error al guardar')
    } finally {
      setSaving(false)
    }
  }

  const handleLogout = async () => {
    setLoggingOut(true)
    try {
      await logout()
      clearProfile()
    } finally {
      setLoggingOut(false)
    }
  }

  return (
    <div className="settings-screen">
      <header className="settings-header">
        <Button variant="ghost" size="sm" onClick={() => navigate('main')}>
          ← Volver
        </Button>
        <h1 className="settings-header__title">Ajustes</h1>
      </header>

      <main className="settings-body">
        <section className="settings-section">
          <h2 className="settings-section__title">Cuenta</h2>
          <div className="settings-account-row">
            <div className="settings-account-info">
              <span className="settings-account-label">Sesión activa</span>
              <span className="settings-account-name">{profile?.username}</span>
            </div>
            <Button
              variant="danger"
              size="sm"
              onClick={handleLogout}
              loading={loggingOut}
              disabled={loggingOut}
            >
              Cerrar sesión ({profile?.username})
            </Button>
          </div>
          {saveError && <p className="settings-save-error">{saveError}</p>}
        </section>

        <section className="settings-section">
          <h2 className="settings-section__title">Launcher</h2>
          <div className="settings-toggle-row">
            <span className="settings-toggle-label">
              Cerrar el launcher al iniciar el juego
            </span>
            <Toggle checked={closeOnLaunch} onChange={setCloseOnLaunch} />
          </div>
        </section>

        <section className="settings-section">
          <h2 className="settings-section__title">Rutas</h2>

          <input
            ref={javaInputRef}
            type="file"
            hidden
            onChange={(e) => {
              const file = e.target.files?.[0]
              if (file) setJavaPath((file as File & { path?: string }).path ?? file.name)
              e.target.value = ''
            }}
          />
          <input
            ref={dirInputRef}
            type="file"
            hidden
            // @ts-expect-error webkitdirectory is not in the TS types
            webkitdirectory=""
            onChange={(e) => {
              const file = e.target.files?.[0]
              if (file) {
                const p = (file as File & { path?: string }).path
                const dir = p ? p.replace(/[\\/][^\\/]+$/, '') : file.webkitRelativePath.split('/')[0]
                setInstallDir(dir)
              }
              e.target.value = ''
            }}
          />

          <div className="settings-stack">
            <div className="input-with-btn">
              <Input
                label="Ruta de Java"
                value={javaPath}
                onChange={setJavaPath}
                placeholder="Autodetectar (JAVA_HOME → PATH)"
              />
              <Button variant="secondary" size="md" onClick={() => javaInputRef.current?.click()}>
                Detectar
              </Button>
            </div>
            <div className="input-with-btn">
              <Input
                label="Carpeta de instalación"
                value={installDir}
                onChange={setInstallDir}
                placeholder="~/.kilam"
              />
              <Button variant="secondary" size="md" onClick={() => dirInputRef.current?.click()}>
                Examinar
              </Button>
            </div>
          </div>
        </section>

        <section className="settings-section">
          <h2 className="settings-section__title">Memoria asignada (JVM)</h2>
          <div className="memory-sliders settings-row">
            <RamSlider
              label="Mínimo"
              value={minMemory}
              min={256}
              max={8192}
              step={256}
              onChange={(v) => { setMinMemory(v); if (v > maxMemory) setMaxMemory(v) }}
            />
            <RamSlider
              label="Máximo"
              value={maxMemory}
              min={256}
              max={8192}
              step={256}
              onChange={(v) => { setMaxMemory(v); if (v < minMemory) setMinMemory(v) }}
            />
          </div>
        </section>
      </main>

      <footer className="settings-footer">
        <div className="settings-footer__btn-group">
          <Button
            variant={saved ? 'secondary' : 'primary'}
            size="sm"
            onClick={handleSave}
            loading={saving}
            disabled={saving || saved}
          >
            {saved ? '✓ Guardado' : 'Guardar cambios'}
          </Button>
        </div>
      </footer>
    </div>
  )
}
