# AUDIT_OPENSEC.md — Auditoría Inicial del Proyecto (FASE 0)

**Fecha de auditoría:** 2026-11-16  
**Repositorio auditado:** `https://github.com/saghet/opensec-studio`  
**Workspace local:** `c:\Users\saghe\Documents\opensec-studio`  
**Alcance:** Regla 1 (Auditoría obligatoria del proyecto existente) y Fase 0 del Plan de Ejecución.

---

## 1. Estado del Repositorio Remoto y Local

| Atributo | Valor observado |
| :--- | :--- |
| **Remoto (`origin`)** | `https://github.com/saghet/opensec-studio` |
| **Rama principal (`main` / `origin/main`)** | `c0e081e` (`fix(release): align manifests, landing links, chapter counts and v1.10.10 SHA256 checksums`) |
| **Commit previo en `main`** | `ae2bab2d14414e9f6fc0f615be66921e28f1dd83` (`docs: update official multilingual illustrated manuals with Chapter 19 (CloudGraph CNAPP)`) |
| **Tag publicado** | `v1.10.10` (apunta al commit `f8a4883e74f49a050e31074cdc2fe8f4490080a7`) |
| **Sincronización local vs remoto** | Sincronizado (`0` commits de diferencia entre `main` y `origin/main` tras el push de `c0e081e`). |
| **Release en GitHub (`v1.10.10`)** | Publicado el `2026-10-22T14:18Z` con `Instalador_OpenSec_Studio_Setup.exe` (1,905,054 bytes), `Instalador_Empresarial_OpenSec_Studio.msi` (2,646,016 bytes), 7 manuales PDF y `SHA256SUMS.txt`. |

### Nota sobre el contenido del repositorio público

El repositorio `saghet/opensec-studio` **no contiene el código fuente de la aplicación Rust/Tauri**. Funciona exclusivamente como repositorio público de distribución, documentación, landing page, marketing y manifiestos de gestores de paquetes para Windows.

---

## 2. Inventario Completo de Archivos Rastreados (33 archivos)

| Categoría | Archivos | Estado |
| :--- | :--- | :--- |
| **Documentación raíz** | `README.md`, `README.es.md`, `README.de.md`, `README.fr.md`, `README.it.md`, `README.ja.md` | Actualizados en `c0e081e` (checksums `v1.10.10` y 19 capítulos). |
| **Licencia y Git** | `LICENSE`, `.gitignore` | `LICENSE` declara **Apache License 2.0** (`Copyright 2026 OpenSec Studio Authors`). |
| **Landing / Web estática** | `docs/index.html`, `landing-page/index.html` | Archivos idénticos (duplicados) en HTML/CSS/JS estático. |
| **Manuales de usuario** | `manuales/manual_{de,en,es,fr,it,ja}.html` y 7 PDFs (`Manual_OpenSec_Studio_*.pdf`) | Incluyen hasta el Capítulo 19 (CloudGraph CNAPP). |
| **Manifiestos Windows** | `winget/el-saghet.OpenSecStudio.yaml`, `scoop/opensec-studio.json`, `chocolatey/opensec-studio.nuspec` | Alineados con los artefactos de `v1.10.10`. |
| **Marketing** | 7 archivos en `marketing/*.md` | Plantillas de lanzamiento y difusión. |

---

## 3. Auditoría de Arquitectura, Módulos, Pruebas y Compilación

Se inspeccionó el árbol completo de archivos rastreados (`git ls-files`) en busca de `Cargo.toml`, `package.json`, `src-tauri/`, archivos `.rs`, `.ts`, `.js` de aplicación o suites de pruebas. **No se encontró código fuente compilable ni pruebas automatizadas.**

### Clasificación obligatoria de módulos (Regla 1)

| Componente / Módulo | Clasificación | Evidencia verificada |
| :--- | :--- | :--- |
| **Arquitectura Rust / Tauri** | **No implementado** *(ausente en el repo)* | No existen `Cargo.toml`, `src-tauri/`, `tauri.conf.json` ni `package.json`. |
| **Frontend de aplicación** | **No implementado** *(ausente en el repo)* | Solo existe landing estática (`docs/index.html` y `landing-page/index.html`). |
| **Backend / API / Servicios** | **No implementado** *(ausente en el repo)* | Sin código de servidor, base de datos ni workers. |
| **MITM Proxy** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **Repeater** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **Intruder** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **DAST** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **YAML Scanner (Nuclei)** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **TCP Audit** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **JWT Inspector** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en documentación y manuales. |
| **CloudGraph / CNAPP** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en commits/manuales (Cap. 19). |
| **SARIF Export** | **No implementado** *(ausente en el repo)* | Referenciado únicamente en mensajes de commit (`fbe8dd8`). |
| **Pruebas automatizadas** | **No implementado** *(ausente en el repo)* | No hay tests unitarios, de integración ni E2E. |
| **Compilación** | **No verificable** *(sin proyecto Rust/Node)* | No existe proyecto compilable en este directorio. |
| **Landing y distribución Windows** | **Implementado parcialmente** | `docs/index.html`, `winget`, `scoop`, `chocolatey`, `README*.md` validados sintácticamente tras `c0e081e`. |

---

## 4. Hallazgos: Duplicados, Inconsistencias y Riesgos Detectados

1. **Ausencia de código fuente en el repositorio (Bloqueante para auditoría de código existente):**
   - Los binarios `.exe` y `.msi` publicados en GitHub Releases `v1.10.10` corresponden a una aplicación compilada cuyo código fuente no está alojado en `saghet/opensec-studio`.
2. **Duplicación de la landing page:**
   - `docs/index.html` y `landing-page/index.html` son copias exactas entre sí. Cualquier cambio futuro debe mantenerse sincronizado o unificarse en una sola ruta.
3. **Inconsistencia legal / de licenciamiento (Regla 16):**
   - El archivo raíz `LICENSE` establece **Apache License 2.0** (`Copyright 2026 OpenSec Studio Authors`).
   - En contraste, `winget/el-saghet.OpenSecStudio.yaml` declara `Evaluación 60 Días / Licencia Vitalicia por Donación Bitcoin` y `scoop/opensec-studio.json` declara `Freeware / Evaluated`, ambos apuntando al mismo `LICENSE`.
   - **Acción requerida:** Definir una licencia única y coherente antes de integrar componentes de terceros (Kismet GPL-2.0, Wireshark GPL-2.0, Airgorah MIT, SentinelWiFi MIT).
4. **Desfase histórico entre tag `v1.10.10` y `main`:**
   - El tag `v1.10.10` (`f8a4883`) es anterior a los últimos commits de documentación/manifiestos (`ae2bab2`, `c0e081e`), aunque los binarios y PDFs subidos al Release `v1.10.10` en GitHub sí coinciden con los checksums actualizados.

---

## 5. Toolchain Local Disponible (Entorno Host Windows)

Verificado mediante `where.exe` en el entorno actual:

- `docker`: disponible (`DockerDesktop\resources\bin\docker.exe`)
- `cargo` / `rustc`: disponibles (`C:\Users\saghe\.cargo\bin\`)
- `python`: disponible (`C:\Python314\python.exe`)

---

## 6. Criterios de Aceptación de FASE 0 y Decisión para FASE 1

- **Estado de FASE 0:** Completada y documentada con evidencia verificable. No se asume ni se simula ningún módulo inexistente.
- **Condición para iniciar FASE 1:**
  - Confirmar que la nueva plataforma **OpenSec Wireless Security Platform** (arquitectura de 3 niveles, Docker, Kismet, Wireshark/`tshark`, WIDS pasivo, RF Analysis, Risk Engine y Roadmap Linux nativo) se construirá desde cero sobre este repositorio (o en un workspace separado donde resida el código fuente privado).
  - Resolver la postura de licenciamiento (`Apache-2.0` vs licencia comercial/evaluación) dado que **Kismet** y **Wireshark** son **GPL-2.0** (interacción vía proceso/API/contenedor evita contaminación copyleft en el Core si se mantiene aislamiento por IPC/HTTP/CLI, pero debe quedar formalizado en `WIRELESS_ARCHITECTURE.md` y `SBOM.md`).
