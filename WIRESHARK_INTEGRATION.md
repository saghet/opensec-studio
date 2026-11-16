# WIRESHARK_INTEGRATION.md — Integración de Wireshark / tshark (FASE 6)

**Proyecto:** OpenSec Studio — Wireless Security Platform  
**Motor:** Wireshark (`wireshark/wireshark`, versión observada `wireshark-4.4.9`, C, licencia **GPL-2.0**).

---

## 1. Evaluación Técnica de Componentes

| Componente | Rol en OpenSec | Decisión de arquitectura |
| :--- | :--- | :--- |
| **`tshark`** | Motor de disección offline sobre capturas PCAP/PCAPNG (`-T fields` / `-T json`). | **Adoptado** dentro del contenedor aislado `opensec-pcap-analyzer` (frontera GPL-2.0). |
| **`capinfos`** | Extracción rápida de metadatos (número de paquetes, duración, tamaño, encapsulación). | **Adoptado** como paso previo al análisis profundo. |
| **`dumpcap`** | Binario mínimo de captura con privilegios reducidos (`CAP_NET_RAW`/`CAP_NET_ADMIN`). | **Evaluado para FASE 10 (Linux nativo)** como alternativa ligera cuando no se requiera Kismet completo. |
| **`libwireshark`** | Biblioteca C de dissectors. | **No enlazada en Rust** (evita acoplamiento ABI C y contaminación GPL-2.0 en el binario principal). |

---

## 2. Formatos y Dissectors IEEE 802.11 / Radiotap

- **PCAP vs PCAPNG:** Se prioriza **PCAPNG** por soportar múltiples interfaces, comentarios de captura, estadísticas de paquetes perdidos y metadatos de sensor; se mantiene compatibilidad de lectura con **PCAP** clásico.
- **Campos Radiotap y 802.11 extraídos por `tshark`:**
  - `frame.time_epoch`, `frame.protocols`
  - `radiotap.channel.freq`, `radiotap.dbm_antsignal`
  - `wlan.fc.type`, `wlan.fc.type_subtype` (Beacons `0x08`, Probe Req `0x04`, Probe Resp `0x05`, Deauth `0x0c`, Disassoc `0x0a`)
  - `wlan.bssid`, `wlan.sa`, `wlan.da`, `wlan.ssid`
  - `wlan.rsn.akms.type` / `eapol` (detección pasiva de cifrado y handshakes sin extraer credenciales)

---

## 3. Capacidades del Módulo (`services/opensec-pcap-analyzer/app.py`)

1. **Validación estricta de ruta y autorización:** Solo procesa archivos `.pcap` / `.pcapng` ubicados dentro del volumen autorizado (`OPENSEC_PCAP_VAULT`), bloqueando *path traversal*.
2. **Metadatos e inventario:** Calcula SHA-256, conteo de tramas, BSSIDs/SSIDs observados, desglose de protocolos y canales.
3. **Detección de anomalías en captura:**
   - Tasa elevada de tramas `Deauthentication` / `Disassociation` (`deauth_spike_in_capture`).
   - Presencia de tramas con cifrado heredado (`WEP` / sin RSN).
4. **Informes técnicos y gerenciales:** Genera resumen técnico (indicadores, BSSIDs, subtipos 802.11) y resumen ejecutivo para dirección/auditoría.
5. **Política de retención (Regla 7):** `purge_expired_pcaps()` elimina automáticamente capturas cuyo `mtime` supere `OPENSEC_PCAP_RETENTION_DAYS` (por defecto 7 días), conservando únicamente metadatos y hallazgos en PostgreSQL.
