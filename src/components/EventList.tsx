import { useEffect , useState } from "react"
import { getActiveEvents } from "../lib/ipc"
import { EventDTO } from "../types/event_store"
import EventItem from "./EventItem"
import { useConfig } from "../store/config"

function EventList({uuid} : {uuid: string}) {
  const [events, setEvents] = useState<EventDTO[]>([])
  const [error, setError] = useState("")
  const [loading, setLoading] = useState(false)

  const config = useConfig((state) => state.config)
  
  useEffect(() => {
    let cancelled = false
    
    const load = async (uuid: string) => {
      try {
        if (!config) return
        setLoading(true)
        const events = await getActiveEvents(uuid, config?.install_dir)
        if (!cancelled) setEvents(events)
      } catch (e) {
        setError(String(e))
      } finally {
        setLoading(false)
      }
    }

    load(uuid)
    
    return () => { 
      cancelled = true 
    }
  }, [uuid, config])

  return (
    <>
      {(
        loading ?
        <p>
          Cargando...
        </p>
        :
        (
          !error ?
            (events.length == 0 ?
              <p>
                No hay eventos disponibles
              </p>
              :
              <ul>
                {
                events.map(
                  dto => (
                    <EventItem key={dto.event.id} dto={dto} />
                  )
                )
              }
              </ul>
            )
          :
          <p>
            Ha habido un error: {error}
          </p>
        )
      )}
    </>
  )
}

export default EventList
