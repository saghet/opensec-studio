<div align="center">

# 🛡️ OpenSec Studio v1.10.10
### L'Atelier DevSecOps & DAST Ultra-Rapide Développé en Rust Natif

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![Auteur](https://img.shields.io/badge/Auteur-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![Plateforme](https://img.shields.io/badge/Plateforme-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![Moteur](https://img.shields.io/badge/Moteur-Rust%20%7C%20Tauri-orange.svg)](https://tauri.app)
[![Sécurité](https://img.shields.io/badge/Défense-Sentinelle%20Anti--Cracking-emerald.svg)](manuales/)
[![Licence](https://img.shields.io/badge/Licence-Évaluation%2060%20Jours%20%7C%20À%20Vie-purple.svg)](LICENSE)

**Disponible dans votre langue :**  
🌐 **[English (US)](README.md)** | **[Español (ES)](README.es.md)** | **[Deutsch (DE)](README.de.md)** | **[Français (FR)](README.fr.md)** | **[Italiano (IT)](README.it.md)** | **[日本語 (JA)](README.ja.md)**

---

> **OpenSec Studio** est l'alternative moderne et ultra-légère à **Burp Suite Professional**, **OWASP ZAP** et **Nuclei** pour les experts en cybersécurité, pentesters et chasseurs de bugs (bug bounty). Conçu en **Rust natif** par **el saghet**, il réunit l'interception de requêtes HTTP/HTTPS MITM, la modification de paquets en temps réel, le fuzzing haute performance et l'analyse automatisée de vulnérabilités, avec un démarrage éclair (< 180 ms) et une empreinte mémoire inférieure à 95 Mo de RAM.

</div>

---

## ⚡ Comparatif : OpenSec Studio vs. Burp Suite Professional

Pourquoi les professionnels de la sécurité informatique choisissent OpenSec Studio :

| Fonctionnalité / Métrique | ☕ Burp Suite Professional | 🛡️ OpenSec Studio (par el saghet) | Avantage Clé |
| :--- | :---: | :---: | :--- |
| **Temps de Démarrage** | 8 – 15 secondes (initialisation JVM) | **< 180 millisecondes** | ⚡ **50x plus rapide** |
| **Consommation RAM (Repos)** | 850 Mo – 1.4 Go | **~55 Mo** | 🍃 **15x plus léger**, aucun ralentissement |
| **Consommation RAM (Fuzzing)** | 2.5 Go – 4.0 Go | **< 95 Mo** | 🚀 Fluidité absolue même sur PC portable |
| **Moteur d'Exécution** | Java Virtual Machine (JVM) | **Rust Natif (x86_64 Stripped)** | 🔒 Sécurité mémoire garantie, zéro faille JVM |
| **Proxy MITM & Intercepteur** | Oui | **Oui (Cœur asynchrone Tokio)** | 🔄 Transmission (Forward) / Rejet (Drop) instantanés |
| **Répéteur & Fuzzer Automatisé** | Oui | **Oui (Intruder avec syntaxe §PAYLOAD§)** | 🎯 Payloads intégrés (SQLi, XSS, Path Traversal) |
| **Règles YAML de Vulnérabilités** | Plugins tiers requis | **Moteur Natif compatible Nuclei** | 📜 Prise en charge directe des templates communautaires |
| **Inspecteur & Décodeur JWT** | Extensions tierces | **Natif et intégré (Decoder)** | 🔍 Détection automatique des failles `alg: none` |
| **Exportation SARIF v2.1.0** | Limitée | **Export Natif en 1 Clic** | 📈 Intégration immédiate avec GitHub Security & CI/CD |
| **Manuels Illustrés en Français** | Uniquement en anglais technique | **Complet en Français (18 chapitres)** | 🌍 Format vectoriel PDF et HTML haute définition |
| **Installation en 1 Ligne sous Windows** | Installateur manuel lourd | `winget install el-saghet.OpenSecStudio` | 📦 Prise en charge native de WinGet et Scoop |
| **Période d'Évaluation** | 30 jours avec modules restreints | **60 Jours d'Accès Total Illimité** | 💎 Tous les modules opérationnels dès le premier jour |

---

## 🚀 Installation Rapide (Windows 10 / 11)

### Option 1 : Gestionnaire de Paquets Windows (Winget - Recommandé)
```powershell
winget install el-saghet.OpenSecStudio
```

### Option 2 : Scoop
```powershell
scoop bucket add opensec https://github.com/saghet/opensec-studio.git
scoop install opensec-studio
```

---

## 📦 Téléchargements Officiels des Fichiers Binaires (Releases v1.10.10)

| Fichier | Format | Taille | Recommandé Pour | Lien de Téléchargement |
| :--- | :---: | :---: | :--- | :---: |
| **Installateur OpenSec Studio** | `.exe` | 1.78 Mo | Installation standard avec raccourcis bureau et menu démarrer | [⬇️ Télécharger .exe](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **Installateur Entreprise** | `.msi` | 2.49 Mo | Déploiements Active Directory (GPO) et parcs d'entreprise | [⬇️ Télécharger .msi](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |
| **Édition Portable** | `.zip` | 16.69 Mo | Clés USB de pentest, laboratoires hors-ligne et environnements isolés | [⬇️ Télécharger .zip](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/OpenSec-Studio-v1.10.10-Windows-x64.zip) |

---

## 🔐 Empreintes Cryptographiques SHA-256

Vérifiez l'intégrité de vos téléchargements avec les hachages officiels :

```text
e11bca47a1a0b648adad882e9f9b37ccb96ba20d40b8cafadc373d0164d6225c  Instalador_OpenSec_Studio_Setup.exe
f226af2852bcf71b6256c3a720a4ae5e52e2171b6b8a10433b05f83eb1c72f0b  Instalador_Empresarial_OpenSec_Studio.msi
abf2b20412765c9e70c39d820df049dfae032a6f081f89371320f6db558530a3  OpenSec Studio.exe (Portable)
50e493990d255d74e914373eff11dd91646106fe7ebe747804007f75ff1dbe7d  OpenSec-Studio-v1.10.10-Windows-x64.zip
```

---

## 📖 Manuels d'Utilisation en 6 Langues

- 🇫🇷 **[Manuel en Français (PDF)](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf)** | [Voir HTML](manuales/manual_fr.html)
- 🇺🇸 **[User Manual in English (PDF)](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf)** | [HTML](manuales/manual_en.html)
- 🇪🇸 **[Manual en Español (PDF)](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf)** | [HTML](manuales/manual_es.html)
- 🇩🇪 **[Benutzerhandbuch auf Deutsch (PDF)](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf)** | [HTML](manuales/manual_de.html)
- 🇮🇹 **[Manuale in Italiano (PDF)](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf)** | [HTML](manuales/manual_it.html)
- 🇯🇵 **[ユーザーマニュアル 日本語 (PDF)](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf)** | [HTML](manuales/manual_ja.html)

---

## 🌐 Site Web Officiel

Visitez le portail officiel :  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 Auteur & Licence

- **Architecte Principal & Créateur :** **el saghet**
- **Période d'Évaluation :** 60 jours d'évaluation sans restriction.
- **Dons & Soutien Communautaire :** Adresse Bitcoin officielle :  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
