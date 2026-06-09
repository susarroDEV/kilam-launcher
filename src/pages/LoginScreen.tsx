import { useState } from "react"
import { login } from "../lib/ipc"
import { useAuth } from "../store/auth"

function LoginScreen() {
  const [username, setUsername] = useState('')
  const setProfile = useAuth((state) => state.setProfile)

  async function handleLogin(username: string) {
    const profile = await login(username)
    setProfile(profile)
  }

  return (
    <main>
      <input
        value={username}
        onChange= {(e) => setUsername(e.target.value)}
      >
      </input>
      <button
        onClick={() => {handleLogin(username)}}
      >
        Login
      </button>
    </main>
  )
}

export default LoginScreen
