# FUNCIONALIDADES_PENDIENTES.md — Brechas y Pendientes Verificados

1. **Código fuente original de escritorio (MITM, DAST, Repeater, Intruder, TCP Audit, JWT, CloudGraph):**
   - No existe en el repositorio público `saghet/opensec-studio`. Si se desea auditar o migrar a Docker, debe aportarse el repositorio fuente privado.
2. **Autenticación JWT / Login de usuarios en `opensec-api`:**
   - La tabla `users` (con `role`: `admin`, `analyst`, `auditor`, `viewer`) ya existe en PostgreSQL; falta implementar `POST /api/v1/auth/login` y validación de token Bearer en `opensec-api` y pantalla de login en `opensec-web`.
3. **Pruebas de hardware Wi-Fi físico (`--profile wireless-sensor`):**
   - Pendientes de ejecución en un host Linux con adaptador compatible (`mt76x2u` / `ath9k_htc`) para validar `monitor mode`, `kismet` en vivo y streaming continuo hacia `POST /api/v1/ingest/kismet`.
4. **Polling automático desde `opensec-worker` hacia Kismet/Redis:**
   - Actualmente `opensec-worker` ejecuta ciclos RF sobre PostgreSQL; el loop automático de consumo de `Redis Streams` para sensores remotos de alta frecuencia queda preparado para la siguiente iteración.
5. **Empaquetado Linux Nativo (`.deb`, `.rpm`, `AppImage`):**
   - Documentado en `LINUX_NATIVE_ROADMAP.md`; pendiente construir los paquetes en entorno Linux.
