use crate::assessment::Severity;

#[derive(Debug, Clone)]
pub struct WidsObservation {
    pub sensor_id: String,
    pub ssid: String,
    pub bssid: String,
    pub channel: u16,
    pub security: String,
    pub rssi_dbm: i32,
    pub timestamp_sec: u64,
    pub is_authorized_bssid: bool,
    pub is_known_org_ssid: bool,
    pub deauth_frames_in_window: u32,
    pub beacon_frames_in_window: u32,
}

#[derive(Debug, Clone)]
pub struct WidsAlert {
    pub rule_id: &'static str,
    pub category: &'static str,
    pub severity: Severity,
    pub confidence: f32,
    pub sensor_id: String,
    pub bssid: String,
    pub dedup_key: String,
    pub timestamp_sec: u64,
    pub evidence: Vec<String>,
    pub explanation: String,
}

pub fn evaluate_wids_observation(
    obs: &WidsObservation,
    baseline_channel: Option<u16>,
    baseline_security: Option<&str>,
) -> Vec<WidsAlert> {
    let mut alerts = Vec::new();
    let window_bucket = obs.timestamp_sec / 300;

    // Multi-indicator Evil Twin suspicion: NEVER trigger solely on shared SSID.
    if obs.is_known_org_ssid && !obs.is_authorized_bssid {
        let mut indicators = vec![
            format!("unauthorized_bssid={}", obs.bssid),
            format!("protected_ssid={}", obs.ssid),
        ];
        let mut score: f32 = 0.45;

        if let Some(base_sec) = baseline_security {
            if base_sec != obs.security {
                indicators.push(format!("security_mismatch={}!=baseline({})", obs.security, base_sec));
                score += 0.30;
            }
        }
        if let Some(base_ch) = baseline_channel {
            if base_ch != obs.channel {
                indicators.push(format!("channel_mismatch={}!=baseline({})", obs.channel, base_ch));
                score += 0.15;
            }
        }

        if indicators.len() >= 3 {
            alerts.push(WidsAlert {
                rule_id: "WIDS-EVILTWIN-001",
                category: "evil_twin_suspect",
                severity: Severity::Critical,
                confidence: score.min(0.99),
                sensor_id: obs.sensor_id.clone(),
                bssid: obs.bssid.clone(),
                dedup_key: format!("WIDS-EVILTWIN-001:{}:{}", obs.bssid, window_bucket),
                timestamp_sec: obs.timestamp_sec,
                evidence: indicators,
                explanation: "Protected organization SSID advertised by unauthorized BSSID with additional anomalous indicators.".to_string(),
            });
        } else {
            alerts.push(WidsAlert {
                rule_id: "WIDS-ROGUE-002",
                category: "unauthorized_bssid",
                severity: Severity::Medium,
                confidence: 0.70,
                sensor_id: obs.sensor_id.clone(),
                bssid: obs.bssid.clone(),
                dedup_key: format!("WIDS-ROGUE-002:{}:{}", obs.bssid, window_bucket),
                timestamp_sec: obs.timestamp_sec,
                evidence: indicators,
                explanation: "Known SSID observed from an unlisted BSSID without secondary spoofing indicators (could be uninventoried mesh/repeater).".to_string(),
            });
        }
    }

    if obs.deauth_frames_in_window >= 25 {
        alerts.push(WidsAlert {
            rule_id: "WIDS-DEAUTH-003",
            category: "deauth_storm",
            severity: Severity::High,
            confidence: 0.90,
            sensor_id: obs.sensor_id.clone(),
            bssid: obs.bssid.clone(),
            dedup_key: format!("WIDS-DEAUTH-003:{}:{}", obs.bssid, window_bucket),
            timestamp_sec: obs.timestamp_sec,
            evidence: vec![format!("deauth_frames_window={}", obs.deauth_frames_in_window)],
            explanation: "Passive observation detected elevated deauthentication frame rate.".to_string(),
        });
    }

    if obs.beacon_frames_in_window >= 500 {
        alerts.push(WidsAlert {
            rule_id: "WIDS-BEACON-004",
            category: "beacon_flood",
            severity: Severity::Medium,
            confidence: 0.80,
            sensor_id: obs.sensor_id.clone(),
            bssid: obs.bssid.clone(),
            dedup_key: format!("WIDS-BEACON-004:{}:{}", obs.sensor_id, window_bucket),
            timestamp_sec: obs.timestamp_sec,
            evidence: vec![format!("beacon_frames_window={}", obs.beacon_frames_in_window)],
            explanation: "Abnormal beacon burst detected passively in sensor window.".to_string(),
        });
    }

    alerts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_ssid_alone_does_not_classify_as_evil_twin() {
        let obs = WidsObservation {
            sensor_id: "sensor-01".to_string(),
            ssid: "CorpNet".to_string(),
            bssid: "de:ad:be:ef:00:01".to_string(),
            channel: 36,
            security: "WPA2-Enterprise".to_string(),
            rssi_dbm: -62,
            timestamp_sec: 1730000100,
            is_authorized_bssid: false,
            is_known_org_ssid: true,
            deauth_frames_in_window: 0,
            beacon_frames_in_window: 10,
        };
        let alerts = evaluate_wids_observation(&obs, Some(36), Some("WPA2-Enterprise"));
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].category, "unauthorized_bssid");
    }

    #[test]
    fn multi_indicator_triggers_evil_twin_suspect() {
        let obs = WidsObservation {
            sensor_id: "sensor-01".to_string(),
            ssid: "CorpNet".to_string(),
            bssid: "de:ad:be:ef:00:02".to_string(),
            channel: 6,
            security: "Open".to_string(),
            rssi_dbm: -45,
            timestamp_sec: 1730000100,
            is_authorized_bssid: false,
            is_known_org_ssid: true,
            deauth_frames_in_window: 40,
            beacon_frames_in_window: 12,
        };
        let alerts = evaluate_wids_observation(&obs, Some(36), Some("WPA2-Enterprise"));
        assert!(alerts.iter().any(|a| a.category == "evil_twin_suspect" && a.confidence >= 0.85));
        assert!(alerts.iter().any(|a| a.category == "deauth_storm"));
    }
}
