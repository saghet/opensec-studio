# MANUAL_DESARROLLO_OPENSEC.md — Manual de Desarrollo

## 1. Preparar el Entorno
- **Rust:** `rustup` / `cargo` (`edition = "2021"`, `rustc >= 1.91` para imágenes Alpine).
- **Python:** Python 3.11+ (sin dependencias externas para tests unitarios).
- **Docker:** Docker Desktop o Docker Engine + Compose v2.

## 2. Comandos Probados

| Tarea | Comando probado |
| :--- | :--- |
| **Compilar y probar todo Rust** | `cargo test --workspace` |
| **Probar Hardware Manager & Kismet Adapter** | `python -m unittest discover -s services/opensec-wireless-agent -v` |
| **Probar Analizador PCAP (`tshark`)** | `python -m unittest discover -s services/opensec-pcap-analyzer -v` |
| **Validar Compose (dev/prod)** | `docker compose -f docker-compose.yml -f docker-compose.prod.yml config -q` |
| **Levantar entorno dev en Docker** | `docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build` |

## 3. Dónde Modificar Cada Componente

- **Agregar regla de Evaluación / WIDS / RF / Risk:**
  - `crates/opensec-core/src/{assessment.rs,wids.rs,rf.rs,risk.rs}`
- **Agregar endpoint REST o consulta PostgreSQL:**
  - `services/opensec-api/src/main.rs` (y esquema en `docker/postgres/init.sql` + `ensure_schema()`).
- **Modificar extracción `tshark` PCAP/PCAPNG:**
  - `services/opensec-pcap-analyzer/app.py` y `test_pcap_analyzer.py`.
- **Modificar interfaz *Wireless Security Center* (FortiGate-style):**
  - `services/opensec-web/index.html` y proxy en `docker/web/nginx.conf`.
- **Modificar detección de hardware Linux / Kismet:**
  - `services/opensec-wireless-agent/{hw_manager.py,kismet_adapter.py}`.

## 4. Publicar una Nueva Versión
1. Ejecutar `cargo test --workspace` y los `unittest` de Python.
2. Reconstruir contenedores con `docker compose ... up -d --build` y validar `/api/health`.
3. Actualizar `CHANGELOG.md`, hacer `git commit` y `git tag vX.Y.Z`.
