use crate::assessment::{AssessmentFinding, Severity};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RiskInput {
    pub network_criticality: f32, // 0.5 (guest) .. 1.5 (corp)
    pub exposure_factor: f32,     // 0.5 .. 1.2
    pub findings: Vec<AssessmentFinding>,
}

#[derive(Debug, Clone)]
pub struct RiskScoreResult {
    pub score: f32, // 0.0 .. 100.0
    pub deduplicated_count: usize,
    pub explanation: String,
    pub top_mitigation: String,
}

fn severity_weight(sev: &Severity) -> f32 {
    match sev {
        Severity::Critical => 40.0,
        Severity::High => 25.0,
        Severity::Medium => 12.0,
        Severity::Low => 5.0,
        Severity::Info => 0.0,
    }
}

pub fn calculate_risk_score(input: &RiskInput) -> RiskScoreResult {
    let mut seen_rules = HashSet::new();
    let mut raw_score: f32 = 0.0;
    let mut top_sev_weight: f32 = -1.0;
    let mut top_mitigation = "Continue passive monitoring.".to_string();

    for f in &input.findings {
        if !seen_rules.insert(f.rule_id) {
            continue;
        }
        let w = severity_weight(&f.severity);
        raw_score += w * f.confidence;
        if w > top_sev_weight {
            top_sev_weight = w;
            top_mitigation = f.remediation.clone();
        }
    }

    let scaled = (raw_score * input.network_criticality * input.exposure_factor).clamp(0.0, 100.0);
    RiskScoreResult {
        score: (scaled * 100.0).round() / 100.0,
        deduplicated_count: seen_rules.len(),
        explanation: format!(
            "Score derived from {} unique findings weighted by confidence, criticality ({:.2}), and exposure ({:.2}).",
            seen_rules.len(),
            input.network_criticality,
            input.exposure_factor
        ),
        top_mitigation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assessment::FindingNature;

    #[test]
    fn deduplicates_identical_rule_ids_and_caps_at_100() {
        let finding = AssessmentFinding {
            rule_id: "ASSESS-WEP-002",
            nature: FindingNature::ProvenVulnerability,
            severity: Severity::Critical,
            confidence: 0.98,
            title: "WEP".into(),
            explanation: "WEP".into(),
            remediation: "Migrate to WPA3-SAE".into(),
        };
        let res = calculate_risk_score(&RiskInput {
            network_criticality: 1.5,
            exposure_factor: 1.0,
            findings: vec![finding.clone(), finding],
        });
        assert_eq!(res.deduplicated_count, 1);
        assert!(res.score > 50.0 && res.score <= 100.0);
        assert_eq!(res.top_mitigation, "Migrate to WPA3-SAE");
    }
}
