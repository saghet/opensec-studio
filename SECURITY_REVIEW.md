# SECURITY_REVIEW.md — Revisión de Seguridad y Gobernanza (FASE 9)

---

## 1. Controles Implementados (Regla 13)

| Control | Implementación | Estado |
| :--- | :--- | :--- |
| **Modo pasivo por defecto** | `OPENSEC_DEFAULT_MODE=passive_only` en `.env.example`, `opensec-api` y `opensec-worker`. | Activo |
| **Kill Switch** | Endpoints `POST /api/v1/kill-switch/{enable,disable}` (`AtomicBool` en `opensec-api`) y chequeo `kill_switch` en `KismetClient`. | Implementado y probado |
| **Scope y Allowlist** | Tablas `authorization_scopes` y `authorized_networks` en `docker/postgres/init.sql` con separación por `org_id`. | Esquema definido |
| **RBAC y Auditoría** | Roles (`admin`, `analyst`, `auditor`, `viewer`) y tabla `audit_logs` append-only. | Esquema definido |
| **Protección PCAP y Retención** | `resolve_safe_pcap_path()` bloquea *path traversal* y `purge_expired_pcaps()` purga archivos expirados (`OPENSEC_PCAP_RETENTION_DAYS=7`). | Implementado y probado |
| **Hardening Docker (`prod`)** | `read_only: true`, `cap_drop: [ALL]`, `no-new-privileges:true`, `opensec_internal` sin puertos públicos, límites CPU/RAM. | Validado en `docker-compose.prod.yml` |

## 2. Alcance Defensivo

OpenSec Studio Wireless Platform **no implementa** capacidades ofensivas automatizadas (desautenticación, inyección ofensiva de paquetes, cracking automatizado ni fuerza bruta WPS). Toda evaluación opera de forma pasiva y sobre redes autorizadas.
