import { useEffect , useState } from "react";
import { getActiveEvents } from "../lib/ipc";
import { EventDTO } from "../types/event_store";

function EventList({uuid} : {uuid: string}) {
  const [events, setEvents] = useState<EventDTO[]>([])

  useEffect(() => {
    const load = async () => {
      setEvents(await getActiveEvents(uuid))
    }
    load()
  }, [uuid])

  return (
    events.length == 0 ?
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
}

export default EventList;
