import React from "react"
import { useState } from "react"
import { login } from "../lib/ipc"
import { useAuth } from "../store/auth"

function LoginScreen() {
  const [error, setError] = useState("")
  const [loading, setLoading] = useState(false)

  const [username, setUsername] = useState('')
  const setProfile = useAuth((state) => state.setProfile)

  async function handleLogin(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    setLoading(true)
    try {
      const profile = await login(username)
      setProfile(profile)
    } catch (e) {
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }

  return (
    <main>
      <form onSubmit={handleLogin}>
        <input
          value={username}
          onChange= {(e) => setUsername(e.target.value)}
        >
        </input>
        <button
          type="submit"
          >
          Login
        </button>
      </form>
      {
        loading &&
        <p>
          Cargando...
        </p>
      }
      {
        error != "" &&
        <p>
          Ha habido un error: {error}
        </p>
      }
    </main>
  )
}

export default LoginScreen
