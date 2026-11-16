# DOCKER_DEPLOYMENT.md — Guía Operativa de Despliegue Docker

## 1. Requisitos
- Docker Engine 24+ / Docker Desktop 4.x+ con Docker Compose v2+.
- Archivo `.env` copiado desde `.env.example` con contraseñas reales (sin `CHANGE_ME`).

## 2. Operaciones del Ciclo de Vida

- **Instalación / Arranque inicial (Desarrollo Windows/Linux):**
  ```bash
  cp .env.example .env
  docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build
  ```
- **Arranque en Producción (hardening `read_only`, `cap_drop: ALL`, sin puertos DB públicos):**
  ```bash
  ./scripts/install.sh prod
  ```
- **Arranque con Sensor Wi-Fi físico (solo Linux):**
  ```bash
  docker compose -f docker-compose.yml -f docker-compose.prod.yml --profile wireless-sensor up -d --build
  ```
- **Detención:**
  ```bash
  docker compose -f docker-compose.yml -f docker-compose.dev.yml down
  ```
- **Actualización segura (con backup previo):**
  ```bash
  ./scripts/update.sh prod
  ```
- **Backup y Restauración:**
  ```bash
  ./scripts/backup.sh
  ./scripts/restore.sh ./backups/opensec_pg_YYYYMMDD_HHMMSS.dump ./backups/opensec_pcaps_YYYYMMDD_HHMMSS.tar.gz
  ```
- **Logs y Diagnóstico:**
  ```bash
  docker compose ps
  docker compose logs -f opensec-api opensec-web opensec-pcap-analyzer
  ```
