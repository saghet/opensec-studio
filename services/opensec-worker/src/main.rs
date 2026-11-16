use opensec_core::rf::{analyze_rf_environment, RfSample};
use postgres::{Client, NoTls};
use std::env;
use std::thread;
use std::time::Duration;

fn run_rf_cycle(db_url: &str) {
    let Ok(mut client) = Client::connect(db_url, NoTls) else {
        return;
    };
    let Ok(rows) = client.query(
        "SELECT COALESCE(band, '2.4GHz'), COALESCE(channel, 0) FROM wireless_networks",
        &[],
    ) else {
        return;
    };
    let samples: Vec<RfSample> = rows
        .into_iter()
        .map(|r| RfSample {
            band: r.get::<_, String>(0),
            channel: r.get::<_, i32>(1).max(0) as u16,
            rssi_dbm: -65,
        })
        .collect();
    if !samples.is_empty() {
        let summary = analyze_rf_environment(&samples);
        let payload = serde_json::json!({
            "total_aps_sampled": summary.total_aps_sampled,
            "band_distribution": summary.band_distribution,
            "congested_24ghz_channels": summary.congested_24ghz_channels,
        });
        println!(r#"{{"service":"opensec-worker","event":"rf_cycle","summary":{payload}}}"#);
    }
}

fn main() {
    let db_url = env::var("DATABASE_URL").unwrap_or_default();
    println!(r#"{{"service":"opensec-worker","event":"started","mode":"passive_only"}}"#);
    loop {
        if !db_url.is_empty() {
            run_rf_cycle(&db_url);
        }
        thread::sleep(Duration::from_secs(30));
        println!(r#"{{"service":"opensec-worker","event":"heartbeat"}}"#);
    }
}
