# Show HN: OpenSec Studio – An ultra-fast, native DAST & security workbench in Rust (Burp Suite alternative)

**Submitted to:** news.ycombinator.com (Show HN)  
**Author:** el saghet  
**Project Link:** https://github.com/saghet/opensec-studio  

---

### Title:
> **Show HN: OpenSec Studio – A fast, native DAST & HTTP proxy in Rust (Burp Suite alternative)**

---

### Body Post:

Hey Hacker News,

I'm **el saghet**, creator of **OpenSec Studio** (https://github.com/saghet/opensec-studio).

For over a decade, security auditors, penetration testers, and bug bounty hunters have had to rely on Java-based desktop tooling. While tools like Burp Suite and OWASP ZAP are feature-rich, they suffer from well-known pain points:
- 15 to 25 seconds startup times.
- 1.5 GB to 3.5 GB of RAM consumption on idle.
- Fan-spinning CPU throttling during high-concurrency fuzzing.
- $449/year licensing pricing that prices out independent researchers, students, and practitioners worldwide.

I built **OpenSec Studio** from scratch to rethink what a modern security workbench should feel like in 2026:

### Key Technical Architecture:
1. **Native Rust + Tauri Engine:**
   - Cold startup in **< 180 ms** on standard Windows machines.
   - Idles at **~55 MB of RAM** (under 95 MB during active scanning).
   - High-throughput asynchronous HTTP socket engine built with Tokio and native TLS (no JVM overhead).

2. **Full DevSecOps & Offensive Security Suite:**
   - **MITM HTTPS Interceptor:** Live proxy on 127.0.0.1:8080 with automated on-the-fly CA certificate generation.
   - **Passive & Active DAST Scanner:** Immediate heuristic detection of SQLi, Reflected/DOM XSS, sensitive directory exposure, and misconfigurations with OASIS SARIF v2.1.0 output.
   - **Nuclei YAML Compatible Engine:** Direct execution of declarative YAML templates for automated vulnerability probing.
   - **Native Repeater & Intruder:** Multi-tab raw HTTP editor with instant re-transmission and payload injection (Sniper, Battering Ram, Pitchfork, Cluster Bomb).
   - **Real OAST Collaborator:** Real TCP, DNS, and HTTP socket listeners to detect blind/out-of-band vulnerabilities (SSRF, Blind XXE).
   - **Air-Gapped Ready:** Bundled offline CVE and vulnerability database feeds, fully usable in air-gapped or high-security networks with zero internet access.

3. **Pedagogical Illustrated Manuals in 6 Languages:**
   Rather than dry 500-page specs, I authored complete illustrated user manuals covering all 18 modules through the pedagogical lens of *"The Great Internet Village"* (postmen, magical photocopiers, customs guards). Available in **English, Spanish, German, French, Italian, and Japanese** (high-res vector PDFs).

4. **License & Philosophy:**
   Every user gets **60 days of completely unrestricted full evaluation**. Reinstallation does not reset the clock (node-locked hardware vault). Afterwards, continuing use or supporting independent development is accessible via a voluntary $30 USD Bitcoin contribution.

The Windows 64-bit release (1-click installer and enterprise MSI) is available on GitHub Releases:
https://github.com/saghet/opensec-studio/releases/tag/v1.10.10

Or install in one line via Winget:
`winget install el-saghet.OpenSecStudio`

I would love to get your feedback on performance, proxy stability, and features you'd like to see next!
