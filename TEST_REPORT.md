# TEST_REPORT.md — Reporte de Pruebas Verificadas (FASE 9)

**Entorno de ejecución:** Windows 11 (`c:\Users\saghe\Documents\opensec-studio`), Docker Desktop `4.99.0` (Engine `29.9.3`, Compose `v5.7.0`), `rustc`/`cargo`, `python`.

---

## 1. Pruebas Ejecutadas y Aprobadas

| Categoría (Regla 15) | Comando ejecutado | Resultado observado |
| :--- | :--- | :--- |
| **Compilación y tests Rust (`opensec-api`)** | `cargo test --workspace` | **2 passed, 0 failed** (`summary_returns_zeroes_never_fake_metrics`, `kill_switch_toggles_cleanly`). |
| **Compilación y tests Rust (`opensec-worker` — Assessment, WIDS, RF, Risk)** | `cargo test --workspace` | **6 passed, 0 failed** (`distinguishes_observed_potential_and_proven_findings`, `enterprise_without_eap_frames_is_inferred_not_vulnerable`, `shared_ssid_alone_does_not_classify_as_evil_twin`, `multi_indicator_triggers_evil_twin_suspect`, `computes_band_and_channel_congestion`, `deduplicates_identical_rule_ids_and_caps_at_100`). |
| **Hardware Compatibility Manager (simulado)** | `python -m unittest discover -s services/opensec-wireless-agent -v` | **1 passed** (`test_probe_adapters_and_limitations`). |
| **Adaptador Kismet y Kill Switch (simulado)** | `python -m unittest discover -s services/opensec-wireless-agent -v` | **3 passed** (`test_normalize_device`, `test_normalize_alert`, `test_kill_switch_blocks_polling`). |
| **PCAP Analyzer / tshark wrapper / Retención** | `python -m unittest discover -s services/opensec-pcap-analyzer -v` | **3 passed** (`test_path_traversal_blocked`, `test_parse_tshark_tsv_and_deauth_anomaly`, `test_analyze_and_retention_purge`). |
| **Validación Docker Compose (`dev` y `prod`)** | `docker compose -f docker-compose.yml -f docker-compose.{dev,prod}.yml config -q` | **Exit code 0** en ambos perfiles. |

---

## 2. Pruebas Pendientes de Hardware / Entorno Linux

| Prueba | Motivo | Criterio de cierre |
| :--- | :--- | :--- |
| **Hardware Wi-Fi real (`hw_manager.py` + `iw list`)** | Ejecutado en host Windows sin acceso directo `nl80211`. | Ejecutar en host Linux con adaptador USB/PCIe compatible (`mt76x2u` / `ath9k_htc`). |
| **Sensor Kismet en vivo (`--profile wireless-sensor`)** | Requiere interfaz en `monitor mode` en Linux. | Levantar perfil `wireless-sensor` en Linux y verificar ingesta real en `opensec-api`. |
| **End-to-End Docker (`up -d` + `pg_dump`/`pg_restore`)** | Pendiente de despliegue en servidor Linux objetivo. | Ejecutar `./scripts/install.sh prod`, `./scripts/backup.sh` y `./scripts/restore.sh`. |
