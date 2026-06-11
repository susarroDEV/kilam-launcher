import { useEffect, useState } from "react"
import "./App.css"
import { useConfig } from "./store/config"
import { getConfig, getSession } from "./lib/ipc"
import { useAuth } from "./store/auth"
import MainScreen from "./pages/MainScreen"
import LoginScreen from "./pages/LoginScreen"
import ConfigTest from "./components/ConfigTest"

function App() {
  const [checking, setChecking] = useState(true)

  const config = useConfig((state) => state.config) 
  const setConfig = useConfig((state) => state.setConfig)

  const profile = useAuth((state) => state.profile)
  const setProfile = useAuth((state) => state.setProfile)


  useEffect(() => {
    const loadConfig = async() => {
      setConfig(await getConfig())
    }
    
    const loadSession = async() => {
      const p = await getSession()
      if (p) {
        setProfile(p)
      }
      setChecking(false)
    }

    loadConfig()
    loadSession()
  }, [setConfig, setProfile])

  return (
    <>
      <header>
        <ConfigTest config = {config}/>
      </header>
      {!checking  &&
        (profile ?
        <MainScreen/>
        :
        <LoginScreen/>
      )}
    </>
  )
}

export default App
