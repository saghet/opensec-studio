#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-./sbom-reports}"
mkdir -p "$OUT_DIR"

IMAGES=(
  "postgres:16.6-alpine3.21"
  "redis:7.4.2-alpine3.21"
  "nginxinc/nginx-unprivileged:1.27.4-alpine3.21"
)

for img in "${IMAGES[@]}"; do
  safe_name="$(echo "$img" | tr '/:' '__')"
  echo "[SBOM] Generando CycloneDX para $img..."
  if command -v syft >/dev/null 2>&1; then
    syft "$img" -o cyclonedx-json > "$OUT_DIR/${safe_name}.cdx.json"
  fi
  if command -v trivy >/dev/null 2>&1; then
    trivy image --severity HIGH,CRITICAL --exit-code 0 "$img" > "$OUT_DIR/${safe_name}.trivy.txt"
  fi
done

echo "[SBOM] Reportes generados en $OUT_DIR"
