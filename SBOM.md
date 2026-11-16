# SBOM.md — Licencias, Cadena de Suministro y SBOM (FASE 9)

---

## 1. Matriz de Licencias (Regla 16)

| Componente | Versión / Tag | Licencia | Estrategia de integración / aislamiento |
| :--- | :--- | :--- | :--- |
| **OpenSec Core (`opensec-api`, `opensec-worker`)** | `0.1.0` | **Apache-2.0** | Código propio en Rust (`std` sin dependencias externas en fase actual). |
| **Kismet** (`kismetwireless/kismet`) | `kismet-2026-10-R1` | **GPL-2.0** | Aislado en contenedor/servicio separado (`opensec-kismet`), comunicado vía REST/WS. |
| **Wireshark / tshark** (`wireshark/wireshark`) | `wireshark-4.4.9` | **GPL-2.0** | Aislado en contenedor/proceso separado (`opensec-pcap-analyzer`), comunicado vía HTTP/JSON. |
| **Airgorah** (`martin-olivier/airgorah`) | `v0.9.0` | **MIT** | Auditoría de referencia; se descarta `crates/gui` (GTK4) y flujos ofensivos. |
| **SentinelWiFi** (`navairgap/SentinelWiFi`) | `v2.1.0` | **MIT** | Referente de heurísticas defensivas/pasivas. |
| **PostgreSQL / Redis / Nginx** | `16.6-alpine3.21`, `7.4.2-alpine3.21`, `1.27.4-alpine3.21` | PostgreSQL / RSALv2-SSPLv1 / BSD-2 | Imágenes oficiales versionadas en `docker-compose.yml`. |

> **Nota de auditoría (FASE 0):** Resolver la discrepancia entre `LICENSE` (Apache-2.0) y las etiquetas de evaluación en `winget`/`scoop` antes de redistribuir binarios comerciales.

---

## 2. Generación Automatizada de SBOM y Escaneo CVE

Ejecutar `./scripts/sbom-scan.sh` para generar reportes CycloneDX (`syft`) y escaneo de vulnerabilidades `HIGH`/`CRITICAL` (`trivy`) sobre las imágenes fijadas.
