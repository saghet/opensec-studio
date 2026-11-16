#!/usr/bin/env bash
set -euo pipefail

if [ ! -f .env ]; then
  cp .env.example .env
  echo "[OpenSec] .env creado desde .env.example. Reemplaza todos los valores CHANGE_ME antes de continuar."
  exit 1
fi

if grep -q "CHANGE_ME" .env; then
  echo "[ERROR] .env aún contiene secretos de ejemplo (CHANGE_ME). Actualízalos antes de desplegar."
  exit 1
fi

MODE="${1:-prod}"
if [ "$MODE" = "dev" ]; then
  docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build
else
  docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d --build
fi

echo "[OpenSec] Despliegue ($MODE) iniciado. Verifica estado con: docker compose ps"
