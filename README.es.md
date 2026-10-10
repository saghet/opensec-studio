<div align="center">

# 🛡️ OpenSec Studio v1.10.10
### La Suite Ultraligera de Ciberseguridad Ofensiva y Auditoría DAST en Rust Nativo

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![Autor](https://img.shields.io/badge/Autor-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![Plataforma](https://img.shields.io/badge/Plataforma-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![Motor](https://img.shields.io/badge/Motor-Rust%20%7C%20Tauri-orange.svg)](https://tauri.app)
[![Defensa](https://img.shields.io/badge/Seguridad-Centinela%20Anti--Cracking-emerald.svg)](manuales/)
[![Licencia](https://img.shields.io/badge/Licencia-Evaluación%2060%20Días%20%7C%20Vitalicia-purple.svg)](LICENSE)

**Disponible en tu idioma:**  
🌐 **[English (US)](README.md)** | **[Español (ES)](README.es.md)** | **[Deutsch (DE)](README.de.md)** | **[Français (FR)](README.fr.md)** | **[Italiano (IT)](README.it.md)** | **[日本語 (JA)](README.ja.md)**

---

> **OpenSec Studio** es la alternativa moderna y ultrarrápida a **Burp Suite Professional**, **OWASP ZAP** y **Nuclei** para pentesters, analistas de seguridad y cazadores de bug bounty. Desarrollado en **Rust nativo** por **el saghet**, combina la intercepción profunda de tráfico HTTP/HTTPS MITM, manipulación de peticiones, fuzzing masivo y escaneo automatizado de vulnerabilidades con un arranque instantáneo (< 180 ms) y consumo menor a 95 MB de RAM.

</div>

---

## ⚡ Comparativa: OpenSec Studio vs. Burp Suite Professional

Por qué los profesionales de ciberseguridad y hackers éticos están migrando de Burp Suite a OpenSec Studio:

| Característica / Métrica | ☕ Burp Suite Professional | 🛡️ OpenSec Studio (por el saghet) | Ventaja Clave |
| :--- | :---: | :---: | :--- |
| **Tiempo de Arranque** | 8 – 15 segundos (carga de máquina Java) | **< 180 milisegundos** | ⚡ **50x más rápido** al iniciar |
| **Consumo de Memoria RAM (Reposo)** | 850 MB – 1.4 GB | **~55 MB** | 🍃 **15x más ligero**, no ralentiza tu PC |
| **Consumo de Memoria (Fuzzing Activo)** | 2.5 GB – 4.0 GB | **< 95 MB** | 🚀 Máximo rendimiento sin bloqueos |
| **Arquitectura de Ejecución** | Java Virtual Machine (JVM) | **Rust Nativo (x86_64 Stripped)** | 🔒 Seguridad de memoria, sin fallos JVM |
| **Proxy MITM e Interceptor** | Sí | **Sí (Motor asíncrono Tokio)** | 🔄 Reenvío (Forward) y descarte (Drop) instantáneo |
| **Repeater de Peticiones y Fuzzer** | Sí | **Sí (Intruder con sintaxis §PAYLOAD§)** | 🎯 Payloads incluidos (SQLi, XSS, Path Traversal) |
| **Reglas de Vulnerabilidad YAML** | Requiere extensiones pesadas | **Motor Nativo compatible con Nuclei** | 📜 Ejecución directa de plantillas YAML comunitarias |
| **Inspector y Decodificador JWT** | Plugins de terceros | **Integrado de serie (Decoder)** | 🔍 Alertas de algoritmo inseguro `alg: none` |
| **Exportación SARIF v2.1.0** | Compleja / Limitada | **Exportación Nativa en 1 Clic** | 📈 Integración directa con GitHub Security y CI/CD |
| **Manuales Ilustrados en Español** | Solo inglés técnico | **Completo en Español (18 capítulos)** | 🌍 Manual ilustrado en PDF vectorial de alta definición |
| **Instalación en Windows en 1 Clic** | Instalador manual pesado | `winget install el-saghet.OpenSecStudio` | 📦 Instalación automatizada con WinGet o Scoop |
| **Periodo de Prueba** | 30 días con funciones capadas | **60 Días Completos sin Restricciones** | 💎 Todas las herramientas desbloqueadas de inicio |

---

## 🚀 Instalación Rápida en Windows 10 / 11

### Opción 1: Administrador de Paquetes de Windows (Winget - Recomendado)
```powershell
winget install el-saghet.OpenSecStudio
```

### Opción 2: Scoop
```powershell
scoop bucket add opensec https://github.com/saghet/opensec-studio.git
scoop install opensec-studio
```

---

## 📦 Descarga de Instaladores Oficiales (GitHub Releases v1.10.10)

| Archivo | Formato | Tamaño | Recomendado Para | Enlace de Descarga |
| :--- | :---: | :---: | :--- | :---: |
| **Instalador OpenSec Studio** | `.exe` | 1.78 MB | Instalación automática con accesos directos de escritorio | [⬇️ Descargar .exe](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **Instalador Empresarial** | `.msi` | 2.49 MB | Despliegue en redes corporativas, Active Directory y GPO | [⬇️ Descargar .msi](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |

---

## 🔐 Integridad Criptográfica y Checksums SHA-256

Verifica tus instaladores descargados con los hashes SHA-256 oficiales:

```text
be90f168c12aee724818e6317087d889d7d1aaaec24741edc7beebd1836a51be  Instalador_OpenSec_Studio_Setup.exe
733b6b5a5ab48fd08e337cbbdb2fb5b2620e731b1c6be4ce72589b380653a361  Instalador_Empresarial_OpenSec_Studio.msi
```

Para verificar en PowerShell:
```powershell
Get-FileHash -Algorithm SHA256 .\Instalador_OpenSec_Studio_Setup.exe
```

---

## 🌟 Módulos y Capacidades del Sistema

### 1. 🛰️ Interceptor Proxy MITM y Banco de Trabajo HTTP (Estilo Burp Suite)
- **Proxy Interceptor en Tiempo Real:** Captura tráfico HTTP/HTTPS en `localhost:8080`. Pausa y edita cabeceras, cookies, parámetros y cuerpo de petición antes de enviarlas (`Forward`) o descartarlas (`Drop`).
- **Historial HTTP con Búsqueda Instantánea:** Búsqueda full-text en milisegundos, filtrado por métodos HTTP (GET, POST, PUT, DELETE), códigos de respuesta (200, 301, 403, 500) y latencia.
- **Repeater de Peticiones:** Entorno multi-pestaña para reenviar llamadas API manualmente, ajustar cabeceras y medir el tiempo exacto de respuesta del servidor en microsegundos.
- **Intruder / Fuzzer Automatizado:** Inyección concurrente de payloads mediante marcadores `§PAYLOAD§` con listas de palabras precargadas para SQL Injection, Cross-Site Scripting (XSS), LFI/RFI y SSRF.
- **Decoder e Inspector JWT:** Codificación y decodificación ultra-rápida en Base64, Hex, URL y entidades HTML. Inspector visual de tokens JWT con avisos de vulnerabilidad (firma omitida o ataque `alg: none`).

### 2. 🎯 Análisis Dinámico de Vulnerabilidades (DAST) y Motor YAML (Estilo Nuclei)
- **Escáner DAST Pasivo y Activo:** Detección automática de cabeceras de seguridad faltantes (`CSP`, `HSTS`, `X-Frame-Options`), cookies inseguras y exposición de archivos confidenciales.
- **Compatibilidad con Plantillas YAML de Nuclei:** Ejecuta reglas declarativas en formato YAML para auditar repositorios `.git` expuestos, fugas de archivos `.env`, CORS con comodín inseguro y firmas de CVEs conocidos.
- **Auditoría de Puertos y Servicios TCP:** Escáner asíncrono de sockets TCP (80, 443, 22, 3306, 5432, 6379, 27017, 8080) con cálculo de riesgo por exposición.
- **Código de Remediación Listo para Desarrolladores:** Cada vulnerabilidad incluye código de corrección en Node.js (Express), Python (Django/FastAPI), Nginx y Docker.
- **Reportes en SARIF v2.1.0 y HTML:** Genera reportes estándar compatibles con **GitHub Security** y resúmenes ejecutivos en **HTML interactivo**.

### 3. 🛡️ Centinela de Seguridad y Blindaje Anti-Cracking
- **Binario Nativo Stripped:** Compilado con optimizaciones Link-Time Optimization (LTO) de Rust.
- **Centinela Anti-Manipulación:** Detección de depuradores, bloqueo de inyección de DLLs (`LOAD_LIBRARY_SEARCH_SYSTEM32`) e integridad de prólogos en memoria para neutralizar hooking.

---

## 📖 Manuales de Usuario Ilustrados en 6 Idiomas

Accede a la documentación completa de 18 capítulos basada en la metáfora pedagógica *"La Gran Aldea de Internet"*:

- 🇪🇸 **[Manual en Español (PDF Vectorial)](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf)** | [Ver Online HTML](manuales/manual_es.html)
- 🇺🇸 **[User Manual in English (PDF)](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf)** | [HTML](manuales/manual_en.html)
- 🇩🇪 **[Benutzerhandbuch auf Deutsch (PDF)](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf)** | [HTML](manuales/manual_de.html)
- 🇫🇷 **[Manuel en Français (PDF)](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf)** | [HTML](manuales/manual_fr.html)
- 🇮🇹 **[Manuale in Italiano (PDF)](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf)** | [HTML](manuales/manual_it.html)
- 🇯🇵 **[ユーザーマニュアル 日本語 (PDF)](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf)** | [HTML](manuales/manual_ja.html)

---

## 🌐 Sitio Web Oficial

Visita el portal web interactivo oficial:  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 Autor y Licenciamiento

- **Autor y Arquitecto Principal:** **el saghet**
- **Periodo de Evaluación:** 60 días de acceso total irrestricto a todas las herramientas.
- **Apoyo Comunitario y Acceso Vitalicio:** Donaciones voluntarias recibidas en la dirección oficial de Bitcoin:  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
