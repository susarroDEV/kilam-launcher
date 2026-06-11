import { LauncherConfig } from "../types/config"

function ConfigTest({config}: {config: LauncherConfig | null}) {
  return (
    <>
      {(
        config ?
          <p>
            {config.install_dir}
          </p>
          :
          <p>
            Cargando config...
          </p>
      )}
    </>
  )
}

export default ConfigTest