# WIDS_DETECTION_CATALOG.md — Catálogo de Detección WIDS y Evaluación (FASE 7)

**Implementación en Rust:** `services/opensec-worker/src/{wids.rs,assessment.rs,rf.rs,risk.rs}`

---

## 1. Reglas WIDS (Detección Defensiva Pasiva)

| Rule ID | Categoría | Criterio de correlación | Severidad | Confianza base |
| :--- | :--- | :--- | :--- | :--- |
| `WIDS-EVILTWIN-001` | `evil_twin_suspect` | SSID protegido + BSSID no autorizado + (`security_mismatch` y/o `channel_mismatch`). **Nunca** por SSID compartido aislado. | `critical` | `0.75 – 0.99` |
| `WIDS-ROGUE-002` | `unauthorized_bssid` | SSID de organización anunciado por BSSID no inventariado, sin indicadores secundarios de suplantación. | `medium` | `0.70` |
| `WIDS-DEAUTH-003` | `deauth_storm` | Observación pasiva de `>= 25` tramas `Deauthentication` en ventana de 300s. | `high` | `0.90` |
| `WIDS-BEACON-004` | `beacon_flood` | Ráfaga anómala de `>= 500` tramas `Beacon` en ventana de 300s por sensor. | `medium` | `0.80` |

Todas las alertas generan `dedup_key` por ventana temporal (`timestamp / 300`), almacenan `evidence[]`, `sensor_id`, `confidence` y `explanation`.

---

## 2. Reglas de Wireless Security Assessment (Regla 8)

Diferenciación estricta entre `ObservedConfig`, `InferredConfig`, `ProvenVulnerability` y `PotentialRisk`:

- `ASSESS-OPEN-001`: Red abierta sin cifrado (`ProvenVulnerability`).
- `ASSESS-WEP-002`: Cifrado WEP heredado (`ProvenVulnerability`, `critical`).
- `ASSESS-TRANSITION-003`: Modo mixto `WPA2/WPA3` con riesgo potencial de downgrade (`PotentialRisk`, `medium`).
- `ASSESS-ENT-004`: `WPA2-Enterprise` observado en beacon sin tramas EAP aún observadas (`InferredConfig`, `info`).
- `ASSESS-PMF-005`: `PMF / 802.11w` deshabilitado (`ObservedConfig`, `medium`).
- `ASSESS-WPS-006`: Elemento WPS anunciado en beacon (`PotentialRisk`, `medium`).
- `ASSESS-CHANGE-007`: Cambio de postura de seguridad respecto a línea base (`ObservedConfig`, `high`).

---

## 3. RF Analysis y Risk Engine (Reglas 10 y 11)

- **RF Analysis (`rf.rs`):** Distribución por bandas (`2.4GHz`, `5GHz`, `6GHz`), densidad por canal y detección de congestión en 2.4 GHz (`>= 4` APs por canal). Se documenta explícitamente que RSSI no equivale a calidad de enlace y que el análisis de espectro físico requiere hardware **SDR** (extensión futura).
- **Risk Engine (`risk.rs`):** Deduplica por `rule_id`, pondera por severidad × confianza × criticidad de red × factor de exposición (`0.0 – 100.0`) y devuelve explicación + mitigación prioritaria.
