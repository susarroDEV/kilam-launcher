import { useEffect } from "react";
import "./App.css";
import { useConfig } from "./store/config";
import { getConfig } from "./lib/ipc";

function App() {
  const config = useConfig((state) => state.config) 
  const setConfig = useConfig((state) => state.setConfig)

  useEffect(() => {
    const loadConfig = async() => {
      setConfig(await getConfig())
    }
    loadConfig()
  }, []
  )

  return (
    <>
    <p>
      {(
        config 
        ?
        <span>
          {config?.install_dir}
        </span>
        :
        <span>
          Cargando config...
        </span>
      )}
    </p>
    </>
  )
}

export default App;
