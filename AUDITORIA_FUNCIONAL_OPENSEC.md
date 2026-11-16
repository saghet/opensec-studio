# AUDITORIA_FUNCIONAL_OPENSEC.md — Auditoría Funcional Verificada (FASE 2)

**Fecha:** 2026-11-16  
**Método:** Inspección completa del repositorio + pruebas en vivo contra `http://localhost:8088`, `PostgreSQL`, `tshark` y `cargo test --workspace`.

---

## 1. Resumen Ejecutivo de Auditoría

1. **Módulos de aplicación web/DAST del producto original (`v1.10.10`):**
   - MITM Proxy, Repeater, Intruder, DAST, YAML Scanner, TCP Audit, JWT Inspector y CloudGraph **no tienen código fuente en este repositorio público** (solo existen en manuales y descripciones comerciales). Por tanto se clasifican como **E (No tiene implementación en este repo)**.
2. **Módulos de la Plataforma Wireless Security (implementados y verificados):**
   - **Control Plane (`opensec-api` + PostgreSQL):** Implementado y verificado en vivo (`GET /api/health`, `scopes`, `allowlist`, `sensors`, `networks`, `clients`, `alerts`, `assessments`, `risks`, `pcaps`, `audit-logs`, `kill-switch`).
   - **PCAP Forensics (`opensec-pcap-analyzer` + `tshark`):** Probado en vivo subiendo una trama IEEE 802.11 Beacon real (`POST /api/v1/pcaps/upload`), disecada por `tshark` y almacenada en PostgreSQL.
   - **Motores Rust (`opensec-core`):** `assessment.rs`, `wids.rs`, `rf.rs`, `risk.rs` probados con `cargo test --workspace` y ejecutados en vivo desde `opensec-api`.
   - **Reporting (`JSON`, `CSV`, `SARIF v2.1.0`, `HTML`):** Verificado en vivo contra datos reales de PostgreSQL.
   - **Dashboard FortiGate-style (`opensec-web`):** Verificado en vivo en `http://localhost:8088` con captura real (`docs/screenshots/dashboard_live.png`).
   - **Hardware Wi-Fi físico y Kismet en vivo:** Probados con suites unitarias simuladas (`hw_manager.py`, `kismet_adapter.py`); **pendiente de prueba con tarjeta Wi-Fi física en host Linux** (Docker Desktop en Windows no expone `nl80211`).
