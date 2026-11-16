# KISMET_INTEGRATION.md — Integración de Kismet (FASE 4)

**Proyecto:** OpenSec Studio — Wireless Security Platform  
**Motor:** Kismet (`kismetwireless/kismet`, último release observado: `kismet-2026-10-R1`, C++, licencia GPL-2.0).

---

## 1. Estrategia de Integración y Aislamiento de Licencia

- Kismet se ejecuta como servicio/contenedor independiente (`opensec-kismet` en Docker o unidad `kismet.service` en Linux nativo).
- OpenSec **no duplica** la captura 802.11, el *channel hopping* ni el mantenimiento de tablas OUI de Kismet.
- Toda comunicación ocurre por **HTTP REST / WebSocket** (puerto `2501` en `127.0.0.1`), manteniendo una frontera de proceso limpia frente a GPL-2.0.

---

## 2. Endpoints de la API de Kismet Utilizados

| Capacidad OpenSec | Endpoint Kismet | Método | Propósito |
| :--- | :--- | :--- | :--- |
| **Health / Estado del sensor** | `/system/status.json` | `GET` | Verificar disponibilidad, memoria, paquetes/seg y estado de fuentes. |
| **Fuentes de captura** | `/datasource/all_sources.json` | `GET` | Confirmar interfaz en `monitor mode`, canales activos y errores de hardware. |
| **Inventario y cambios incrementales** | `/devices/last-time/{ts}/devices.json` | `GET` / `POST` (field filter) | Obtener únicamente dispositivos nuevos o actualizados desde el último polling (`{ts}`). |
| **Alertas nativas de Kismet** | `/alerts/last-time/{ts}/alerts.json` | `GET` | Ingestar alertas nativas (deauth flood, beacon anomalies, etc.) hacia el motor WIDS. |
| **Flujo en tiempo real (opcional)** | `/eventbus/events.ws` | `WebSocket` | Suscripción push a eventos `NEW_DEVICE` y `ALERT`. |

**Autenticación:** Se utiliza `KISMET_API_TOKEN` (header `KISMET: <token>` o cookie de sesión) o credenciales dedicadas `KISMET_USER`/`KISMET_PASSWORD` configuradas en `.env` (nunca en código).

---

## 3. Mapeo de Datos Kismet → Modelo Canónico OpenSec (`DATA_MODEL.md`)

| Campo Kismet JSON | Campo OpenSec (`wireless_networks` / `observations`) | Regla de normalización |
| :--- | :--- | :--- |
| `kismet.device.base.macaddr` | `bssid` | Normalizado en minúsculas `aa:bb:cc:dd:ee:ff`. |
| `dot11.device.last_beaconed_ssid` / `advertised_ssid_map[].ssid` | `ssid` | Cadena UTF-8 saneada (vacío si es SSID oculto). |
| `kismet.device.base.manuf` | `oui_vendor` | Fabricante OUI resuelto por Kismet. |
| `kismet.device.base.frequency` | `band` | `2400..2499 MHz` → `2.4GHz`; `5000..5899 MHz` → `5GHz`; `5925..7125 MHz` → `6GHz`. |
| `kismet.device.base.channel` | `channel` | Entero IEEE 802.11. |
| `kismet.common.signal.last_signal` | `rssi_dbm` | Entero en dBm. |
| `dot11.advertisedssid.crypt_string` | `security_observed` | Clasificado en `WPA3-SAE`, `WPA2/WPA3`, `WPA2-Enterprise`, `WPA2-PSK`, `WPA`, `WEP`, `Open`. |
| `dot11.advertisedssid.wps_state` | `wps_observed` | Booleano (`true` si WPS anunciado en IE). |
| `dot11.device.associated_client_map` | `wireless_clients` | Lista de MACs de clientes asociados al BSSID. |

---

## 4. Controles de Seguridad en el Adaptador

1. **ScopeGuard / Kill Switch:** Antes de cada ciclo de polling, `kismet_adapter.py` verifica el estado del Kill Switch. Si está activo, detiene la ingesta inmediatamente.
2. **Solo observación pasiva:** El adaptador únicamente realiza peticiones `GET` de telemetría pasiva; no invoca acciones destructivas ni de inyección.
3. **Resiliencia:** Si Kismet no está accesible (por ejemplo, en entornos sin hardware Wi-Fi), el adaptador reporta `sensor_unreachable` sin bloquear el Control Plane.
