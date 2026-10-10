<div align="center">

# 🛡️ OpenSec Studio v1.10.10
### The Ultra-Fast DevSecOps & DAST Workbench in Native Rust

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![Author](https://img.shields.io/badge/Author-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![Engine](https://img.shields.io/badge/Core-Rust%20%7C%20Tauri%20Stripped-orange.svg)](https://tauri.app)
[![Defense](https://img.shields.io/badge/Defense-Anti--Cracking%20Sentinel-emerald.svg)](manuales/)
[![License](https://img.shields.io/badge/License-60--Day%20Evaluation%20%7C%20Lifetime-purple.svg)](LICENSE)

**Read this in your language:**  
🌐 **[English (US)](README.md)** | **[Español (ES)](README.es.md)** | **[Deutsch (DE)](README.de.md)** | **[Français (FR)](README.fr.md)** | **[Italiano (IT)](README.it.md)** | **[日本語 (JA)](README.ja.md)**

---

> **OpenSec Studio** is an ultra-fast, modern alternative to **Burp Suite Professional**, **OWASP ZAP**, and **Nuclei**. Engineered in native **Rust** by **el saghet**, it combines deep HTTP/HTTPS MITM traffic interception, request tampering, fuzzing, and automated vulnerability scanning with instant cold boots (< 180 ms) and an ultralight memory footprint (< 95 MB RAM).

</div>

---

## ⚡ OpenSec Studio vs. Burp Suite Professional

Why cybersecurity researchers, bug bounty hunters, and ethical hackers are switching from Burp Suite to OpenSec Studio:

| Feature / Metric | ☕ Burp Suite Professional | 🛡️ OpenSec Studio (by el saghet) | Impact & Advantage |
| :--- | :---: | :---: | :--- |
| **Startup Time** | 8 – 15 seconds (JVM warm-up) | **< 180 milliseconds** | ⚡ **50x faster launch** on modern Windows |
| **RAM Footprint (Idle)** | 850 MB – 1.4 GB | **~55 MB** | 🍃 **15x lighter**, zero memory hogs |
| **RAM Footprint (Fuzzing)** | 2.5 GB – 4.0 GB | **< 95 MB** | 🚀 Never freezes your machine |
| **Underlying Engine** | Java Virtual Machine (JVM) | **Native Rust (x86_64 Stripped)** | 🔒 Memory-safe, zero JVM vulnerabilities |
| **MITM Proxy & Tamper** | Yes | **Yes (High-Performance Tokio Core)** | 🔄 Instant packet Forward / Drop |
| **Request Repeater & Fuzzer** | Yes | **Yes (Intruder with marker syntax §)** | 🎯 Pre-built SQLi, XSS, Path Traversal payloads |
| **YAML Vulnerability Rules** | Requires third-party plugins | **Native Nuclei-Compatible Engine** | 📜 Run community YAML rules out of the box |
| **JWT Debugger & Inspector** | Third-party BApp store | **Native Integrated Decoder** | 🔍 Auto-detection of `alg: none` flaws |
| **SARIF v2.1.0 Export** | Limited / Complex | **Native 1-Click Export** | 📈 Direct GitHub Security & CI/CD pipeline ingest |
| **Illustrated User Manuals** | English only | **6 Languages (ES, EN, DE, FR, IT, JA)** | 🌍 Full vector PDF & HTML manuals included |
| **1-Click Package Managers** | Manual installer only | `winget install el-saghet.OpenSecStudio` | 📦 Automated installation via WinGet & Scoop |
| **Evaluation Period** | 30 days (Community restricted) | **60 Days Full Unrestricted Access** | 💎 Complete feature access out of the box |

---

## 🚀 Quick Install (Windows 10 / 11)

### Option 1: Windows Package Manager (Winget - Recommended)
```powershell
winget install el-saghet.OpenSecStudio
```

### Option 2: Scoop
```powershell
scoop bucket add opensec https://github.com/saghet/opensec-studio.git
scoop install opensec-studio
```

---

## 📦 Official Binary Downloads (GitHub Releases v1.10.10)

| Package | Format | File Size | Recommended For | Direct Download |
| :--- | :---: | :---: | :--- | :---: |
| **OpenSec Studio Setup** | `.exe` | 1.78 MB | Quick 1-click desktop installation with automatic desktop/start shortcuts | [⬇️ Download .exe](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **Enterprise Installer** | `.msi` | 2.49 MB | Corporate environments, sysadmins & Active Directory GPO deployment | [⬇️ Download .msi](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |

---

## 🔐 Cryptographic Integrity & SHA-256 Checksums

Verify your downloaded binaries against our official cryptographic checksums:

```text
be90f168c12aee724818e6317087d889d7d1aaaec24741edc7beebd1836a51be  Instalador_OpenSec_Studio_Setup.exe
733b6b5a5ab48fd08e337cbbdb2fb5b2620e731b1c6be4ce72589b380653a361  Instalador_Empresarial_OpenSec_Studio.msi
```

To verify in PowerShell:
```powershell
Get-FileHash -Algorithm SHA256 .\Instalador_OpenSec_Studio_Setup.exe
```

---

## 🌟 Core Modules & Architecture

### 1. 🛰️ MITM Proxy Interceptor & HTTP Workbench
- **Real-Time Traffic Interception:** Intercept HTTP/HTTPS requests on localhost:8080. Inspect and alter headers, cookies, query parameters, and request body before forwarding or dropping.
- **Searchable HTTP History:** Instant full-text search, regex filtering by HTTP status, MIME type, method (GET, POST, PUT, DELETE, PATCH), and server response latency.
- **Request Repeater:** Multi-tab workbench to replay manual API requests, benchmark server response times down to microseconds, and craft customized payloads.
- **Intruder / Automated Fuzzer:** Multi-threaded parameter substitution using `§PAYLOAD§` markers with preloaded wordlists for SQL Injection, Cross-Site Scripting (XSS), Directory Traversal, and SSRF.
- **Decoder & JWT Inspector:** High-speed Base64, Hex, URL, and HTML entity encoding/decoding. Visual JWT claim inspector with cryptographic verification warnings (such as unverified signature or `none` algorithm attacks).

### 2. 🎯 Dynamic Application Security Testing (DAST) & YAML Engine
- **Passive & Active DAST Scanners:** Instant identification of missing security headers (`Content-Security-Policy`, `Strict-Transport-Security`, `X-Frame-Options`), insecure cookie attributes (`HttpOnly`, `SameSite`, `Secure`), and sensitive information disclosures.
- **Nuclei-Compatible YAML Engine:** Natively executes declarative security templates to audit misconfigurations, exposed `.git`/`.env` directories, open CORS policies, and known CVE signatures.
- **TCP Host & Port Audit:** High-speed asynchronous socket scanner auditing TCP ports (80, 443, 22, 3306, 5432, 6379, 27017, 8080) with vulnerability risk ratings.
- **Actionable Remediation Code:** Every vulnerability report contains tailored mitigation code snippets for Node.js (Express), Python (Django/FastAPI), Nginx, and Docker.
- **SARIF v2.1.0 & Executive Reports:** Export compliance audits directly to **SARIF v2.1.0** (natively ingested by GitHub Code Scanning and VS Code) or executive **HTML** reports.

### 3. 🛡️ Native Rust Core & Anti-Cracking Defense Sentinel
- **Stripped Native Binary:** Compiled with Rust LTO (Link Time Optimization), stripping symbols and eliminating JVM overhead.
- **Active Anti-Cracking Protection:** Multi-layer defense sentinel protecting against reverse engineering, DLL hijacking (`LOAD_LIBRARY_SEARCH_SYSTEM32`), API hooking, and debugger attachment.

---

## 📖 Illustrated User Manuals (6 Languages)

Complete 18-chapter illustrated user manuals using the intuitive pedagogical framework *"The Great Internet Village"*, compiled into high-definition vector PDFs and interactive HTML:

| Language | Illustrated PDF Manual | Interactive HTML |
| :--- | :---: | :---: |
| 🇪🇸 **Español** | [Manual_OpenSec_Studio_ES_El_Saghet.pdf](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf) | [manual_es.html](manuales/manual_es.html) |
| 🇺🇸 **English** | [Manual_OpenSec_Studio_EN_El_Saghet.pdf](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf) | [manual_en.html](manuales/manual_en.html) |
| 🇩🇪 **Deutsch** | [Manual_OpenSec_Studio_DE_El_Saghet.pdf](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf) | [manual_de.html](manuales/manual_de.html) |
| 🇫🇷 **Français** | [Manual_OpenSec_Studio_FR_El_Saghet.pdf](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf) | [manual_fr.html](manuales/manual_fr.html) |
| 🇮🇹 **Italiano** | [Manual_OpenSec_Studio_IT_El_Saghet.pdf](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf) | [manual_it.html](manuales/manual_it.html) |
| 🇯🇵 **日本語** | [Manual_OpenSec_Studio_JA_El_Saghet.pdf](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf) | [manual_ja.html](manuales/manual_ja.html) |

---

## 🌐 Official Website & Web Portal

Explore live demos, feature tours, and download links on the official web portal:  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 Author & Licensing

- **Lead Architect & Creator:** **el saghet**
- **Evaluation Period:** 60 days unrestricted evaluation period with full access to all modules.
- **Community & Lifetime Support:** Voluntary donations welcome via official Bitcoin address:  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
