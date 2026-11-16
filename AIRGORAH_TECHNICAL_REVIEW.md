# AIRGORAH_TECHNICAL_REVIEW.md — Auditoría Técnica de Airgorah (FASE 5)

**Proyecto auditado:** `https://github.com/martin-olivier/airgorah`  
**Versión inspeccionada:** `v0.9.0` (Rust `edition = "2024"`, `resolver = "3"`)  
**Licencia:** **MIT** (`Copyright Martin Olivier`) — compatible con Apache-2.0 manteniendo avisos de copyright.

---

## 1. Arquitectura Real de Airgorah (`v0.9.0`)

Airgorah está estructurado como un *Cargo workspace* con 3 crates:

| Crate | Tipo | Dependencias clave | Responsabilidad |
| :--- | :--- | :--- | :--- |
| `crates/common` (`airgorah-common`) | Librería | `serde`, `serde_json`, `which`, `libwifi = "0.5"`, `radiotap = "1"` | Tipos compartidos (`types.rs`, `channel.rs`, `handshake.rs`), validación de dependencias (`deps.rs`) y protocolo IPC (`ipc.rs`). |
| `crates/agent` (`airgorah-agent`) | Binario privilegiado | `airgorah-common`, `nix`, `libc`, `libwifi`, `radiotap`, `csv`, `ctrlc`, `chrono` | Proceso privilegiado que gestiona interfaces Wi-Fi, escaneo, captura y operaciones de desautenticación. |
| `crates/gui` (`airgorah`) | Binario GUI | `airgorah-common`, `gtk4 = "0.11"`, `ureq`, `toml`, `regex` | Interfaz de escritorio Linux en **GTK4**. |

---

## 2. Evaluación según Regla 6

1. **¿Es una librería de integración directa?**
   - **No.** Es una aplicación de escritorio Linux GTK4 con un agente privilegiado acoplado mediante un protocolo IPC privado (`crates/common/src/ipc.rs`). Sus comandos CLI/IPC **no constituyen una API pública estable**.
2. **¿Qué crates y componentes son aprovechables?**
   - **Directamente reutilizables (ecosistema Rust):** `libwifi` (`0.5`) y `radiotap` (`1`) para decodificación pasiva de tramas IEEE 802.11 y cabeceras Radiotap en memoria sin depender de GTK4 ni de binarios externos.
   - **Reutilizable como referencia de diseño (con licencia MIT):** La separación de privilegios entre un agente mínimo (`agent`) y la capa de presentación (`gui`), así como las estructuras de canales y tipos 802.11 en `crates/common/src/{channel.rs,types.rs}`.
3. **Componentes que requieren privilegios:**
   - `crates/agent` utiliza `nix` (`socket`, `signal`, `user`, `fs`) y `libc` para cambiar interfaces a *monitor mode*, fijar canales y capturar tramas crudas, requiriendo `CAP_NET_ADMIN` y `CAP_NET_RAW` (o ejecución como `root`).
4. **Dependencias externas del entorno Linux:**
   - Herramientas de sistema para gestión de interfaces/captura (verificadas vía `which` en `deps.rs`).
5. **Qué se descarta explícitamente:**
   - `crates/gui` (`gtk4`): **No se incrusta** en Tauri ni en `opensec-web`. La interfaz de OpenSec permanece 100% independiente.
   - Flujos ofensivos de `crates/agent` (desautenticación, inyección ofensiva o automatización de cracking): **Descartados**. OpenSec opera en modo pasivo y defensivo.

---

## 3. Decisión de Implementación para OpenSec Studio

- **No ejecutar `airgorah` como subproceso ni copiar el repositorio completo.**
- **Captura pasiva principal:** Delegada a **Kismet** (FASE 4) y **dumpcap/tshark** (FASE 6).
- **Adaptador Rust puro para OpenSec:** Cuando el motor Rust (`opensec-worker`) necesite inspeccionar metadatos Radiotap/802.11 en memoria de forma nativa (FASE 10 / Linux nativo), utilizará directamente los crates `radiotap` y `libwifi` bajo modelos propios de OpenSec, manteniendo la independencia total de GTK4 y de Airgorah.
