import os
import tempfile
import time
import unittest
from pathlib import Path

from app import analyze_pcap_file, parse_tshark_tsv, purge_expired_pcaps, resolve_safe_pcap_path


class TestPcapAnalyzer(unittest.TestCase):
    def test_path_traversal_blocked(self):
        with tempfile.TemporaryDirectory() as tmp:
            vault = Path(tmp)
            with self.assertRaises(ValueError):
                resolve_safe_pcap_path("../secret.pcap", vault=vault)
            with self.assertRaises(ValueError):
                resolve_safe_pcap_path("malware.exe", vault=vault)

    def test_parse_tshark_tsv_and_deauth_anomaly(self):
        lines = ["radiotap:wlan\t0x0008\taa:bb:cc:11:22:33\tCorpWiFi"]
        lines.extend(["radiotap:wlan\t0x000c\taa:bb:cc:11:22:33\t" for _ in range(25)])
        lines.append("radiotap:wlan:eapol\t0x0008\taa:bb:cc:11:22:33\tCorpWiFi")
        result = parse_tshark_tsv("\n".join(lines))

        self.assertEqual(result["total_frames"], 27)
        self.assertEqual(result["deauth_frames"], 25)
        self.assertEqual(result["eapol_frames"], 1)
        self.assertEqual(result["unique_bssids"], ["aa:bb:cc:11:22:33"])
        self.assertEqual(len(result["anomalies"]), 1)
        self.assertEqual(result["anomalies"][0]["type"], "deauth_disassoc_spike")

    def test_analyze_and_retention_purge(self):
        with tempfile.TemporaryDirectory() as tmp:
            vault = Path(tmp)
            old_pcap = vault / "old.pcapng"
            new_pcap = vault / "new.pcapng"
            old_pcap.write_bytes(b"old_dummy")
            new_pcap.write_bytes(b"new_dummy")

            now = time.time()
            os.utime(old_pcap, (now - 10 * 86400, now - 10 * 86400))
            os.utime(new_pcap, (now, now))

            report = analyze_pcap_file(new_pcap, runner=lambda _: "radiotap:wlan\t0x0008\t11:22:33:44:55:66\tLab\n")
            self.assertEqual(report["filename"], "new.pcapng")
            self.assertIn("Lab", report["technical_report"]["unique_ssids"])

            purged = purge_expired_pcaps(vault=vault, retention_days=7, now_ts=now)
            self.assertEqual(purged, ["old.pcapng"])
            self.assertFalse(old_pcap.exists())
            self.assertTrue(new_pcap.exists())


if __name__ == "__main__":
    unittest.main()
