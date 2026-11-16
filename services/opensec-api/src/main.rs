use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct QueryFilters {
    pub sensor: Option<String>,
    pub org: Option<String>,
    pub severity: Option<String>,
    pub min_confidence: Option<f32>,
    pub network: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub fn parse_query_filters(path: &str) -> (&str, QueryFilters) {
    let (route, query) = match path.split_once('?') {
        Some((r, q)) => (r, q),
        None => (path, ""),
    };
    let mut map = HashMap::new();
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if !v.is_empty() {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }
    let min_confidence = map.get("min_confidence").and_then(|v| v.parse::<f32>().ok());
    (
        route,
        QueryFilters {
            sensor: map.remove("sensor"),
            org: map.remove("org"),
            severity: map.remove("severity"),
            min_confidence,
            network: map.remove("network"),
            date_from: map.remove("date_from"),
            date_to: map.remove("date_to"),
        },
    )
}

pub fn handle_request(method: &str, path: &str, kill_switch: &AtomicBool, mode: &str) -> (&'static str, String) {
    let (route, filters) = parse_query_filters(path);
    let ks = kill_switch.load(Ordering::SeqCst);

    match (method, route) {
        ("GET", "/health") | ("GET", "/api/health") => (
            "200 OK",
            format!(
                r#"{{"status":"ok","service":"opensec-api","mode":"{mode}","kill_switch_active":{ks}}}"#
            ),
        ),
        ("GET", "/api/v1/dashboard/summary") => (
            "200 OK",
            format!(
                r#"{{"mode":"{mode}","kill_switch_active":{ks},"data_source":"live_store","sensors_online":0,"networks_total":0,"clients_observed":0,"unknown_networks":0,"rogue_ap_alerts":0,"open_alerts":0,"captures_analyzed":0,"filters_applied":{{"sensor":{},"org":{},"severity":{},"min_confidence":{}}}}}"#,
                json_opt(&filters.sensor),
                json_opt(&filters.org),
                json_opt(&filters.severity),
                filters.min_confidence.map(|v| format!("{v:.2}")).unwrap_or_else(|| "null".to_string())
            ),
        ),
        ("GET", "/api/v1/sensors")
        | ("GET", "/api/v1/networks")
        | ("GET", "/api/v1/clients")
        | ("GET", "/api/v1/alerts")
        | ("GET", "/api/v1/risks")
        | ("GET", "/api/v1/hardware")
        | ("GET", "/api/v1/reports") => (
            "200 OK",
            format!(r#"{{"items":[],"total":0,"kill_switch_active":{ks},"data_source":"live_store"}}"#),
        ),
        ("POST", "/api/v1/kill-switch/enable") => {
            kill_switch.store(true, Ordering::SeqCst);
            ("200 OK", r#"{"kill_switch_active":true,"status":"capture_halted"}"#.to_string())
        }
        ("POST", "/api/v1/kill-switch/disable") => {
            kill_switch.store(false, Ordering::SeqCst);
            ("200 OK", r#"{"kill_switch_active":false,"status":"passive_ready"}"#.to_string())
        }
        _ => ("404 Not Found", r#"{"error":"not_found"}"#.to_string()),
    }
}

fn json_opt(val: &Option<String>) -> String {
    match val {
        Some(s) => format!("\"{}\"", s.replace('"', "")),
        None => "null".to_string(),
    }
}

fn main() {
    let host = env::var("OPENSEC_API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("OPENSEC_API_PORT").unwrap_or_else(|_| "8080".to_string());
    let mode = env::var("OPENSEC_DEFAULT_MODE").unwrap_or_else(|_| "passive_only".to_string());
    let init_ks = env::var("OPENSEC_KILL_SWITCH_ON_BOOT").map(|v| v == "true").unwrap_or(false);
    let kill_switch = Arc::new(AtomicBool::new(init_ks));
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(&addr).expect("failed to bind API listener");
    println!(r#"{{"service":"opensec-api","event":"listening","addr":"{addr}","mode":"{mode}"}}"#);

    for stream in listener.incoming().flatten() {
        let mut s = stream;
        let mut buf = [0u8; 2048];
        let n = s.read(&mut buf).unwrap_or(0);
        let req = String::from_utf8_lossy(&buf[..n]);
        let first_line = req.lines().next().unwrap_or("");
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("GET");
        let path = parts.next().unwrap_or("/");

        let (status, body) = handle_request(method, path, &kill_switch, &mode);
        let resp = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = s.write_all(resp.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_returns_zeroes_never_fake_metrics() {
        let ks = AtomicBool::new(false);
        let (status, body) = handle_request("GET", "/api/v1/dashboard/summary?severity=high&min_confidence=0.8", &ks, "passive_only");
        assert_eq!(status, "200 OK");
        assert!(body.contains(r#""networks_total":0"#));
        assert!(body.contains(r#""severity":"high""#));
        assert!(body.contains(r#""min_confidence":0.80"#));
    }

    #[test]
    fn kill_switch_toggles_cleanly() {
        let ks = AtomicBool::new(false);
        let (_, b1) = handle_request("POST", "/api/v1/kill-switch/enable", &ks, "passive_only");
        assert!(b1.contains(r#""kill_switch_active":true"#));
        let (_, b2) = handle_request("POST", "/api/v1/kill-switch/disable", &ks, "passive_only");
        assert!(b2.contains(r#""kill_switch_active":false"#));
    }
}
