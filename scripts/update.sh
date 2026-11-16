#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-prod}"
echo "[OpenSec] Creando respaldo previo a la actualización..."
./scripts/backup.sh

echo "[OpenSec] Reconstruyendo imágenes y aplicando actualización..."
if [ "$MODE" = "dev" ]; then
  docker compose -f docker-compose.yml -f docker-compose.dev.yml pull --ignore-buildable
  docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build --remove-orphans
else
  docker compose -f docker-compose.yml -f docker-compose.prod.yml pull --ignore-buildable
  docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d --build --remove-orphans
fi

docker compose ps
