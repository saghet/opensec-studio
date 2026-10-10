<div align="center">

# 🛡️ OpenSec Studio v1.10.10
### Il Workbench DevSecOps e DAST Ultra-Rapido Sviluppato in Rust Nativo

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![Autore](https://img.shields.io/badge/Autore-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![Piattaforma](https://img.shields.io/badge/Piattaforma-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![Motore](https://img.shields.io/badge/Core-Rust%20%7C%20Tauri-orange.svg)](https://tauri.app)
[![Sicurezza](https://img.shields.io/badge/Protezione-Anti--Cracking%20Sentinel-emerald.svg)](manuales/)
[![Licenza](https://img.shields.io/badge/Licenza-Valutazione%2060%20Giorni%20%7C%20A%20Vita-purple.svg)](LICENSE)

**Disponibile nella tua lingua:**  
🌐 **[English (US)](README.md)** | **[Español (ES)](README.es.md)** | **[Deutsch (DE)](README.de.md)** | **[Français (FR)](README.fr.md)** | **[Italiano (IT)](README.it.md)** | **[日本語 (JA)](README.ja.md)**

---

> **OpenSec Studio** è l'alternativa moderna, leggera e ad alte prestazioni a **Burp Suite Professional**, **OWASP ZAP** e **Nuclei** per ricercatori di sicurezza, penetration tester ed hacker etici. Progettato in **Rust nativo** da **el saghet**, combina intercettazione del traffico HTTP/HTTPS MITM in tempo reale, modifica dei pacchetti, fuzzing ad alta velocità e scansione automatizzata delle vulnerabilità con avvio fulmineo (< 180 ms) e consumo inferiore a 95 MB di RAM.

</div>

---

## ⚡ Confronto: OpenSec Studio vs. Burp Suite Professional

Perché i professionisti di sicurezza informatica scelgono OpenSec Studio:

| Funzionalità / Metrica | ☕ Burp Suite Professional | 🛡️ OpenSec Studio (di el saghet) | Vantaggio Chiave |
| :--- | :---: | :---: | :--- |
| **Tempo di Avvio** | 8 – 15 secondi (avvio macchina Java) | **< 180 millisecondi** | ⚡ **50x più rapido** all'avvio |
| **Consumo RAM (Inattivo)** | 850 MB – 1.4 GB | **~55 MB** | 🍃 **15x più leggero**, nessun rallentamento del PC |
| **Consumo RAM (Fuzzing Intenso)** | 2.5 GB – 4.0 GB | **< 95 MB** | 🚀 Zero crash o blocchi di memoria |
| **Motore Sottostante** | Java Virtual Machine (JVM) | **Rust Nativo (x86_64 Stripped)** | 🔒 Massima sicurezza della memoria |
| **Proxy MITM e Intercettore** | Sì | **Sì (Motore asincrono Tokio)** | 🔄 Inoltro (Forward) e scarto (Drop) istantanei |
| **Ripetitore e Fuzzer Automatico** | Sì | **Sì (Intruder con sintassi §PAYLOAD§)** | 🎯 Payload integrati (SQLi, XSS, Path Traversal) |
| **Regole di Vulnerabilità YAML** | Richiede plugin aggiuntivi | **Motore Nativo compatibile Nuclei** | 📜 Esecuzione immediata dei template comunitari |
| **Ispettore e Decodificatore JWT** | Componenti aggiuntivi BApp | **Integrato di serie (Decoder)** | 🔍 Allerta automatica per algoritmo `alg: none` |
| **Esportazione SARIF v2.1.0** | Complessa / Limitata | **Esportazione Nativa in 1 Clic** | 📈 Ingestione diretta su GitHub Security e CI/CD |
| **Manuali Illustrati in Italiano** | Solo in inglese tecnico | **Completo in Italiano (18 capitoli)** | 🌍 Manuale vettoriale PDF in alta definizione |
| **Installazione in 1 Clic su Windows**| Installatore manuale pesante | `winget install el-saghet.OpenSecStudio` | 📦 Supporto nativo per WinGet e Scoop |
| **Periodo di Valutazione** | 30 giorni con limitazioni | **60 Giorni Completi senza Restrizioni** | 💎 Tutte le funzionalità attive dal primo avvio |

---

## 🚀 Installazione Rapida (Windows 10 / 11)

### Opzione 1: Gestore Pacchetti Windows (Winget - Consigliato)
```powershell
winget install el-saghet.OpenSecStudio
```

### Opzione 2: Scoop
```powershell
scoop bucket add opensec https://github.com/saghet/opensec-studio.git
scoop install opensec-studio
```

---

## 📦 Download Binari Ufficiali (GitHub Releases v1.10.10)

| File | Formato | Dimensione | Consigliato Per | Link di Download |
| :--- | :---: | :---: | :--- | :---: |
| **Programma di Installazione Setup** | `.exe` | 1.78 MB | Installazione desktop rapida con creazione automatica collegamenti | [⬇️ Scarica .exe](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **Installatore Aziendale (MSI)** | `.msi` | 2.49 MB | Reti aziendali, amministratori di sistema e distribuzione GPO | [⬇️ Scarica .msi](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |
| **Edizione Portatile (Portable)** | `.zip` | 16.69 MB | Chiavette USB di pentest, laboratori isolati e forensic | [⬇️ Scarica .zip](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/OpenSec-Studio-v1.10.10-Windows-x64.zip) |

---

## 🔐 Integrità Crittografica e Checksum SHA-256

Verifica l'integrità dei tuoi file scaricati con gli hash SHA-256 ufficiali:

```text
e11bca47a1a0b648adad882e9f9b37ccb96ba20d40b8cafadc373d0164d6225c  Instalador_OpenSec_Studio_Setup.exe
f226af2852bcf71b6256c3a720a4ae5e52e2171b6b8a10433b05f83eb1c72f0b  Instalador_Empresarial_OpenSec_Studio.msi
abf2b20412765c9e70c39d820df049dfae032a6f081f89371320f6db558530a3  OpenSec Studio.exe (Versione Portatile)
50e493990d255d74e914373eff11dd91646106fe7ebe747804007f75ff1dbe7d  OpenSec-Studio-v1.10.10-Windows-x64.zip
```

---

## 📖 Manuali Utente in 6 Lingue

- 🇮🇹 **[Manuale in Italiano (PDF)](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf)** | [Visualizza HTML](manuales/manual_it.html)
- 🇺🇸 **[User Manual in English (PDF)](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf)** | [HTML](manuales/manual_en.html)
- 🇪🇸 **[Manual en Español (PDF)](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf)** | [HTML](manuales/manual_es.html)
- 🇩🇪 **[Benutzerhandbuch auf Deutsch (PDF)](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf)** | [HTML](manuales/manual_de.html)
- 🇫🇷 **[Manuel en Français (PDF)](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf)** | [HTML](manuales/manual_fr.html)
- 🇯🇵 **[ユーザーマニュアル 日本語 (PDF)](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf)** | [HTML](manuales/manual_ja.html)

---

## 🌐 Sito Web Ufficiale

Visita il portale web ufficiale:  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 Autore e Licenza

- **Architetto Principale & Autore:** **el saghet**
- **Periodo di Valutazione:** 60 giorni ad accesso completo illimitato.
- **Sostegno della Comunità:** Indirizzo Bitcoin ufficiale per donazioni:  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
