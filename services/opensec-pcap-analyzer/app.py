import hashlib
import json
import os
import subprocess
import time
from collections import Counter, defaultdict
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional


PCAP_VAULT = Path(os.environ.get("OPENSEC_PCAP_VAULT", "/var/lib/opensec/pcaps"))
RETENTION_DAYS = int(os.environ.get("OPENSEC_PCAP_RETENTION_DAYS", "7"))


def resolve_safe_pcap_path(filename: str, vault: Path = PCAP_VAULT) -> Path:
    if not filename or not filename.lower().endswith((".pcap", ".pcapng")):
        raise ValueError("invalid_extension")
    candidate = (vault / filename).resolve()
    vault_resolved = vault.resolve()
    if vault_resolved not in candidate.parents:
        raise ValueError("path_traversal_blocked")
    if not candidate.is_file():
        raise FileNotFoundError("pcap_not_found")
    return candidate


def _run_tshark(cmd: List[str]) -> str:
    res = subprocess.run(cmd, capture_output=True, text=True, check=False, timeout=60)
    if res.returncode != 0:
        raise RuntimeError(res.stderr.strip() or "tshark_failed")
    return res.stdout


def freq_to_band_and_channel(freq_str: str) -> (Optional[str], Optional[int]):
    try:
        freq = int(float(freq_str))
    except (ValueError, TypeError):
        return None, None
    if 2412 <= freq <= 2484:
        ch = 14 if freq == 2484 else (freq - 2407) // 5
        return "2.4GHz", ch
    if 5000 <= freq < 5900:
        return "5GHz", (freq - 5000) // 5
    if 5955 <= freq <= 7115:
        return "6GHz", (freq - 5950) // 5
    return None, None


def infer_security_from_akms(akms: str, privacy_bit: str, is_beacon_or_probe: bool) -> str:
    if akms:
        parts = {p.strip() for p in akms.replace(";", ",").split(",") if p.strip()}
        has_sae = "8" in parts or "24" in parts
        has_psk = "2" in parts or "4" in parts or "6" in parts
        has_eap = "1" in parts or "3" in parts or "5" in parts
        if has_sae and has_psk:
            return "WPA2/WPA3"
        if has_sae:
            return "WPA3-SAE"
        if has_eap:
            return "WPA2-Enterprise"
        if has_psk:
            return "WPA2-PSK"
        return "WPA2-PSK"
    if is_beacon_or_probe:
        if privacy_bit in ("1", "True", "true"):
            return "WEP"
        if privacy_bit in ("0", "False", "false"):
            return "Open"
    return "Unknown"


def parse_tshark_tsv(tsv_output: str) -> Dict[str, Any]:
    total_frames = 0
    subtypes = Counter()
    protocols = Counter()
    bssids = set()
    ssids = set()
    deauth_count = 0
    disassoc_count = 0
    eapol_count = 0

    networks: Dict[str, Dict[str, Any]] = {}
    clients_by_bssid: Dict[str, set] = defaultdict(set)

    for raw in tsv_output.splitlines():
        if not raw.strip():
            continue
        parts = raw.split("\t")
        # Columns:
        # 0: frame.protocols, 1: wlan.fc.type_subtype, 2: wlan.bssid, 3: wlan.ssid,
        # 4: radiotap.channel.freq, 5: radiotap.dbm_antsignal, 6: wlan.rsn.akms.type,
        # 7: wlan.fixed.capabilities.privacy, 8: wlan.sa, 9: wlan.da
        protos = parts[0] if len(parts) > 0 else ""
        subtype = parts[1].strip() if len(parts) > 1 else ""
        bssid = parts[2].strip().lower() if len(parts) > 2 else ""
        ssid_raw = parts[3].strip() if len(parts) > 3 else ""
        if ssid_raw and len(ssid_raw) % 2 == 0 and all(c in "0123456789abcdefABCDEF" for c in ssid_raw):
            try:
                ssid = bytes.fromhex(ssid_raw).decode("utf-8", errors="replace")
            except ValueError:
                ssid = ssid_raw
        else:
            ssid = ssid_raw
        freq_str = parts[4].strip() if len(parts) > 4 else ""
        rssi_str = parts[5].strip() if len(parts) > 5 else ""
        akms = parts[6].strip() if len(parts) > 6 else ""
        privacy = parts[7].strip() if len(parts) > 7 else ""
        sa = parts[8].strip().lower() if len(parts) > 8 else ""
        da = parts[9].strip().lower() if len(parts) > 9 else ""

        total_frames += 1
        for p in protos.split(":"):
            if p:
                protocols[p] += 1
        if "eapol" in protos.lower():
            eapol_count += 1

        is_deauth = subtype in ("0x000c", "12", "0x0c")
        is_disassoc = subtype in ("0x000a", "10", "0x0a")
        is_beacon = subtype in ("0x0008", "8", "0x08")
        is_probe_resp = subtype in ("0x0005", "5", "0x05")

        if subtype:
            subtypes[subtype] += 1
            if is_deauth:
                deauth_count += 1
            elif is_disassoc:
                disassoc_count += 1

        if bssid:
            bssids.add(bssid)
            net = networks.setdefault(
                bssid,
                {
                    "bssid": bssid,
                    "ssid": "",
                    "band": None,
                    "channel": None,
                    "rssi_dbm": None,
                    "security_observed": "Unknown",
                    "pmf_status": "unknown",
                    "wps_observed": False,
                    "beacon_frames": 0,
                    "deauth_frames": 0,
                    "eapol_frames": 0,
                    "clients": [],
                },
            )
            if ssid and not net["ssid"]:
                net["ssid"] = ssid
            band, ch = freq_to_band_and_channel(freq_str)
            if band and not net["band"]:
                net["band"] = band
            if ch and not net["channel"]:
                net["channel"] = ch
            if rssi_str:
                try:
                    net["rssi_dbm"] = int(float(rssi_str.split(",")[0]))
                except ValueError:
                    pass
            sec = infer_security_from_akms(akms, privacy, is_beacon or is_probe_resp)
            if sec != "Unknown":
                net["security_observed"] = sec
            if is_beacon:
                net["beacon_frames"] += 1
            if is_deauth or is_disassoc:
                net["deauth_frames"] += 1
            if "eapol" in protos.lower():
                net["eapol_frames"] += 1
            for mac in (sa, da):
                if mac and mac != bssid and mac != "ff:ff:ff:ff:ff:ff" and not mac.startswith("33:33:") and not mac.startswith("01:00:5e"):
                    clients_by_bssid[bssid].add(mac)

        if ssid:
            ssids.add(ssid)

    for bssid, net in networks.items():
        net["clients"] = sorted(clients_by_bssid[bssid])

    anomalies = []
    if total_frames > 0 and (deauth_count + disassoc_count) >= 20:
        ratio = round((deauth_count + disassoc_count) / total_frames, 3)
        anomalies.append({
            "type": "deauth_disassoc_spike",
            "severity": "high" if ratio >= 0.1 else "medium",
            "count": deauth_count + disassoc_count,
            "ratio": ratio,
            "explanation": "Elevated deauthentication/disassociation frames observed in capture.",
        })

    return {
        "total_frames": total_frames,
        "unique_bssids": sorted(bssids),
        "unique_ssids": sorted(ssids),
        "protocol_breakdown": dict(protocols.most_common(15)),
        "subtype_counts": dict(subtypes),
        "eapol_frames": eapol_count,
        "deauth_frames": deauth_count,
        "disassoc_frames": disassoc_count,
        "anomalies": anomalies,
        "networks": list(networks.values()),
    }


def analyze_pcap_file(
    pcap_path: Path,
    runner: Callable[[List[str]], str] = _run_tshark,
) -> Dict[str, Any]:
    sha256 = hashlib.sha256(pcap_path.read_bytes()).hexdigest()
    cmd = [
        "tshark",
        "-r",
        str(pcap_path),
        "-T",
        "fields",
        "-e",
        "frame.protocols",
        "-e",
        "wlan.fc.type_subtype",
        "-e",
        "wlan.bssid",
        "-e",
        "wlan.ssid",
        "-e",
        "radiotap.channel.freq",
        "-e",
        "radiotap.dbm_antsignal",
        "-e",
        "wlan.rsn.akms.type",
        "-e",
        "wlan.fixed.capabilities.privacy",
        "-e",
        "wlan.sa",
        "-e",
        "wlan.da",
    ]
    tsv = runner(cmd)
    tech = parse_tshark_tsv(tsv)
    executive_summary = (
        f"Analyzed {tech['total_frames']} frames across {len(tech['unique_bssids'])} BSSIDs "
        f"and {len(tech['unique_ssids'])} SSIDs; detected {len(tech['anomalies'])} anomalies."
    )
    return {
        "filename": pcap_path.name,
        "sha256": sha256,
        "size_bytes": pcap_path.stat().st_size,
        "technical_report": tech,
        "executive_summary": executive_summary,
    }


def purge_expired_pcaps(
    vault: Path = PCAP_VAULT,
    retention_days: int = RETENTION_DAYS,
    now_ts: Optional[float] = None,
) -> List[str]:
    if not vault.exists():
        return []
    now = now_ts if now_ts is not None else time.time()
    cutoff = now - (retention_days * 86400)
    purged = []
    for item in vault.iterdir():
        if item.is_file() and item.name.lower().endswith((".pcap", ".pcapng")):
            if item.stat().st_mtime < cutoff:
                item.unlink()
                purged.append(item.name)
    return purged


class AnalyzerHandler(BaseHTTPRequestHandler):
    def _send_json(self, code: int, payload: Dict[str, Any]):
        body = json.dumps(payload).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        if self.path == "/health":
            res = subprocess.run(["tshark", "--version"], capture_output=True, text=True, check=False)
            self._send_json(200 if res.returncode == 0 else 503, {
                "status": "ok" if res.returncode == 0 else "degraded",
                "service": "opensec-pcap-analyzer",
                "tshark": res.stdout.splitlines()[0] if res.stdout else "unavailable",
            })
        else:
            self._send_json(404, {"error": "not_found"})

    def do_POST(self):
        if self.path == "/analyze":
            length = int(self.headers.get("Content-Length", "0"))
            raw = self.rfile.read(length) if length > 0 else b"{}"
            try:
                data = json.loads(raw.decode("utf-8"))
                pcap_path = resolve_safe_pcap_path(data.get("filename", ""))
                self._send_json(200, analyze_pcap_file(pcap_path))
            except (ValueError, FileNotFoundError) as exc:
                self._send_json(400, {"error": str(exc)})
            except Exception as exc:
                self._send_json(500, {"error": str(exc)})
        elif self.path == "/purge":
            purged = purge_expired_pcaps()
            self._send_json(200, {"purged": purged, "retention_days": RETENTION_DAYS})
        else:
            self._send_json(404, {"error": "not_found"})

    def log_message(self, format, *args):
        print(json.dumps({"service": "opensec-pcap-analyzer", "msg": format % args}))


if __name__ == "__main__":
    HTTPServer(("0.0.0.0", 8090), AnalyzerHandler).serve_forever()
