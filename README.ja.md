<div align="center">

# 🛡️ OpenSec Studio v1.10.10
### ネイティブRust製 超高速DevSecOps＆DAST脆弱性診断ワークベンチ

[![Release](https://img.shields.io/badge/Release-v1.10.10-cyan.svg)](https://github.com/saghet/opensec-studio/releases/tag/v1.10.10)
[![開発者](https://img.shields.io/badge/Author-el%20saghet-amber.svg)](https://github.com/saghet/opensec-studio)
[![プラットフォーム](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20x64-blue.svg)](https://github.com/saghet/opensec-studio/releases)
[![エンジン](https://img.shields.io/badge/Engine-Rust%20%7C%20Tauri-orange.svg)](https://tauri.app)
[![防御機構](https://img.shields.io/badge/Defense-Anti--Cracking%20Sentinel-emerald.svg)](manuales/)
[![ライセンス](https://img.shields.io/badge/License-60日間フル機能評価%20%7C%20永続ライセンス-purple.svg)](LICENSE)

**言語を選択:**  
🌐 **[English (US)](README.md)** | **[Español (ES)](README.es.md)** | **[Deutsch (DE)](README.de.md)** | **[Français (FR)](README.fr.md)** | **[Italiano (IT)](README.it.md)** | **[日本語 (JA)](README.ja.md)**

---

> **OpenSec Studio** は、セキュリティエンジニア、ペネトレーションテスター、バグバウンティハンター向けに設計された、**Burp Suite Professional**、**OWASP ZAP**、**Nuclei** に代わる次世代の超軽量・超高速サイバーセキュリティツールです。**el saghet** により**ネイティブRust**で開発され、HTTP/HTTPS MITM通信の傍受・改ざん、高速ファジング、自動脆弱性スキャンを、180ミリ秒未満の瞬時起動と95MB未満の省メモリ設計で実現しています。

</div>

---

## ⚡ 徹底比較: OpenSec Studio vs. Burp Suite Professional

セキュリティ診断の専門家がBurp SuiteからOpenSec Studioへ移行する理由：

| 機能 / 指標 | ☕ Burp Suite Professional | 🛡️ OpenSec Studio (el saghet作) | 優位性・メリット |
| :--- | :---: | :---: | :--- |
| **起動時間** | 8〜15秒 (JVM初期化) | **180ミリ秒未満 (< 180ms)** | ⚡ **50倍高速**な瞬時立ち上がり |
| **メモリ使用量 (アイドル時)** | 850 MB 〜 1.4 GB | **約55 MB** | 🍃 **15倍軽量**、低スペックPCでも快適 |
| **メモリ使用量 (高負荷ファジング)** | 2.5 GB 〜 4.0 GB | **95 MB未満 (< 95 MB)** | 🚀 メモリリークやフリーズが皆無 |
| **実行基盤** | Java Virtual Machine (JVM) | **ネイティブRust (x86_64 Stripped)** | 🔒 メモリ安全性確保、JVM依存脆弱性排除 |
| **MITMプロキシ & インターセプター** | 対応 | **対応 (Tokio非同期コア採用)** | 🔄 パケットの即時転送 (Forward) / 破棄 (Drop) |
| **リピーター & 高速ファザー** | 対応 | **対応 (Intruder: §PAYLOAD§記法)** | 🎯 SQLi, XSS, パストラバーサルリスト内蔵 |
| **YAML脆弱性テンプレート実行** | 外部拡張機能が必要 | **Nuclei互換エンジン標準搭載** | 📜 コミュニティのYAMLルールをそのまま実行可能 |
| **JWTデバッガー & デコーダー** | BAppストアのプラグイン依存 | **ネイティブ標準搭載 (Decoder)** | 🔍 `alg: none` 攻撃等の脆弱性を自動警告 |
| **SARIF v2.1.0 レポート出力** | 制限あり | **ワンクリックでネイティブ出力** | 📈 GitHub Security や CI/CD パイプラインと直結 |
| **イラスト入り日本語マニュアル** | 英語のみ | **完全日本語対応 (全18章)** | 🌍 高解像度ベクターPDF＆HTMLマニュアル付属 |
| **Windows 1コマンド導入** | 重いインストーラーの手動DL | `winget install el-saghet.OpenSecStudio` | 📦 WinGet & Scoop による自動インストール |
| **評価利用期間** | 30日間 (コミュニティ版機能制限) | **60日間 無制限フル機能アクセス** | 💎 全機能が初日から即座に利用可能 |

---

## 🚀 クイックインストール (Windows 10 / 11)

### 方法 1: Windowsパッケージマネージャー (Winget - 推奨)
```powershell
winget install el-saghet.OpenSecStudio
```

### 方法 2: Scoop
```powershell
scoop bucket add opensec https://github.com/saghet/opensec-studio.git
scoop install opensec-studio
```

---

## 📦 公式バイナリダウンロード (GitHub Releases v1.10.10)

| パッケージ | 形式 | ファイルサイズ | 用途・推奨環境 | ダウンロードリンク |
| :--- | :---: | :---: | :--- | :---: |
| **OpenSec Studio セットアップ** | `.exe` | 1.78 MB | デスクトップ＆スタートメニューへの自動ショートカット作成 | [⬇️ .exe をダウンロード](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_OpenSec_Studio_Setup.exe) |
| **企業向けインストーラー (MSI)** | `.msi` | 2.49 MB | 企業LAN、情シス管理者、Active Directory GPO一括配布 | [⬇️ .msi をダウンロード](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/Instalador_Empresarial_OpenSec_Studio.msi) |
| **ポータブル版 (Portable ZIP)** | `.zip` | 16.69 MB | USBメモリ運用、スタンドアロン隔離診断環境 | [⬇️ .zip をダウンロード](https://github.com/saghet/opensec-studio/releases/download/v1.10.10/OpenSec-Studio-v1.10.10-Windows-x64.zip) |

---

## 🔐 暗号学的整合性とSHA-256チェックサム

ダウンロードしたバイナリのハッシュ値を照合して整合性を検証してください：

```text
e11bca47a1a0b648adad882e9f9b37ccb96ba20d40b8cafadc373d0164d6225c  Instalador_OpenSec_Studio_Setup.exe
f226af2852bcf71b6256c3a720a4ae5e52e2171b6b8a10433b05f83eb1c72f0b  Instalador_Empresarial_OpenSec_Studio.msi
abf2b20412765c9e70c39d820df049dfae032a6f081f89371320f6db558530a3  OpenSec Studio.exe (ポータブル単体実行ファイル)
50e493990d255d74e914373eff11dd91646106fe7ebe747804007f75ff1dbe7d  OpenSec-Studio-v1.10.10-Windows-x64.zip
```

PowerShellでの検証コマンド：
```powershell
Get-FileHash -Algorithm SHA256 .\Instalador_OpenSec_Studio_Setup.exe
```

---

## 📖 6言語対応 イラスト入り公式ユーザーマニュアル

「インターネットの大村」という直感的な教育モデルを採用した全18章の完全ガイド：

- 🇯🇵 **[日本語マニュアル (高解像度PDF)](manuales/Manual_OpenSec_Studio_JA_El_Saghet.pdf)** | [HTMLで閲覧](manuales/manual_ja.html)
- 🇺🇸 **[User Manual in English (PDF)](manuales/Manual_OpenSec_Studio_EN_El_Saghet.pdf)** | [HTML](manuales/manual_en.html)
- 🇪🇸 **[Manual en Español (PDF)](manuales/Manual_OpenSec_Studio_ES_El_Saghet.pdf)** | [HTML](manuales/manual_es.html)
- 🇩🇪 **[Benutzerhandbuch auf Deutsch (PDF)](manuales/Manual_OpenSec_Studio_DE_El_Saghet.pdf)** | [HTML](manuales/manual_de.html)
- 🇫🇷 **[Manuel en Français (PDF)](manuales/Manual_OpenSec_Studio_FR_El_Saghet.pdf)** | [HTML](manuales/manual_fr.html)
- 🇮🇹 **[Manuale in Italiano (PDF)](manuales/Manual_OpenSec_Studio_IT_El_Saghet.pdf)** | [HTML](manuales/manual_it.html)

---

## 🌐 公式ウェブサイト

公式Webポータルはこちら：  
👉 **[https://saghet.github.io/opensec-studio](https://saghet.github.io/opensec-studio)**

---

## 👤 開発者＆ライセンス

- **リードアーキテクト＆開発者:** **el saghet**
- **評価期間:** 全機能・全ツール 60日間完全無制限アクセス。
- **コミュニティ支援・永続利用:** 公式Bitcoinアドレスへの自発的寄付を歓迎しています：  
  `bc1q8x4jdf78p223m2x9t82wq5g6c9y00p7k7w8c3a`
