#!/usr/bin/env bash
set -euo pipefail

DUMP_FILE="${1:?Uso: $0 <ruta_dump_pg.dump> [ruta_pcaps.tar.gz]}"
PCAP_TAR="${2:-}"

source .env
echo "[OpenSec] Deteniendo servicios de aplicación antes de restaurar..."
docker compose stop opensec-api opensec-worker

echo "[OpenSec] Restaurando base de datos PostgreSQL desde $DUMP_FILE..."
cat "$DUMP_FILE" | docker compose exec -T opensec-postgres pg_restore -U "${POSTGRES_USER:-opensec_app}" -d "${POSTGRES_DB:-opensec}" --clean --if-exists

if [ -n "$PCAP_TAR" ] && [ -f "$PCAP_TAR" ]; then
  echo "[OpenSec] Restaurando bóveda PCAP desde $PCAP_TAR..."
  docker run --rm -v opensec-studio_opensec_pcap_vault:/dst -v "$(realpath "$PCAP_TAR")":/backup.tar.gz:ro alpine:3.21.3 \
    sh -c "rm -rf /dst/* && tar -xzf /backup.tar.gz -C /dst"
fi

echo "[OpenSec] Reiniciando servicios..."
docker compose start opensec-api opensec-worker
