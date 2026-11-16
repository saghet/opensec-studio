import json
from dataclasses import asdict, dataclass, field
from typing import Any, Dict, List, Optional
from urllib.error import URLError
from urllib.request import Request, urlopen


@dataclass
class NormalizedNetwork:
    bssid: str
    ssid: str
    oui_vendor: Optional[str]
    band: Optional[str]
    channel: Optional[int]
    rssi_dbm: Optional[int]
    security_observed: str
    wps_observed: bool
    associated_clients: List[str] = field(default_factory=list)
    raw_crypt_string: Optional[str] = None


@dataclass
class NormalizedKismetAlert:
    rule_id: str
    header: str
    severity: str
    bssid: Optional[str]
    explanation: str
    timestamp: float


def frequency_to_band(freq_khz_or_mhz: Optional[float]) -> Optional[str]:
    if not freq_khz_or_mhz:
        return None
    freq_mhz = freq_khz_or_mhz / 1000.0 if freq_khz_or_mhz > 10000 else float(freq_khz_or_mhz)
    if 2400 <= freq_mhz < 2500:
        return "2.4GHz"
    if 5000 <= freq_mhz < 5900:
        return "5GHz"
    if 5925 <= freq_mhz <= 7125:
        return "6GHz"
    return None


def classify_security(crypt_string: Optional[str]) -> str:
    if not crypt_string:
        return "Unknown"
    upper = crypt_string.upper()
    if "NONE" in upper or upper.strip() == "OPEN":
        return "Open"
    if "WEP" in upper:
        return "WEP"
    has_sae = "SAE" in upper or "WPA3" in upper
    has_psk = "PSK" in upper or "WPA2" in upper
    has_eap = "EAP" in upper or "802.1X" in upper
    if has_sae and has_psk:
        return "WPA2/WPA3"
    if has_sae:
        return "WPA3-SAE"
    if has_eap:
        return "WPA2-Enterprise"
    if has_psk:
        return "WPA2-PSK"
    return crypt_string


def normalize_kismet_device(dev: Dict[str, Any]) -> Optional[NormalizedNetwork]:
    dev_type = dev.get("kismet.device.base.type", "")
    if dev_type and "Wi-Fi AP" not in dev_type and "802.11" not in dev_type:
        return None

    bssid = (dev.get("kismet.device.base.macaddr") or "").lower().strip()
    if not bssid:
        return None

    dot11 = dev.get("dot11.device", {})
    ssid_map = dot11.get("dot11.device.advertised_ssid_map") or []
    first_ssid = ssid_map[0] if isinstance(ssid_map, list) and ssid_map else {}

    ssid = (
        first_ssid.get("dot11.advertisedssid.ssid")
        or dot11.get("dot11.device.last_beaconed_ssid")
        or ""
    )
    crypt_str = first_ssid.get("dot11.advertisedssid.crypt_string")
    wps_state = bool(first_ssid.get("dot11.advertisedssid.wps_state", 0))

    signal = dev.get("kismet.device.base.signal", {})
    rssi = signal.get("kismet.common.signal.last_signal")

    channel_raw = dev.get("kismet.device.base.channel")
    try:
        channel = int(channel_raw) if channel_raw not in (None, "") else None
    except ValueError:
        channel = None

    freq = dev.get("kismet.device.base.frequency")
    clients_map = dot11.get("dot11.device.associated_client_map") or {}
    clients = sorted(k.lower() for k in clients_map.keys()) if isinstance(clients_map, dict) else []

    return NormalizedNetwork(
        bssid=bssid,
        ssid=ssid,
        oui_vendor=dev.get("kismet.device.base.manuf"),
        band=frequency_to_band(freq),
        channel=channel,
        rssi_dbm=int(rssi) if rssi is not None else None,
        security_observed=classify_security(crypt_str),
        wps_observed=wps_state,
        associated_clients=clients,
        raw_crypt_string=crypt_str,
    )


def normalize_kismet_alert(alert: Dict[str, Any]) -> NormalizedKismetAlert:
    sev_num = int(alert.get("kismet.alert.severity", 10))
    severity = "critical" if sev_num >= 20 else "high" if sev_num >= 15 else "medium" if sev_num >= 10 else "low"
    return NormalizedKismetAlert(
        rule_id=f"kismet:{alert.get('kismet.alert.header', 'UNKNOWN')}",
        header=alert.get("kismet.alert.header", "UNKNOWN"),
        severity=severity,
        bssid=(alert.get("kismet.alert.transmitter_mac") or "").lower() or None,
        explanation=alert.get("kismet.alert.text", ""),
        timestamp=float(alert.get("kismet.alert.timestamp", 0.0)),
    )


class KismetClient:
    def __init__(self, base_url: str, api_token: Optional[str] = None, kill_switch: bool = False):
        self.base_url = base_url.rstrip("/")
        self.api_token = api_token
        self.kill_switch = kill_switch

    def _get_json(self, path: str) -> Any:
        if self.kill_switch:
            raise RuntimeError("Kill switch active: Kismet ingestion halted")
        req = Request(f"{self.base_url}{path}")
        if self.api_token:
            req.add_header("KISMET", self.api_token)
        try:
            with urlopen(req, timeout=5) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except URLError as exc:
            raise ConnectionError(f"Kismet unreachable at {self.base_url}: {exc}") from exc

    def fetch_networks_since(self, last_ts: int = 0) -> List[dict]:
        raw = self._get_json(f"/devices/last-time/{last_ts}/devices.json")
        out = []
        for dev in raw:
            norm = normalize_kismet_device(dev)
            if norm:
                out.append(asdict(norm))
        return out

    def fetch_alerts_since(self, last_ts: int = 0) -> List[dict]:
        raw = self._get_json(f"/alerts/last-time/{last_ts}/alerts.json")
        return [asdict(normalize_kismet_alert(a)) for a in raw]
