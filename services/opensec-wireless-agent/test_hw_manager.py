import unittest
from hw_manager import probe_all_adapters

SAMPLE_IW_DEV = """\
phy#0
\tInterface wlan0
\t\tifindex 3
\t\twdev 0x1
\t\taddr 00:c0:ca:99:11:22
\t\ttype managed
phy#1
\tInterface wlan1
\t\tifindex 4
\t\twdev 0x100000001
\t\taddr dc:a6:32:aa:bb:cc
\t\ttype managed
"""

SAMPLE_IW_LIST = """\
Wiphy phy0
\tSupported interface modes:
\t\t * managed
\t\t * monitor
\tBand 1:
\t\tFrequencies:
\t\t\t* 2412 MHz [1] (20.0 dBm)
\t\t\t* 2437 MHz [6] (20.0 dBm)
\tBand 2:
\t\tFrequencies:
\t\t\t* 5180 MHz [36] (20.0 dBm)
\t\t\t* 5260 MHz [52] (20.0 dBm) (disabled)
Wiphy phy1
\tSupported interface modes:
\t\t * managed
\tBand 1:
\t\tFrequencies:
\t\t\t* 2412 MHz [1] (20.0 dBm)
"""

SAMPLE_REG = "global\ncountry US: DFS-FCC\n"


def fake_runner(cmd):
    if cmd == ["iw", "dev"]:
        return SAMPLE_IW_DEV
    if cmd == ["iw", "list"]:
        return SAMPLE_IW_LIST
    if cmd == ["iw", "reg", "get"]:
        return SAMPLE_REG
    if cmd == ["ethtool", "-i", "wlan0"]:
        return "driver: mt76x2u\nfirmware-version: 1.0\n"
    if cmd == ["ethtool", "-i", "wlan1"]:
        return "driver: brcmfmac\nfirmware-version: 7.45.202\n"
    return ""


class TestHardwareManager(unittest.TestCase):
    def test_probe_adapters_and_limitations(self):
        adapters = probe_all_adapters(runner=fake_runner)
        self.assertEqual(len(adapters), 2)

        mt = adapters[0]
        self.assertEqual(mt.interface_name, "wlan0")
        self.assertEqual(mt.driver, "mt76x2u")
        self.assertTrue(mt.supports_monitor_mode)
        self.assertEqual(mt.supported_bands, ["2.4GHz", "5GHz"])
        self.assertEqual(mt.available_channels, [1, 6, 36])
        self.assertEqual(mt.reg_domain, "US")
        self.assertEqual(mt.limitations, [])

        brcm = adapters[1]
        self.assertEqual(brcm.interface_name, "wlan1")
        self.assertFalse(brcm.supports_monitor_mode)
        self.assertIn("monitor_mode_not_supported_by_driver_or_firmware", brcm.limitations)
        self.assertIn("brcmfmac_requires_custom_firmware_for_monitor_mode", brcm.limitations)


if __name__ == "__main__":
    unittest.main()
