# MATRIZ_FUNCIONALIDADES.md — Clasificación de Módulos (FASE 2)

**Escala:** `A` Implementado y funciona | `B` Implementado parcialmente | `C` UI sin backend | `D` Simulado | `E` Sin implementación | `F` Con errores | `G` Funciona, necesita mejoras.

| Módulo | Clasificación | Evidencia / Verificación | Observaciones |
| :--- | :---: | :--- | :--- |
| **MITM Proxy** | **E** | Ausente en `git ls-files` | Solo referenciado en `manuales/` y `README*.md`. |
| **Repeater / Intruder** | **E** | Ausente en `git ls-files` | Código fuente Tauri no está en el repo público. |
| **DAST / YAML Scanner** | **E** | Ausente en `git ls-files` | Código fuente Tauri no está en el repo público. |
| **TCP Audit / JWT Inspector** | **E** | Ausente en `git ls-files` | Código fuente Tauri no está en el repo público. |
| **CloudGraph CNAPP** | **E** | Ausente en `git ls-files` | Documentado en Cap. 19; sin código fuente aquí. |
| **SARIF Export (Wireless)** | **A** | `GET /api/v1/reports/export?format=sarif` probado | Genera SARIF v2.1.0 real desde `security_assessments` en PostgreSQL. |
| **Policy & Scope / Allowlist** | **A** | `POST /api/v1/scopes`, `POST /api/v1/allowlist` probados | Persiste en PostgreSQL y actualiza `is_authorized` en redes. |
| **PCAP Forensics (`tshark`)** | **A** | `POST /api/v1/pcaps/upload` probado con Beacon real | Diseca con `tshark`, detecta redes/anomalías y purga por retención. |
| **Posture Assessment (WPA2/3)** | **A** | `cargo test` + ejecución en vivo desde PCAP | Diferencia `Observed`, `Inferred`, `Proven`, `Potential`. |
| **WIDS Engine** | **A** | `cargo test` + persistencia en `wids_alerts` | Correlación multi-indicador + deduplicación + `Ack/Resolve`. |
| **RF Analysis & Risk Scoring** | **A** | `cargo test` + `GET /api/v1/dashboard/summary` / `risks` | Distribución de bandas/canales y score `0–100` deduplicado. |
| **Dashboard Wireless Center** | **G** | Captura real en `http://localhost:8088` | Funcional; pendiente paginación avanzada y login JWT en UI. |
| **Kismet & Hardware Manager** | **B** | 4 unit tests OK (`hw_manager.py`, `kismet_adapter.py`) | Lógica lista; pendiente prueba con tarjeta Wi-Fi física en Linux. |
| **Autenticación / RBAC API** | **B** | Tabla `users` y `OPENSEC_JWT_SECRET` definidos | Falta endpoint `/api/v1/auth/login` y middleware JWT en `opensec-api`. |
