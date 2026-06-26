import { useEffect, useState } from 'react'
import './styles/global.css'
import { useConfig } from './store/config'
import { useAuth } from './store/auth'
import { useNavigation } from './store/navigation'
import { getConfig, getSession } from './lib/ipc'
import LoginScreen from './pages/LoginScreen'
import MainScreen from './pages/MainScreen'
import DownloadScreen from './pages/DownloadScreen'
import SettingsScreen from './pages/SettingsScreen'

export default function App() {
  const [checking, setChecking] = useState(true)

  const setConfig  = useConfig((s) => s.setConfig)
  const profile    = useAuth((s) => s.profile)
  const setProfile = useAuth((s) => s.setProfile)
  const screen     = useNavigation((s) => s.screen)

  useEffect(() => {
    const init = async () => {
      const [cfg, session] = await Promise.all([getConfig(), getSession()])
      setConfig(cfg)
      if (session) setProfile(session)
      setChecking(false)
    }
    init()
  }, [setConfig, setProfile])

  if (checking) {
    return (
      <div style={{
        height: '100%',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: 'var(--color-text-muted)',
        fontSize: '13px',
      }}>
        Cargando...
      </div>
    )
  }

  if (!profile) return <LoginScreen />

  if (screen === 'download') return <DownloadScreen />
  if (screen === 'settings') return <SettingsScreen />
  return <MainScreen />
}
