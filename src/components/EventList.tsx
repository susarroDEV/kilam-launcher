import { useEffect , useState } from "react";
import { getActiveEvents } from "../lib/ipc";
import { EventDTO } from "../types/event_store";

function EventList({uuid} : {uuid: string}) {
  const [events, setEvents] = useState<EventDTO[]>([])
  const [error, setError] = useState("")
  const [loading, setLoading] = useState(false)

  
  useEffect(() => {
    let cancelled = false
    
    const load = async (uuid: string) => {
      try {
        setLoading(true)
        const events = await getActiveEvents(uuid)
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
  }, [uuid])

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
                    <li key={dto.event.id}>
                      <header>
                        {dto.status}
                      </header>
                      <h1>
                        {dto.event.name}
                      </h1>
                      <p>
                        {dto.event.description}
                      </p>
                    </li>
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

export default EventList;
