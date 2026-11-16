# DOCKER_MIGRATION.md — Arquitectura y Operación Docker (FASE 2)

**Proyecto:** OpenSec Studio — Wireless Security Platform  
**Composición:** Multi-servicio desacoplada (sin contenedor monolítico).

---

## 1. Justificación de Servicios

| Servicio | Imagen / Base | Red | Justificación técnica |
| :--- | :--- | :--- | :--- |
| `opensec-postgres` | `postgres:16.6-alpine3.21` | `opensec_internal` | Persistencia relacional de organizaciones, usuarios, scopes, inventario Wi-Fi, alertas WIDS y auditoría. |
| `opensec-redis` | `redis:7.4.2-alpine3.21` | `opensec_internal` | Bus de eventos (`Redis Streams`), caché de estado de sensores y Kill Switch. Protegido con `--requirepass`. |
| `opensec-api` | `rust:1.85.0-alpine3.21` → `alpine:3.21.3` | `opensec_internal` + `opensec_edge` | Control Plane REST/WebSocket, RBAC, Scope Guard y puente hacia `opensec-pcap-analyzer`. |
| `opensec-worker` | `rust:1.85.0-alpine3.21` → `alpine:3.21.3` | `opensec_internal` | Data & Intelligence: normalización, correlación temporal WIDS, evaluación y scoring. |
| `opensec-pcap-analyzer` | `alpine:3.21.3` (`tshark` + `python3`) | `opensec_internal` | Aislamiento estricto del motor GPL-2.0 (`tshark`) en proceso/contenedor independiente; acceso `:ro` a `opensec_pcap_vault`. |
| `opensec-web` | `nginxinc/nginx-unprivileged:1.27.4-alpine3.21` | `opensec_edge` | Dashboard *Wireless Security Center* no privilegiado; único puerto publicado (`8088` -> `8080`). |
| `opensec-kismet` *(perfil `wireless-sensor`)* | `debian:bookworm-20250224-slim` | `host` (Linux) | Sensor pasivo 802.11; separado en perfil opcional para requerir `NET_ADMIN`/`NET_RAW` solo en hosts Linux con tarjeta Wi-Fi. |
| `opensec-wireless-agent` *(perfil `wireless-sensor`)* | `debian:bookworm-20250224-slim` | `host` (Linux) | Detección de hardware (`iw`, `ethtool`, `nl80211`) y adaptador Kismet → Redis Streams. |

---

## 2. Seguridad y Hardening

- **Aislamiento de red:** `opensec_internal` tiene `internal: true`. En producción (`docker-compose.prod.yml`), PostgreSQL, Redis, Worker y PCAP Analyzer **no exponen puertos al host**.
- **Contenedores endurecidos (`prod`):** `read_only: true`, `cap_drop: [ALL]`, `no-new-privileges:true`, usuarios no-root (`10001`, `10002`, `nginx-unprivileged`), `tmpfs` con `noexec,nosuid` y límites explícitos de CPU/memoria.
- **Logs estructurados:** Driver `json-file` con rotación (`max-size: 10m`, `max-file: 5`).
- **Gestión de credenciales:** Todas las credenciales se inyectan vía `.env` (plantilla en `.env.example` sin secretos).

---

## 3. Operación: Instalación, Actualización, Backup y Recuperación

- **Instalación:** `./scripts/install.sh [dev|prod]` (verifica que `.env` no contenga `CHANGE_ME`).
- **Actualización:** `./scripts/update.sh [dev|prod]` (ejecuta respaldo automático previo antes de reconstruir).
- **Backup:** `./scripts/backup.sh` genera dump custom de PostgreSQL (`pg_dump -Fc`), archivo comprimido de `opensec_pcap_vault` y `SHA256SUMS`.
- **Restauración / Recuperación:** `./scripts/restore.sh <dump.dump> [pcaps.tar.gz]` detiene `api` y `worker`, restaura DB y bóveda PCAP, y reinicia servicios.
- **SBOM y Escaneo de Vulnerabilidades:** `./scripts/sbom-scan.sh` genera SBOM en formato CycloneDX (`syft`) y escaneo CVE (`trivy`).
