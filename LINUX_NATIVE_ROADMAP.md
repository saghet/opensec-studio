# LINUX_NATIVE_ROADMAP.md — Hoja de Ruta hacia Distribución Linux Nativa (FASE 10)

---

## 1. Arquitectura Sin Dependencia de Docker (Regla 14)

Los motores implementados (`opensec-api`, `opensec-worker`, `opensec-pcap-analyzer`, `opensec-wireless-agent`) leen exclusivamente variables de entorno estándar (`DATABASE_URL`, `REDIS_URL`, `OPENSEC_PCAP_VAULT`, `OPENSEC_DEFAULT_MODE`) y se comunican por HTTP/TCP, por lo que pueden ejecutarse directamente sobre el host Linux mediante **systemd** sin modificar la lógica de negocio.

---

## 2. Servicios `systemd` Previstos

| Unidad | Usuario | Privilegios / Capabilities |
| :--- | :--- | :--- |
| `opensec-api.service` | `opensec` (sin privilegios) | Ninguna capability especial; escucha en `127.0.0.1:8080` detrás de Nginx/Caddy. |
| `opensec-worker.service` | `opensec` (sin privilegios) | Ninguna capability especial. |
| `opensec-pcap-analyzer.service` | `opensec-pcap` | Acceso de solo lectura a `/var/lib/opensec/pcaps` (invoca `tshark`). |
| `opensec-wireless-agent.service` | `opensec-sensor` | `AmbientCapabilities=CAP_NET_ADMIN CAP_NET_RAW`, `NoNewPrivileges=true`. |
| `kismet.service` | `kismet` | Grupo `kismet` / `CAP_NET_ADMIN CAP_NET_RAW` sobre interfaz monitor. |

---

## 3. Empaquetado y Distribución

1. **`.deb` (Debian / Ubuntu):** Paquete `opensec-studio` (Control Plane + Worker + UI) y `opensec-sensor` (agente + dependencias `iw`, `ethtool`, `tshark`, `kismet`).
2. **`.rpm` (RHEL / Fedora / Rocky):** Spec equivalente con unidades `systemd` en `/usr/lib/systemd/system/`.
3. **AppImage (UI / cliente de escritorio):** Compatible para la interfaz Tauri/Web de escritorio; los daemons privilegiados (`kismet`, `opensec-wireless-agent`) deben permanecer como servicios `systemd` del sistema.
4. **Instalador automatizado:** Script `install-linux.sh` que verifica kernel (`nl80211`), grupo `kismet`, configuración `/etc/opensec/opensec.env` y habilita las unidades `systemd`.
