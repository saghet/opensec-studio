# TROUBLESHOOTING.md — Diagnóstico y Solución de Problemas

## 1. `ERR_CONNECTION_REFUSED` en `http://localhost:8088`
- **Causa 1 (Puerto ocupado en Windows):** El puerto `8080` suele estar ocupado en el host. `docker-compose.dev.yml` usa `18080:8080` para evitar colisión. Verifica con `docker compose ps`.
- **Causa 2 (Docker Desktop apagado o aún compilando):** Usa `scripts/launch-opensec.ps1`, que espera hasta que `/healthz` devuelva HTTP `200` antes de abrir el navegador.

## 2. Error `vault_write_error: Permission denied (os error 13)` al subir PCAP
- **Causa:** El volumen `opensec_pcap_vault` fue creado con propietario `root` en lugar de `uid 10001` (`opensec`).
- **Solución:**
  ```bash
  docker exec -u 0 opensec-api chown 10001:10001 /var/lib/opensec/pcaps
  docker exec -u 0 opensec-api chmod 755 /var/lib/opensec/pcaps
  ```

## 3. Error `host not found in upstream "opensec-api"` en Nginx
- **Causa:** Resolución estática al arrancar Nginx antes de que `opensec-api` esté en DNS.
- **Solución:** Ya resuelto en `docker/web/nginx.conf` mediante `resolver 127.0.0.11 valid=10s ipv6=off;`.

## 4. Sin adaptadores Wi-Fi detectados en Windows
- **Causa:** Docker Desktop en Windows ejecuta una VM Linux sin acceso directo a tarjetas Wi-Fi `nl80211` del host.
- **Solución:** En Windows utiliza **PCAP Lab (tshark)** subiendo archivos `.pcap`/`.pcapng`, o conecta un sensor **Kismet** desde un host Linux con adaptador USB/PCIe compatible (`mt76x2u`, `ath9k_htc`).

## 5. Recopilar Logs para Soporte
```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml ps -a
docker compose logs --tail 100 opensec-api opensec-web opensec-pcap-analyzer opensec-postgres
```
