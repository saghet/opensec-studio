#!/usr/bin/env bash
set -euo pipefail

BACKUP_DIR="${BACKUP_DIR:-./backups}"
STAMP="$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

source .env
echo "[OpenSec] Exportando PostgreSQL..."
docker compose exec -T opensec-postgres pg_dump -U "${POSTGRES_USER:-opensec_app}" -d "${POSTGRES_DB:-opensec}" -Fc > "$BACKUP_DIR/opensec_pg_${STAMP}.dump"

echo "[OpenSec] Respaldando bóveda PCAP..."
docker run --rm -v opensec-studio_opensec_pcap_vault:/src:ro -v "$(pwd)/$BACKUP_DIR":/dst alpine:3.21.3 \
  tar -czf "/dst/opensec_pcaps_${STAMP}.tar.gz" -C /src .

sha256sum "$BACKUP_DIR/opensec_pg_${STAMP}.dump" "$BACKUP_DIR/opensec_pcaps_${STAMP}.tar.gz" > "$BACKUP_DIR/SHA256SUMS_${STAMP}.txt"
echo "[OpenSec] Respaldo completado en $BACKUP_DIR"
