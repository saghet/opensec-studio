# CHANGELOG.md — Historial de Cambios

## [Wireless Platform v0.1.0] - 2026-11-16
### Added
- Plataforma Docker multi-servicio (`opensec-postgres`, `opensec-redis`, `opensec-api`, `opensec-worker`, `opensec-pcap-analyzer`, `opensec-web`, perfil `wireless-sensor` con `opensec-kismet` y `opensec-wireless-agent`).
- Crate compartido `crates/opensec-core` con motores `assessment`, `wids`, `rf` y `risk`.
- Persistencia real en PostgreSQL para Scopes, Allowlist, Sensores, Redes, Clientes, Alertas WIDS, Assessments, Risk Scores, Capturas PCAP y Audit Logs.
- Analizador PCAP/PCAPNG con `tshark`, detección de anomalías deauth/disassoc, decodificación hex de SSIDs y purga automática por retención.
- Dashboard **Wireless Security Center** organizado por menús estilo FortiGate y exportación en **HTML, JSON, CSV y SARIF v2.1.0**.
- Scripts operativos (`install.sh`, `update.sh`, `backup.sh`, `restore.sh`, `sbom-scan.sh`, `launch-opensec.ps1`) y documentación técnica/usuario completa.

## [v1.10.10] - 2026-10-22
- Actualización de manifiestos (`winget`, `scoop`, `chocolatey`), checksums SHA-256 y manuales multilingües hasta Capítulo 19 (CloudGraph CNAPP).
