use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct RfSample {
    pub band: String,
    pub channel: u16,
    pub rssi_dbm: i32,
}

#[derive(Debug, Clone)]
pub struct RfSummary {
    pub total_aps_sampled: usize,
    pub band_distribution: BTreeMap<String, usize>,
    pub channel_density: BTreeMap<u16, usize>,
    pub congested_24ghz_channels: Vec<u16>,
    pub note: &'static str,
}

pub fn analyze_rf_environment(samples: &[RfSample]) -> RfSummary {
    let mut band_distribution: BTreeMap<String, usize> = BTreeMap::new();
    let mut channel_density: BTreeMap<u16, usize> = BTreeMap::new();

    for s in samples {
        *band_distribution.entry(s.band.clone()).or_insert(0) += 1;
        *channel_density.entry(s.channel).or_insert(0) += 1;
    }

    let congested_24ghz_channels = channel_density
        .iter()
        .filter(|(&ch, &count)| ch <= 14 && count >= 4)
        .map(|(&ch, _)| ch)
        .collect();

    RfSummary {
        total_aps_sampled: samples.len(),
        band_distribution,
        channel_density,
        congested_24ghz_channels,
        note: "RSSI reflects observed signal power, not link throughput/SNR quality. Physical spectrum analysis requires dedicated SDR hardware.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_band_and_channel_congestion() {
        let samples = vec![
            RfSample { band: "2.4GHz".into(), channel: 6, rssi_dbm: -55 },
            RfSample { band: "2.4GHz".into(), channel: 6, rssi_dbm: -61 },
            RfSample { band: "2.4GHz".into(), channel: 6, rssi_dbm: -65 },
            RfSample { band: "2.4GHz".into(), channel: 6, rssi_dbm: -70 },
            RfSample { band: "5GHz".into(), channel: 36, rssi_dbm: -50 },
        ];
        let summary = analyze_rf_environment(&samples);
        assert_eq!(summary.total_aps_sampled, 5);
        assert_eq!(summary.congested_24ghz_channels, vec![6]);
        assert_eq!(summary.band_distribution.get("5GHz"), Some(&1));
    }
}
