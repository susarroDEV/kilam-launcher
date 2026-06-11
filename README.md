<a name="readme-top"></a>

<div align="center" style="margin-top: 20px; margin-bottom: 20px;">

# 🚀 KILAM Launcher 🎮

<a href="https://kilam.net">
  <img width="200px" src="public/logos/kilam-rounded.avif" alt="Logo KILAM" />
</a>

**El launcher oficial de los eventos de Minecraft del Grupo KILAM**

_Inicia sesión, elige tu evento y juega. El launcher se encarga del resto._

</div>

<div align="center">

[![Tauri](https://img.shields.io/badge/Tauri%202-24C8D8?style=for-the-badge&logo=tauri&logoColor=white&color=1B1B1F)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white&color=b7410e)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white&color=blue)](https://www.typescriptlang.org/)

</div>

🌍 **KILAM Launcher** es la aplicación de escritorio que prepara y lanza los eventos de Minecraft organizados por la comunidad [KILAM](https://kilam.net). En lugar de instalar mods a mano, copiar carpetas y rezar para que las versiones coincidan, el launcher descarga exactamente lo que cada evento necesita, verifica la integridad de cada archivo y arranca el juego listo para jugar. ✨

<details>
<summary>📜 Tabla de contenidos</summary>

- [🚀 KILAM Launcher 🎮](#-kilam-launcher-)
  - [✨ ¿Qué hace?](#-qué-hace)
  - [🏛️ Arquitectura](#️-arquitectura)
    - [Las tres capas](#las-tres-capas)
    - [Módulos de negocio](#módulos-de-negocio)
    - [Convenciones del proyecto](#convenciones-del-proyecto)
  - [🗺️ Estado del proyecto](#️-estado-del-proyecto)
  - [📖 Para empezar](#-para-empezar)
    - [⚙️ Prerequisitos](#️-prerequisitos)
    - [🔧 Instalación](#-instalación)
  - [🗂️ Estructura del proyecto](#️-estructura-del-proyecto)
  - [🛠️ Stack](#️-stack)
  - [📬 Contacto](#-contacto)

</details>

## ✨ ¿Qué hace?

El launcher tiene una responsabilidad acotada y deliberadamente simple:

1. 🔐 **Autenticación** — Identifica al jugador (modo offline con UUID determinista estilo Mojang; Microsoft OAuth en el roadmap).
2. 📅 **Catálogo de eventos** — Descarga el manifest remoto de eventos KILAM y muestra solo aquellos a los que el jugador tiene acceso.
3. ⬇️ **Preparación del entorno** — Descarga los mods y assets de cada evento, verificando cada archivo con SHA-256. Si algo falta o ha cambiado, lo sabe.
4. 🎮 **Lanzamiento** — Construye los argumentos de la JVM y arranca Minecraft con el modloader y la versión exacta del evento.

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>

## 🏛️ Arquitectura

Este proyecto no es solo un launcher: es también un ejercicio deliberado de arquitectura limpia en Rust. Si vas a contribuir, **respeta el modelo de capas** — es la regla número uno del repo.

### Las tres capas

```
┌──────────────────────┐
│     Presentación     │  React + TypeScript
│   (UI, estado, UX)   │
└──────────┬───────────┘
           │  Tauri invoke / events  (adaptador, no capa)
┌──────────▼───────────┐
│      Business        │  Rust puro
│  (auth, events,      │
│   download, launch)  │
└──────────┬───────────┘
           │  traits abstractos
┌──────────▼───────────┐
│   Infraestructura    │  reqwest, tokio fs,
│                      │  std::process, store
└──────────────────────┘
```

**Regla clave:** la lógica de negocio nunca toca Tauri directamente. Los *commands* son adaptadores finos — reciben la petición, construyen la implementación de infraestructura y delegan en el trait. Nada más.

El patrón se repite en cada módulo:

```
business/<módulo>.rs   →  define el trait (el contrato) + entidades + lógica pura
infra/<módulo>.rs      →  implementa el trait (red, disco, proceso...)
commands/<módulo>.rs   →  adaptador fino entre Tauri y el trait
```

Esto permite, por ejemplo, añadir `MicrosoftAuthProvider` el día de mañana sin tocar una sola línea de business ni de commands.

### Módulos de negocio

| Módulo | Pregunta que responde |
|---|---|
| `auth` | ¿Quién eres? |
| `event_store` | ¿Qué eventos hay y en qué estado están? |
| `downloader` | ¿Cómo preparo el entorno para un evento? |
| `launcher` | ¿Cómo arranco Minecraft? |

### Convenciones del proyecto

- 🧱 **Errores tipados** con `thiserror`: cada módulo tiene su subenum (`AuthError`, `EventError`...) que `LauncherError` envuelve con `#[from]`.
- 🐻 **Zustand** para estado global en el front; `useState` para lo efímero.
- 💾 **Persistencia** con `tauri-plugin-store`: un fichero por responsabilidad (`config.json`, `session.json`).
- 🔌 **IPC centralizado**: el front solo habla con Rust a través de `lib/ipc.ts`. Nunca `invoke` suelto en componentes.
- 🪞 **Tipos espejo**: cada entidad Rust tiene su gemelo TypeScript en `types/`.

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>

## 🗺️ Estado del proyecto

El desarrollo avanza por fases, cada una con su documento de diseño cerrado **antes** de escribir código:

| Fase | Contenido | Estado |
|---|---|---|
| 1️⃣ Esqueleto que camina | Scaffold, capas, slice vertical con `LauncherConfig` | ✅ |
| 2️⃣ Auth offline | `AuthProvider`, sesión persistente, login/logout | ✅ |
| 3️⃣ Event Store | Manifest remoto, filtrado por acceso, estado de eventos | ✅ |
| 4️⃣ Downloader | Descarga de assets con SHA-256 y progreso en tiempo real | 🔨 En curso |
| 5️⃣ Launcher | Provisión del cliente base y lanzamiento de Minecraft | ⬜ |
| 6️⃣ Polish | UI con identidad KILAM, Microsoft OAuth | ⬜ |

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>

## 📖 Para empezar

### ⚙️ Prerequisitos

- [Rust](https://www.rust-lang.org/tools/install) (toolchain estable)
- [Node.js](https://nodejs.org/) ≥ 18
- PNPM

  ```sh
  npm install -g pnpm
  ```

- Las [dependencias de sistema de Tauri 2](https://v2.tauri.app/start/prerequisites/) según tu plataforma

### 🔧 Instalación

1. Clona el repositorio

   ```sh
   git clone https://github.com/susarrodev/kilam-launcher.git
   cd kilam-launcher
   ```

2. Instala las dependencias

   ```sh
   pnpm install
   ```

3. Arranca en modo desarrollo

   ```sh
   pnpm tauri dev
   ```

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>

## 🗂️ Estructura del proyecto

```
kilam-launcher/
├── src/                    # Frontend (React + TypeScript)
│   ├── App.tsx             # Routing condicional según sesión
│   ├── lib/ipc.ts          # Único punto de invoke/listen hacia Rust
│   ├── store/              # Stores de Zustand (config, auth)
│   ├── types/              # Tipos espejo de las entidades Rust
│   ├── components/         # Componentes (EventList, ...)
│   └── pages/              # LoginScreen, MainScreen
│
└── src-tauri/src/          # Backend (Rust)
    ├── lib.rs              # Entry point, registro de commands
    ├── error.rs            # LauncherError + Result<T> propio
    ├── commands/           # Adaptadores finos Tauri ↔ business
    ├── business/           # Lógica pura: traits, entidades, reglas
    └── infra/              # Implementaciones: red, disco, store
```

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>

## 🛠️ Stack

- [![Tauri](https://img.shields.io/badge/Tauri%202-fff?style=for-the-badge&logo=tauri&logoColor=24C8D8&color=1B1B1F)](https://tauri.app/)
- [![Rust](https://img.shields.io/badge/Rust-b7410e?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
- [![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://react.dev/)
- [![TypeScript](https://img.shields.io/badge/Typescript-007ACC?style=for-the-badge&logo=typescript&logoColor=white&color=blue)](https://www.typescriptlang.org/)
- [![Zustand](https://img.shields.io/badge/Zustand-433e38?style=for-the-badge&logo=react&logoColor=white)](https://zustand-demo.pmnd.rs/)

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>

## 📬 Contacto

¿Preguntas, sugerencias o ganas de un evento KILAM? Pasa por [kilam.net](https://kilam.net) o escribe a [susarroDEV](https://susarrodev.com). 💌

<p align="right">(<a href="#readme-top">volver arriba</a>)</p>