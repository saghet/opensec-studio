#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingNature {
    ObservedConfig,
    InferredConfig,
    ProvenVulnerability,
    PotentialRisk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PmfStatus {
    Required,
    Capable,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct NetworkPostureInput {
    pub ssid: String,
    pub bssid: String,
    pub security_observed: String,
    pub pmf: PmfStatus,
    pub wps_observed: bool,
    pub is_authorized: bool,
    pub previous_security: Option<String>,
    pub eap_evidence_observed: bool,
}

#[derive(Debug, Clone)]
pub struct AssessmentFinding {
    pub rule_id: &'static str,
    pub nature: FindingNature,
    pub severity: Severity,
    pub confidence: f32,
    pub title: String,
    pub explanation: String,
    pub remediation: String,
}

pub fn assess_network_posture(net: &NetworkPostureInput) -> Vec<AssessmentFinding> {
    let mut findings = Vec::new();
    let sec = net.security_observed.to_uppercase();

    if sec == "OPEN" || sec == "NONE" {
        findings.push(AssessmentFinding {
            rule_id: "ASSESS-OPEN-001",
            nature: FindingNature::ProvenVulnerability,
            severity: if net.is_authorized { Severity::High } else { Severity::Medium },
            confidence: 0.95,
            title: "Open unencrypted wireless network".to_string(),
            explanation: format!("BSSID {} ({}) advertises no link-layer encryption.", net.bssid, net.ssid),
            remediation: "Enable WPA3-SAE or WPA2-Enterprise with 802.1X.".to_string(),
        });
    } else if sec.contains("WEP") {
        findings.push(AssessmentFinding {
            rule_id: "ASSESS-WEP-002",
            nature: FindingNature::ProvenVulnerability,
            severity: Severity::Critical,
            confidence: 0.98,
            title: "Deprecated WEP encryption observed".to_string(),
            explanation: format!("BSSID {} uses legacy WEP which is cryptographically broken.", net.bssid),
            remediation: "Decommission WEP immediately and migrate to WPA3-SAE or WPA2-AES (CCMP).".to_string(),
        });
    } else if sec == "WPA2/WPA3" {
        findings.push(AssessmentFinding {
            rule_id: "ASSESS-TRANSITION-003",
            nature: FindingNature::PotentialRisk,
            severity: Severity::Medium,
            confidence: 0.85,
            title: "WPA2/WPA3 Transition Mode enabled".to_string(),
            explanation: "Transition mode allows legacy WPA2-PSK association, creating potential downgrade exposure.".to_string(),
            remediation: "Plan migration to pure WPA3-SAE with PMF required where clients support 802.11w.".to_string(),
        });
    } else if sec.contains("ENTERPRISE") && !net.eap_evidence_observed {
        findings.push(AssessmentFinding {
            rule_id: "ASSESS-ENT-004",
            nature: FindingNature::InferredConfig,
            severity: Severity::Info,
            confidence: 0.70,
            title: "802.1X/EAP Enterprise advertised (EAP exchange not yet observed)".to_string(),
            explanation: "Beacon advertises Enterprise AKM; inner EAP method cannot be verified from beacons alone.".to_string(),
            remediation: "Verify RADIUS server certificate validation and enforce PEAP/EAP-TLS on managed endpoints.".to_string(),
        });
    }

    if net.pmf == PmfStatus::Disabled && !sec.contains("OPEN") && !sec.contains("WEP") {
        findings.push(AssessmentFinding {
            rule_id: "ASSESS-PMF-005",
            nature: FindingNature::ObservedConfig,
            severity: Severity::Medium,
            confidence: 0.90,
            title: "Protected Management Frames (802.11w / PMF) disabled".to_string(),
            explanation: "Management frames are unprotected, increasing exposure to spoofed deauthentication/disassociation.".to_string(),
            remediation: "Enable PMF (802.11w) as Capable or Required on the WLAN controller.".to_string(),
        });
    }

    if net.wps_observed {
        findings.push(AssessmentFinding {
            rule_id: "ASSESS-WPS-006",
            nature: FindingNature::PotentialRisk,
            severity: Severity::Medium,
            confidence: 0.85,
            title: "Wi-Fi Protected Setup (WPS) advertised".to_string(),
            explanation: "WPS Information Element is present in beacons; PIN-based WPS increases brute-force risk.".to_string(),
            remediation: "Disable WPS on the access point unless strictly required.".to_string(),
        });
    }

    if let Some(prev) = &net.previous_security {
        if prev != &net.security_observed {
            findings.push(AssessmentFinding {
                rule_id: "ASSESS-CHANGE-007",
                nature: FindingNature::ObservedConfig,
                severity: Severity::High,
                confidence: 0.90,
                title: "Security posture change detected".to_string(),
                explanation: format!("Security changed from '{}' to '{}' on {}.", prev, net.security_observed, net.bssid),
                remediation: "Verify whether the WLAN security profile change was authorized.".to_string(),
            });
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishes_observed_potential_and_proven_findings() {
        let net = NetworkPostureInput {
            ssid: "Corp-WLAN".to_string(),
            bssid: "aa:bb:cc:11:22:33".to_string(),
            security_observed: "WPA2/WPA3".to_string(),
            pmf: PmfStatus::Disabled,
            wps_observed: true,
            is_authorized: true,
            previous_security: Some("WPA3-SAE".to_string()),
            eap_evidence_observed: false,
        };
        let findings = assess_network_posture(&net);
        assert!(findings.iter().any(|f| f.rule_id == "ASSESS-TRANSITION-003" && f.nature == FindingNature::PotentialRisk));
        assert!(findings.iter().any(|f| f.rule_id == "ASSESS-PMF-005" && f.nature == FindingNature::ObservedConfig));
        assert!(findings.iter().any(|f| f.rule_id == "ASSESS-CHANGE-007" && f.severity == Severity::High));
    }

    #[test]
    fn enterprise_without_eap_frames_is_inferred_not_vulnerable() {
        let net = NetworkPostureInput {
            ssid: "Corp-8021X".to_string(),
            bssid: "aa:bb:cc:11:22:44".to_string(),
            security_observed: "WPA2-Enterprise".to_string(),
            pmf: PmfStatus::Required,
            wps_observed: false,
            is_authorized: true,
            previous_security: None,
            eap_evidence_observed: false,
        };
        let findings = assess_network_posture(&net);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].nature, FindingNature::InferredConfig);
        assert_eq!(findings[0].severity, Severity::Info);
    }
}
