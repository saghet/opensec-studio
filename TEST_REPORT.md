# TEST_REPORT.md — Reporte de Pruebas Verificadas

**Entorno:** Windows 11 (`c:\Users\saghe\Documents\opensec-studio`), Docker Desktop `4.99.0` (Engine `29.9.3`, Compose `v5.7.0`), `cargo`/`rustc`, `python`.

---

## 1. Pruebas Ejecutadas y Aprobadas

| Categoría | Comando / Endpoint | Resultado observado |
| :--- | :--- | :--- |
| **Compilación y tests Rust (`opensec-core` + `opensec-api`)** | `cargo test --workspace` | **7 passed, 0 failed** (6 en `opensec-core`, 1 en `opensec-api`). |
| **Hardware Compatibility Manager & Kismet Adapter** | `python -m unittest discover -s services/opensec-wireless-agent -v` | **4 passed, 0 failed**. |
| **PCAP Analyzer / `tshark` / Retención** | `python -m unittest discover -s services/opensec-pcap-analyzer -v` | **3 passed, 0 failed**. |
| **Validación Docker Compose (`dev` y `prod`)** | `docker compose -f docker-compose.yml -f docker-compose.{dev,prod}.yml config -q` | **Exit code 0** en ambos perfiles. |
| **Despliegue Docker en vivo** | `docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build` | Todos los servicios levantados y `healthy` (`opensec-postgres`, `opensec-redis`, `opensec-api`, `opensec-pcap-analyzer`, `opensec-web`, `opensec-worker`). |
| **API Health & PostgreSQL** | `GET http://localhost:8088/api/health` | **HTTP 200** (`db_connected: true`). |
| **Scope & Allowlist CRUD** | `POST /api/v1/scopes`, `POST /api/v1/allowlist` | **HTTP 201** persistido en PostgreSQL. |
| **Pipeline PCAP + `tshark` + Assessment + Risk** | `POST /api/v1/pcaps/upload?filename=test_beacon.pcap` | **HTTP 200**: `tshark` disecó Beacon real, generó `ASSESS-OPEN-001` (`proven_vulnerability`) y `risk_score: 9.12`. |
| **Exportación de Reportes (`SARIF`, `HTML`)** | `GET /api/v1/reports/export?format={sarif,html}` | **HTTP 200** con resultados reales desde PostgreSQL. |
| **Captura real de Interfaz Web** | `msedge.exe --headless=new --screenshot=... http://localhost:8088/` | Captura guardada en `docs/screenshots/dashboard_live.png` (`73,807` bytes). |

---

## 2. Pruebas Pendientes de Hardware / Entorno Linux

| Prueba | Motivo | Criterio de cierre |
| :--- | :--- | :--- |
| **Hardware Wi-Fi físico (`hw_manager.py` + `iw list`)** | Docker Desktop en Windows no expone `nl80211`. | Ejecutar en host Linux con adaptador USB/PCIe compatible (`mt76x2u` / `ath9k_htc`). |
| **Sensor Kismet en vivo (`--profile wireless-sensor`)** | Requiere interfaz en `monitor mode` en Linux. | Levantar perfil `wireless-sensor` en Linux y verificar ingesta real en `POST /api/v1/ingest/kismet`. |
