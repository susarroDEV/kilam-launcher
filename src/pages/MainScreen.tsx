import EventList from "../components/EventList"
import { logout } from "../lib/ipc"
import { useAuth } from "../store/auth"

function MainScreen() {
  const profile = useAuth((state) => state.profile)
  const clearProfile = useAuth((state) => state.clearProfile)

  async function handleLogout() {
    await logout()
    clearProfile()
  }

  return (
    <main>
      <h1>
        Nombre:
        <span>
          {profile?.username}
        </span>
      </h1>
      <h2>
        ID:
        <span>
          {profile?.uuid}
        </span>
      </h2>
      <button
        onClick={() => {handleLogout()}}
      >
        Logout
      </button>
      {
        profile && 
        <EventList
          uuid={profile.uuid}
          />
      }
    </main>
  )
}

export default MainScreen
