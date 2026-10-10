<div align="center">

# 🛡️ OpenSec Studio v1.10.10
### Die ultraschnelle DevSecOps- & DAST-Workbench in nativem Rust

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![Autor](https://img.shields.io/badge/Autor-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![Plattform](https://img.shields.io/badge/Plattform-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![Engine](https://img.shields.io/badge/Kern-Rust%20%7C%20Tauri-orange.svg)](https://tauri.app)
[![Sicherheit](https://img.shields.io/badge/Schutz-Anti--Cracking%20Sentinel-emerald.svg)](manuales/)
[![Lizenz](https://img.shields.io/badge/Lizenz-60--Tage%20Testversion%20%7C%20Dauerhaft-purple.svg)](LICENSE)

**In Ihrer Sprache verfügbar:**  
🌐 **[English (US)](README.md)** | **[Español (ES)](README.es.md)** | **[Deutsch (DE)](README.de.md)** | **[Français (FR)](README.fr.md)** | **[Italiano (IT)](README.it.md)** | **[日本語 (JA)](README.ja.md)**

---

> **OpenSec Studio** ist die moderne, extrem leichtgewichtige Alternative zu **Burp Suite Professional**, **OWASP ZAP** und **Nuclei** für Sicherheitsforscher, Pentesters und ethische Hacker. Entwickelt in **nativem Rust** von **el saghet**, kombiniert die Suite tiefgreifende HTTP/HTTPS-MITM-Verkehrsüberwachung, Request-Manipulation, automatisiertes Fuzzing und DAST-Schwachstellenscans mit sofortigem Kaltstart (< 180 ms) und weniger als 95 MB RAM-Verbrauch.

</div>

---

## ⚡ Vergleich: OpenSec Studio vs. Burp Suite Professional

Warum Sicherheitsexperten von Burp Suite zu OpenSec Studio wechseln:

| Funktion / Metrik | ☕ Burp Suite Professional | 🛡️ OpenSec Studio (von el saghet) | Vorteil |
| :--- | :---: | :---: | :--- |
| **Startzeit** | 8 – 15 Sekunden (JVM-Start) | **< 180 Millisekunden** | ⚡ **50x schnellerer Start** |
| **RAM-Verbrauch (Leerlauf)** | 850 MB – 1,4 GB | **~55 MB** | 🍃 **15x sparsamer**, kein Speicherhunger |
| **RAM-Verbrauch (Fuzzing)** | 2,5 GB – 4,0 GB | **< 95 MB** | 🚀 Kein Einfrieren des Betriebssystems |
| **Laufzeitumgebung** | Java Virtual Machine (JVM) | **Natives Rust (x86_64 Stripped)** | 🔒 Speichersicher, keine JVM-Lücken |
| **MITM-Proxy & Interceptor** | Ja | **Ja (Asynchroner Tokio-Kern)** | 🔄 Sofortiges Weiterleiten (Forward) / Verwerfen (Drop) |
| **Repeater & Automatisierter Fuzzer** | Ja | **Ja (Intruder mit §PAYLOAD§-Syntax)** | 🎯 Integrierte Payloads (SQLi, XSS, Path Traversal) |
| **YAML-Sicherheitsregeln** | Drittanbieter-Plugins erforderlich | **Nativer Nuclei-kompatibler Parser** | 📜 Unterstützt Community-YAML-Vorlagen |
| **JWT-Debugger & Inspektor** | Nur über BApp-Store | **Nativ integriert (Decoder)** | 🔍 Automatische Warnung bei `alg: none` |
| **SARIF v2.1.0-Export** | Eingeschränkt | **Nativer 1-Klick-Export** | 📈 Nahtlose CI/CD- & GitHub Security-Integration |
| **Illustrierte Handbücher auf Deutsch** | Nur auf Englisch | **Vollständig auf Deutsch (PDF & HTML)** | 🌍 18 Kapitel inklusive anschaulicher Diagramme |
| **1-Klick-Installation unter Windows** | Manuelle Installation | `winget install el-saghet.OpenSecStudio` | 📦 Automatisierte WinGet- & Scoop-Unterstützung |
| **Evaluierungszeitraum** | 30 Tage (Community eingeschränkt) | **60 Tage uneingeschränkter Vollzugriff** | 💎 Alle Funktionen sofort einsatzbereit |

---

## 🚀 Schnelle Installation (Windows 10 / 11)

### Option 1: Windows Package Manager (Winget - Empfohlen)
```powershell
winget install el-saghet.OpenSecStudio
```

### Option 2: Scoop
```powershell
scoop bucket add opensec https://github.com/saghet/opensec-studio.git
scoop install opensec-studio
```

---

## 📦 Offizielle Binärdownloads (GitHub Releases v1.10.10)

| Datei | Format | Größe | Empfohlen für | Download |
| :--- | :---: | :---: | :--- | :---: |
| **OpenSec Studio Setup** | `.exe` | 1.78 MB | 1-Klick-Desktop-Installation mit Startmenü-Verknüpfungen | [⬇️ Herunterladen .exe](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **Enterprise Installer** | `.msi` | 2.49 MB | IT-Abteilungen, Unternehmensnetzwerke und GPO-Rollouts | [⬇️ Herunterladen .msi](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |
| **Portable Edition** | `.zip` | 16.69 MB | Standalone auf USB-Sticks, geschlossene Pentest-Umgebungen | [⬇️ Herunterladen .zip](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/OpenSec-Studio-v1.10.10-Windows-x64.zip) |

---

## 🔐 Kryptografische Integrität & SHA-256 Prüfsummen

Überprüfen Sie Ihre Downloads anhand der offiziellen Prüfsummen:

```text
e11bca47a1a0b648adad882e9f9b37ccb96ba20d40b8cafadc373d0164d6225c  Instalador_OpenSec_Studio_Setup.exe
f226af2852bcf71b6256c3a720a4ae5e52e2171b6b8a10433b05f83eb1c72f0b  Instalador_Empresarial_OpenSec_Studio.msi
abf2b20412765c9e70c39d820df049dfae032a6f081f89371320f6db558530a3  OpenSec Studio.exe (Portabel)
50e493990d255d74e914373eff11dd91646106fe7ebe747804007f75ff1dbe7d  OpenSec-Studio-v1.10.10-Windows-x64.zip
```

---

## 📖 Benutzerhandbücher in 6 Sprachen

- 🇩🇪 **[Benutzerhandbuch auf Deutsch (PDF)](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf)** | [HTML ansehen](manuales/manual_de.html)
- 🇺🇸 **[User Manual in English (PDF)](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf)** | [HTML](manuales/manual_en.html)
- 🇪🇸 **[Manual en Español (PDF)](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf)** | [HTML](manuales/manual_es.html)
- 🇫🇷 **[Manuel en Français (PDF)](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf)** | [HTML](manuales/manual_fr.html)
- 🇮🇹 **[Manuale in Italiano (PDF)](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf)** | [HTML](manuales/manual_it.html)
- 🇯🇵 **[ユーザーマニュアル 日本語 (PDF)](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf)** | [HTML](manuales/manual_ja.html)

---

## 🌐 Offizielle Website

Besuchen Sie das Webportal:  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 Autor & Lizenz

- **Chefarchitekt:** **el saghet**
- **Testzeitraum:** 60 Tage uneingeschränkter Testzugang.
- **Freiwillige Spenden & dauerhafter Zugang:** Offizielle Bitcoin-Adresse:  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
