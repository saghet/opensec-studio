# HARDWARE_COMPATIBILITY.md — Hardware Wi-Fi, Kernel Linux y Compatibilidad (FASE 3)

**Proyecto:** OpenSec Studio — Wireless Security Platform  
**Componente:** Wireless Hardware Compatibility Manager (`opensec-wireless-agent`)

---

## 1. Fundamentos Técnicos en Linux (`nl80211` / `cfg80211`)

1. **Managed Mode vs Monitor Mode:**
   - **Managed (`station`):** El kernel/firmware filtra tramas por BSSID/destino y entrega paquetes Ethernet 802.3 al stack IP. No expone tramas de gestión (Beacons, Probes, Auth/Deauth) de redes ajenas.
   - **Monitor (`monitor`):** Desactiva el filtrado de BSSID en la interfaz virtual (`iw dev <iface> set type monitor`) y entrega tramas IEEE 802.11 crudas precedidas por cabeceras **Radiotap** (RSSI, canal, frecuencia, tasa, flags FCS, antena).
   - **No todos los adaptadores soportan monitor mode:** Algunos chipsets/firmwares lo bloquean o lo limitan a un único canal o sin Radiotap completo. El `Wireless Hardware Compatibility Manager` verifica explícitamente `Supported interface modes` vía `iw list` (`nl80211`) antes de habilitar un sensor.
2. **Bandas, Canales y Dominios Regulatorios:**
   - **2.4 GHz (Canales 1–14):** Canal 12–13 restringidos en FCC (US), canal 14 solo Japón (802.11b).
   - **5 GHz (UNII-1/2/2e/3):** Canales DFS (52–144) sujetos a detección de radar y restricciones regulatorias.
   - **6 GHz (Wi-Fi 6E / 7):** Requiere hardware compatible, kernel moderno con soporte `6 GHz` en `cfg80211`, `regulatory.db` actualizado y país habilitado (`iw reg get`).
3. **USB vs PCIe:**
   - **USB:** Portátil y fácil de asignar a VMs/contenedores, pero sensible a caídas de potencia en puertos USB 2.0/3.0 y a interferencia USB 3.0 en 2.4 GHz si no hay blindaje.
   - **PCIe / M.2:** Menor latencia y mejor estabilidad DMA/antenas, aunque algunos chipsets integrados (p. ej. Intel `iwlwifi`) están diseñados para modo cliente y presentan limitaciones en captura pasiva multi-canal.

---

## 2. Matriz de Compatibilidad Verificable

> **Criterio:** La compatibilidad real siempre se verifica en tiempo de ejecución mediante `hw_manager.py` (`iw list`, `ethtool -i`, `/sys/class/net/<iface>/device`). Esta matriz documenta comportamientos conocidos de referencia en kernel Linux, sin asumir soporte universal.

| Familia / Chipset | Bus | Driver Linux | Firmware | Bandas | Monitor Mode | Limitaciones conocidas |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Atheros AR9271** | USB | `ath9k_htc` (in-kernel) | `ath9k_htc/htc_9271-*.fw` (`linux-firmware`) | 2.4 GHz | Completo | Solo 2.4 GHz (802.11n); sin 5/6 GHz. |
| **MediaTek MT7612U / MT7610U** | USB | `mt76x2u` / `mt76x0u` (in-kernel) | `mt7662u*.bin` (`linux-firmware`) | 2.4 / 5 GHz | Completo | Requiere kernel >= 4.19 para soporte estable in-tree. |
| **MediaTek MT7921AU / MT7921U** | USB | `mt7921u` (in-kernel) | `mediatek/WIFI_MT7961_*.bin` | 2.4 / 5 / 6 GHz (según SKU) | Soportado en kernels recientes | 6 GHz depende de dominio regulatorio y versión de `linux-firmware`. |
| **Qualcomm Atheros AR93xx/AR94xx** | PCIe | `ath9k` (in-kernel) | Sin blob propietario externo | 2.4 / 5 GHz (según tarjeta) | Completo | Hardware legado (802.11n). |
| **Realtek RTL8812AU / RTL8814AU** | USB | Out-of-tree (`88XXau` / `rtw88`) | Integrado / según driver | 2.4 / 5 GHz | Depende del driver | Requiere DKMS/módulo externo; incompatibilidades frecuentes tras actualizaciones de kernel. |
| **Intel AX200 / AX210 / BE200** | PCIe/M.2 | `iwlwifi` (in-kernel) | `iwlwifi-*.ucode` | 2.4 / 5 / 6 GHz | **Limitado / No recomendado para sensor** | Firmware de cliente; problemas conocidos en captura monitor estable o canales DFS. |
| **Broadcom (Raspberry Pi onboard)** | SDIO/PCIe | `brcmfmac` | `brcm/brcmfmac*.bin` | 2.4 / 5 GHz | **No nativo sin parche (nexmon)** | Firmware propietario no expone monitor mode estándar in-tree. |

---

## 3. Acceso al Hardware desde Docker vs Linux Nativo

- **Docker en Linux (`--profile wireless-sensor`):**
  - Requiere `network_mode: host` y `cap_add: [NET_ADMIN, NET_RAW]` para interactuar con `nl80211` y crear/leer interfaces monitor en el namespace de red del host.
  - **En Docker Desktop (Windows / macOS):** Los contenedores corren dentro de una VM Linux sin acceso directo a tarjetas Wi-Fi PCIe/USB del host sin passthrough USB explícito. En esos entornos, el sensor de hardware reporta `no_wireless_adapters_detected` sin fallar.
- **Linux Nativo (`systemd`, FASE 10):**
  - El agente corre directamente en el host con `AmbientCapabilities=CAP_NET_ADMIN CAP_NET_RAW`, eliminando la capa de contenedorización.

---

## 4. Especificación del `Wireless Hardware Compatibility Manager`

Implementado en [services/opensec-wireless-agent/hw_manager.py](file:///c:/Users/saghe/Documents/opensec-studio/services/opensec-wireless-agent/hw_manager.py). Detecta y reporta en JSON estructurado:

- **Identificación:** nombre de interfaz, MAC, fabricante/PCI-USB IDs (`/sys/class/net/<iface>/device/uevent`).
- **Stack kernel:** `driver`, `firmware_version` (`ethtool -i`).
- **Capacidades `nl80211` (`iw list`):** `supports_monitor_mode`, `supported_bands` (`2.4GHz`, `5GHz`, `6GHz`), `available_channels` (excluyendo canales `disabled`).
- **Regulación (`iw reg get`):** país regulatorio activo (`country XX`).
- **Limitaciones detectadas:** advertencias explícitas si falta `monitor mode`, si `driver == iwlwifi` o `brcmfmac`, o si no se detecta `firmware`.
