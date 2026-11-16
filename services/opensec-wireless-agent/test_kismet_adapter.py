import unittest
from kismet_adapter import KismetClient, normalize_kismet_alert, normalize_kismet_device

SAMPLE_DEVICE = {
    "kismet.device.base.type": "Wi-Fi AP",
    "kismet.device.base.macaddr": "AA:BB:CC:11:22:33",
    "kismet.device.base.manuf": "Ubiquiti Inc",
    "kismet.device.base.channel": "36",
    "kismet.device.base.frequency": 5180000,
    "kismet.device.base.signal": {"kismet.common.signal.last_signal": -54},
    "dot11.device": {
        "dot11.device.last_beaconed_ssid": "CorpNet-5G",
        "dot11.device.advertised_ssid_map": [
            {
                "dot11.advertisedssid.ssid": "CorpNet-5G",
                "dot11.advertisedssid.crypt_string": "WPA2+PSK WPA3+SAE",
                "dot11.advertisedssid.wps_state": 0,
            }
        ],
        "dot11.device.associated_client_map": {
            "11:22:33:44:55:66": {}
        },
    },
}

SAMPLE_ALERT = {
    "kismet.alert.header": "DEAUTHFLOOD",
    "kismet.alert.severity": 15,
    "kismet.alert.transmitter_mac": "AA:BB:CC:11:22:33",
    "kismet.alert.text": "Observed elevated deauthentication frame rate",
    "kismet.alert.timestamp": 1730000000.5,
}


class TestKismetAdapter(unittest.TestCase):
    def test_normalize_device(self):
        net = normalize_kismet_device(SAMPLE_DEVICE)
        self.assertIsNotNone(net)
        self.assertEqual(net.bssid, "aa:bb:cc:11:22:33")
        self.assertEqual(net.ssid, "CorpNet-5G")
        self.assertEqual(net.band, "5GHz")
        self.assertEqual(net.channel, 36)
        self.assertEqual(net.rssi_dbm, -54)
        self.assertEqual(net.security_observed, "WPA2/WPA3")
        self.assertFalse(net.wps_observed)
        self.assertEqual(net.associated_clients, ["11:22:33:44:55:66"])

    def test_normalize_alert(self):
        alert = normalize_kismet_alert(SAMPLE_ALERT)
        self.assertEqual(alert.rule_id, "kismet:DEAUTHFLOOD")
        self.assertEqual(alert.severity, "high")
        self.assertEqual(alert.bssid, "aa:bb:cc:11:22:33")

    def test_kill_switch_blocks_polling(self):
        client = KismetClient("http://127.0.0.1:2501", kill_switch=True)
        with self.assertRaises(RuntimeError):
            client.fetch_networks_since(0)


if __name__ == "__main__":
    unittest.main()
