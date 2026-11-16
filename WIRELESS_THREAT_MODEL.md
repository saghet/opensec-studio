# WIRELESS_THREAT_MODEL.md — Modelo de Amenazas Wi-Fi y Plataforma (FASE 7)

---

## 1. Amenazas al Entorno Inalámbrico Monitoreado

| Amenaza | Superficie | Detección en OpenSec (Pasiva) | Mitigación recomendada |
| :--- | :--- | :--- | :--- |
| **Evil Twin / Rogue AP** | Capa 2 IEEE 802.11 | `WIDS-EVILTWIN-001` (correlación BSSID + seguridad + canal) | 802.1X/EAP-TLS con validación estricta de certificado RADIUS; inventario de BSSIDs. |
| **Deauthentication / Disassoc Storm** | Tramas de gestión sin cifrar | `WIDS-DEAUTH-003` + analizador `tshark` | Habilitar PMF (`802.11w` Required) y migrar a WPA3-SAE. |
| **Downgrade WPA2/WPA3 Transition** | Negociación RSN | `ASSESS-TRANSITION-003` | Migrar a WPA3-SAE puro en SSIDs compatibles. |
| **Protocolos heredados (WEP / Open / WPS)** | Configuración AP | `ASSESS-OPEN-001`, `ASSESS-WEP-002`, `ASSESS-WPS-006` | Deshabilitar WEP/WPS; exigir WPA2-AES o WPA3. |

## 2. Amenazas a la Propia Plataforma OpenSec

- **Exposición de datos sensibles en PCAPs:** Mitigada mediante bóveda aislada (`opensec_pcap_vault`), purga automática por retención (`purge_expired_pcaps`) y validación anti-*path traversal*.
- **Uso no autorizado fuera de Scope:** Mitigada con `authorization_scopes`, `kill_switch_active` y modo `passive_only` obligatorio por defecto.
- **Compromiso de contenedor analizador/sensor:** Mitigada con `cap_drop: ALL`, `read_only: true`, redes internas aisladas (`opensec_internal`) y separación de privilegios.
