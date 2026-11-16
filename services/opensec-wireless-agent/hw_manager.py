import json
import os
import re
import subprocess
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Callable, List, Optional


@dataclass
class AdapterCapabilityReport:
    interface_name: str
    phy: Optional[str]
    mac_address: Optional[str]
    driver: Optional[str]
    firmware_version: Optional[str]
    vendor_device: Optional[str]
    supports_monitor_mode: bool
    supported_bands: List[str] = field(default_factory=list)
    available_channels: List[int] = field(default_factory=list)
    reg_domain: Optional[str] = None
    limitations: List[str] = field(default_factory=list)


def _run_cmd(cmd: List[str]) -> str:
    try:
        res = subprocess.run(cmd, capture_output=True, text=True, check=False, timeout=5)
        return res.stdout if res.returncode == 0 else ""
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return ""


def parse_iw_dev(output: str) -> List[dict]:
    adapters = []
    current_phy = None
    current_iface = None
    for raw in output.splitlines():
        line = raw.strip()
        if line.startswith("phy#"):
            current_phy = line.replace("#", "")
        elif line.startswith("Interface "):
            if current_iface:
                adapters.append(current_iface)
            current_iface = {
                "interface_name": line.split(" ", 1)[1].strip(),
                "phy": current_phy,
                "mac_address": None,
            }
        elif current_iface and line.startswith("addr "):
            current_iface["mac_address"] = line.split(" ", 1)[1].strip()
    if current_iface:
        adapters.append(current_iface)
    return adapters


def parse_ethtool_i(output: str) -> dict:
    driver = None
    firmware = None
    for line in output.splitlines():
        if line.startswith("driver:"):
            driver = line.split(":", 1)[1].strip() or None
        elif line.startswith("firmware-version:"):
            firmware = line.split(":", 1)[1].strip() or None
    return {"driver": driver, "firmware_version": firmware}


def parse_iw_reg_get(output: str) -> Optional[str]:
    m = re.search(r"country\s+([A-Z0-9]{2}):", output)
    return m.group(1) if m else None


def parse_iw_list_for_phy(output: str, phy: Optional[str]) -> dict:
    supports_monitor = False
    bands = set()
    channels = set()

    # Split by "Wiphy " blocks if multiple PHYs exist
    blocks = re.split(r"(?m)^Wiphy\s+", output)
    target_block = output
    if phy:
        for b in blocks:
            if b.startswith(phy):
                target_block = b
                break

    in_modes = False
    in_freqs = False
    for raw in target_block.splitlines():
        stripped = raw.strip()
        if stripped.startswith("Supported interface modes:"):
            in_modes = True
            in_freqs = False
            continue
        if stripped.startswith("Frequencies:"):
            in_freqs = True
            in_modes = False
            continue
        if stripped.startswith(("Band ", "Supported commands:", "valid interface combinations:")):
            in_modes = False
            in_freqs = False

        if in_modes and stripped.startswith("* "):
            mode = stripped[2:].strip()
            if mode == "monitor":
                supports_monitor = True

        if in_freqs and "(disabled)" not in stripped:
            m = re.search(r"\*\s+(\d+(?:\.\d+)?)\s+MHz\s+\[(\d+)\]", stripped)
            if m:
                freq = float(m.group(1))
                ch = int(m.group(2))
                channels.add(ch)
                if 2400 <= freq < 2500:
                    bands.add("2.4GHz")
                elif 5000 <= freq < 5900:
                    bands.add("5GHz")
                elif 5925 <= freq <= 7125:
                    bands.add("6GHz")

    return {
        "supports_monitor_mode": supports_monitor,
        "supported_bands": sorted(bands),
        "available_channels": sorted(channels),
    }


def detect_limitations(driver: Optional[str], supports_monitor: bool, bands: List[str]) -> List[str]:
    limits = []
    if not supports_monitor:
        limits.append("monitor_mode_not_supported_by_driver_or_firmware")
    if driver == "iwlwifi":
        limits.append("iwlwifi_client_firmware_may_limit_passive_multichannel_capture")
    if driver == "brcmfmac":
        limits.append("brcmfmac_requires_custom_firmware_for_monitor_mode")
    if not bands:
        limits.append("no_enabled_regulatory_channels_detected")
    return limits


def probe_all_adapters(runner: Callable[[List[str]], str] = _run_cmd) -> List[AdapterCapabilityReport]:
    iw_dev_out = runner(["iw", "dev"])
    if not iw_dev_out:
        return []

    iw_list_out = runner(["iw", "list"])
    reg_domain = parse_iw_reg_get(runner(["iw", "reg", "get"]))
    reports: List[AdapterCapabilityReport] = []

    for item in parse_iw_dev(iw_dev_out):
        iface = item["interface_name"]
        phy = item["phy"]
        eth = parse_ethtool_i(runner(["ethtool", "-i", iface]))
        caps = parse_iw_list_for_phy(iw_list_out, phy)

        vendor_device = None
        uevent_path = Path(f"/sys/class/net/{iface}/device/uevent")
        if uevent_path.exists():
            for line in uevent_path.read_text(errors="ignore").splitlines():
                if line.startswith(("PCI_ID=", "PRODUCT=", "MODALIAS=")):
                    vendor_device = line
                    break

        limitations = detect_limitations(
            eth["driver"], caps["supports_monitor_mode"], caps["supported_bands"]
        )
        reports.append(
            AdapterCapabilityReport(
                interface_name=iface,
                phy=phy,
                mac_address=item["mac_address"],
                driver=eth["driver"],
                firmware_version=eth["firmware_version"],
                vendor_device=vendor_device,
                supports_monitor_mode=caps["supports_monitor_mode"],
                supported_bands=caps["supported_bands"],
                available_channels=caps["available_channels"],
                reg_domain=reg_domain,
                limitations=limitations,
            )
        )
    return reports


if __name__ == "__main__":
    results = [asdict(r) for r in probe_all_adapters()]
    print(json.dumps({"service": "opensec-wireless-agent", "adapters": results}, indent=2))
