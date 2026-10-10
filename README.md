# 🛡️ OpenSec Studio v1.10.10

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![Author](https://img.shields.io/badge/Author-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![Engine](https://img.shields.io/badge/Core-Rust%20%7C%20Tauri%20Stripped-orange.svg)](https://tauri.app)
[![Defense](https://img.shields.io/badge/Defense-Anti--Cracking%20Sentinel-emerald.svg)](manuales/)
[![License](https://img.shields.io/badge/License-60--Day%20Evaluation%20%7C%20Lifetime-purple.svg)](LICENSE)

> **DevSecOps & DAST Workbench:** The next-generation vulnerability analysis and auditing suite for cybersecurity researchers, ethical hackers, and bug bounty hunters. 
> Combining the deep traffic interception of **Burp Suite** with the lightning-fast automated template scanning of **Nuclei**, re-engineered from scratch with a native **Rust** core for sub-180ms cold startup and less than 95 MB RAM consumption.
>
> Architected and maintained by **el saghet**.

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
| **OpenSec Studio Setup** | `.exe` | 1.78 MB | Quick 1-click desktop installation with automatic shortcuts | [⬇️ Download .exe](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **Enterprise Installer** | `.msi` | 2.49 MB | Corporate environments, sysadmins & Active Directory GPO deployment | [⬇️ Download .msi](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |
| **Portable Edition** | `.zip` | 16.69 MB | Standalone USB drives, zero-install pentest environments & offline rigs | [⬇️ Download .zip](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/OpenSec-Studio-v1.10.10-Windows-x64.zip) |

---

## 🔐 Cryptographic Integrity & SHA-256 Checksums

Verify your downloaded binaries against our official cryptographic checksums:

```text
e11bca47a1a0b648adad882e9f9b37ccb96ba20d40b8cafadc373d0164d6225c  Instalador_OpenSec_Studio_Setup.exe
f226af2852bcf71b6256c3a720a4ae5e52e2171b6b8a10433b05f83eb1c72f0b  Instalador_Empresarial_OpenSec_Studio.msi
abf2b20412765c9e70c39d820df049dfae032a6f081f89371320f6db558530a3  OpenSec Studio.exe (Portable)
50e493990d255d74e914373eff11dd91646106fe7ebe747804007f75ff1dbe7d  OpenSec-Studio-v1.10.10-Windows-x64.zip
```

To verify on PowerShell:
```powershell
Get-FileHash -Algorithm SHA256 .\Instalador_OpenSec_Studio_Setup.exe
```

---

## 🌟 Key Capabilities & Features

### 1. Web Traffic Interception & Inspection (Burp Suite Alternative)
- **MITM Proxy Interceptor:** Real-time HTTP/HTTPS traffic capture with full payload modification (Forward / Drop).
- **HTTP History:** High-speed searchable history with method, status code, latency, and regex filtering.
- **Request Repeater:** Multi-tab manual request workbench with millisecond precision response timing.
- **Intruder / Fuzzer:** Multi-threaded payload injection (`§PAYLOAD§`) with preconfigured lists (SQLi, XSS, Path Traversal).
- **Decoder & JWT Inspector:** Instant Base64, Hex, URL encoding/decoding and structured JWT parsing with security claim alerts (e.g. `alg: none`).

### 2. Automated Vulnerability Scanning (Nuclei Alternative)
- **Active & Passive DAST Engines:** Automatic detection of misconfigured headers (CSP, HSTS), unflagged cookies, information disclosure, and sensitive file leaks.
- **Nuclei-Compatible YAML Engine:** Execution of custom or community YAML vulnerability templates.
- **Host & Port Audit:** Fast concurrent TCP socket discovery for exposed database, SSH, and administrative ports.
- **Standardized Export:** Direct report export to **SARIF v2.1.0** (GitHub Security compliant) and **Executive HTML**.

### 3. Native Rust Engine & Security Sentinel
- **Cold Boot in < 180ms:** Instant launch with no JVM overhead.
- **Minimal RAM Footprint:** Idles at ~55 MB RAM, stays below 95 MB under heavy concurrency.
- **Anti-Cracking Sentinel:** Multi-layered defense preventing binary patching, DLL hijacking, and API hooking.

---

## 📖 Illustrated User Manuals (6 Languages)

Complete, step-by-step illustrated documentation guides available in the [`manuales/`](manuales/) directory:

- 🇪🇸 **[Manual de Usuario en Español (PDF)](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf)** | [Ver HTML](manuales/manual_es.html)
- 🇺🇸 **[User Manual in English (PDF)](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf)** | [View HTML](manuales/manual_en.html)
- 🇩🇪 **[Benutzerhandbuch auf Deutsch (PDF)](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf)** | [HTML ansehen](manuales/manual_de.html)
- 🇫🇷 **[Manuel d'utilisation en Français (PDF)](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf)** | [Voir HTML](manuales/manual_fr.html)
- 🇮🇹 **[Manuale Utente in Italiano (PDF)](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf)** | [Visualizza HTML](manuales/manual_it.html)
- 🇯🇵 **[ユーザーマニュアル 日本語 (PDF)](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf)** | [HTMLを見る](manuales/manual_ja.html)

---

## 🌐 Official Website

Visit the official interactive web portal at:  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 Author & Licensing

- **Author & Architect:** **el saghet**
- **Evaluation:** 60 days unrestricted evaluation period.
- **Support & Development:** Voluntary contributions welcome via official Bitcoin address:  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
