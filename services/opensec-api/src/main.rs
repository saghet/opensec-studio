use opensec_core::assessment::{assess_network_posture, NetworkPostureInput, PmfStatus, Severity};
use opensec_core::rf::{analyze_rf_environment, RfSample};
use opensec_core::risk::{calculate_risk_score, RiskInput};
use opensec_core::wids::{evaluate_wids_observation, WidsObservation};
use postgres::{Client, NoTls};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_http::{Header, Method, Response, Server, StatusCode};

#[derive(Debug, Clone)]
pub struct QueryFilters {
    pub sensor: Option<String>,
    pub org: Option<String>,
    pub severity: Option<String>,
    pub min_confidence: Option<f32>,
    pub network: Option<String>,
    pub format: Option<String>,
    pub filename: Option<String>,
    pub id: Option<String>,
}

pub fn parse_query_filters(url: &str) -> (&str, QueryFilters) {
    let (route, query) = match url.split_once('?') {
        Some((r, q)) => (r, q),
        None => (url, ""),
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
            format: map.remove("format"),
            filename: map.remove("filename"),
            id: map.remove("id"),
        },
    )
}

fn severity_str(s: &Severity) -> &'static str {
    match s {
        Severity::Critical => "critical",
        Severity::High => "high",
        Severity::Medium => "medium",
        Severity::Low => "low",
        Severity::Info => "info",
    }
}

fn nature_str(n: &opensec_core::assessment::FindingNature) -> &'static str {
    match n {
        opensec_core::assessment::FindingNature::ObservedConfig => "observed_config",
        opensec_core::assessment::FindingNature::InferredConfig => "inferred_config",
        opensec_core::assessment::FindingNature::ProvenVulnerability => "proven_vulnerability",
        opensec_core::assessment::FindingNature::PotentialRisk => "potential_risk",
    }
}

fn connect_db() -> Result<Client, String> {
    let db_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set".to_string())?;
    Client::connect(&db_url, NoTls).map_err(|e| format!("db_connect_error: {e}"))
}

fn ensure_schema(client: &mut Client) -> Result<String, String> {
    let ddl = r#"
    CREATE TABLE IF NOT EXISTS wireless_clients (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
      client_mac TEXT NOT NULL,
      associated_bssid TEXT,
      first_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
      last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
      UNIQUE (org_id, client_mac)
    );
    CREATE TABLE IF NOT EXISTS pcap_captures (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
      filename TEXT NOT NULL,
      sha256 TEXT NOT NULL,
      size_bytes BIGINT NOT NULL,
      total_frames INT NOT NULL DEFAULT 0,
      deauth_frames INT NOT NULL DEFAULT 0,
      eapol_frames INT NOT NULL DEFAULT 0,
      executive_summary TEXT NOT NULL DEFAULT '',
      technical_report JSONB NOT NULL DEFAULT '{}'::jsonb,
      status TEXT NOT NULL DEFAULT 'analyzed',
      analyzed_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );
    CREATE TABLE IF NOT EXISTS security_assessments (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
      bssid TEXT NOT NULL,
      ssid TEXT NOT NULL DEFAULT '',
      rule_id TEXT NOT NULL,
      finding_nature TEXT NOT NULL,
      severity TEXT NOT NULL,
      confidence NUMERIC(4,3) NOT NULL,
      title TEXT NOT NULL,
      explanation TEXT NOT NULL,
      remediation TEXT NOT NULL,
      detected_at TIMESTAMPTZ NOT NULL DEFAULT now(),
      UNIQUE (org_id, bssid, rule_id)
    );
    INSERT INTO organizations (name, slug)
    VALUES ('Default Organization', 'default')
    ON CONFLICT (slug) DO NOTHING;
    "#;
    client.batch_execute(ddl).map_err(|e| format!("ddl_error: {e}"))?;
    let row = client
        .query_one("SELECT id::text FROM organizations WHERE slug = 'default' LIMIT 1", &[])
        .map_err(|e| format!("org_lookup_error: {e}"))?;
    Ok(row.get(0))
}

fn log_audit(client: &mut Client, org_id: &str, action: &str, resource_type: &str, resource_id: &str, meta: Value) {
    let _ = client.execute(
        "INSERT INTO audit_logs (org_id, action, resource_type, resource_id, metadata) VALUES ($1::text::uuid, $2, $3, $4, $5)",
        &[&org_id, &action, &resource_type, &resource_id, &meta],
    );
}

#[derive(Deserialize)]
struct PcapNetworkDto {
    bssid: String,
    #[serde(default)]
    ssid: String,
    band: Option<String>,
    channel: Option<i32>,
    rssi_dbm: Option<i32>,
    #[serde(default = "default_unknown")]
    security_observed: String,
    #[serde(default)]
    wps_observed: bool,
    #[serde(default)]
    beacon_frames: i32,
    #[serde(default)]
    deauth_frames: i32,
    #[serde(default)]
    eapol_frames: i32,
    #[serde(default)]
    clients: Vec<String>,
}

fn default_unknown() -> String {
    "Unknown".to_string()
}

fn ingest_networks_pipeline(
    client: &mut Client,
    org_id: &str,
    sensor_name: &str,
    nets: &[PcapNetworkDto],
) -> Result<Value, String> {
    let now_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    // Load allowlist for org
    let allow_rows = client
        .query(
            "SELECT COALESCE(an.ssid_pattern, ''), COALESCE(an.bssid::text, '')
             FROM authorized_networks an
             JOIN authorization_scopes s ON s.id = an.scope_id
             WHERE s.org_id = $1::text::uuid",
            &[&org_id],
        )
        .unwrap_or_default();

    let mut allowed_ssids: Vec<String> = Vec::new();
    let mut allowed_bssids: Vec<String> = Vec::new();
    for r in allow_rows {
        let s: String = r.get(0);
        let b: String = r.get(1);
        if !s.is_empty() {
            allowed_ssids.push(s);
        }
        if !b.is_empty() {
            allowed_bssids.push(b.to_lowercase());
        }
    }

    let mut networks_upserted = 0;
    let mut assessments_upserted = 0;
    let mut alerts_upserted = 0;

    for n in nets {
        let bssid_lc = n.bssid.to_lowercase();
        let is_auth_bssid = allowed_bssids.contains(&bssid_lc);
        let is_known_ssid = !n.ssid.is_empty() && allowed_ssids.iter().any(|s| s == &n.ssid);
        let is_authorized = is_auth_bssid || (is_known_ssid && allowed_bssids.is_empty());

        let net_row = client
            .query_one(
                "INSERT INTO wireless_networks (org_id, bssid, ssid, band, channel, security_observed, wps_observed, is_authorized, last_seen_at)
                 VALUES ($1::text::uuid, $2::text::macaddr, $3, $4, $5, $6, $7, $8, now())
                 ON CONFLICT (org_id, bssid, ssid) DO UPDATE SET
                   band = COALESCE(EXCLUDED.band, wireless_networks.band),
                   channel = COALESCE(EXCLUDED.channel, wireless_networks.channel),
                   security_observed = CASE WHEN EXCLUDED.security_observed = 'Unknown' THEN wireless_networks.security_observed ELSE EXCLUDED.security_observed END,
                   wps_observed = wireless_networks.wps_observed OR EXCLUDED.wps_observed,
                   is_authorized = EXCLUDED.is_authorized,
                   last_seen_at = now()
                 RETURNING id::text",
                &[
                    &org_id,
                    &bssid_lc,
                    &n.ssid,
                    &n.band,
                    &n.channel,
                    &n.security_observed,
                    &n.wps_observed,
                    &is_authorized,
                ],
            )
            .map_err(|e| format!("upsert_network_error: {e}"))?;
        let network_id: String = net_row.get(0);
        networks_upserted += 1;

        for c in &n.clients {
            let _ = client.execute(
                "INSERT INTO wireless_clients (org_id, client_mac, associated_bssid, last_seen_at)
                 VALUES ($1::text::uuid, $2, $3, now())
                 ON CONFLICT (org_id, client_mac) DO UPDATE SET
                   associated_bssid = EXCLUDED.associated_bssid,
                   last_seen_at = now()",
                &[&org_id, &c.to_lowercase(), &bssid_lc],
            );
        }

        // 1. Security Posture Assessment
        let findings = assess_network_posture(&NetworkPostureInput {
            ssid: n.ssid.clone(),
            bssid: bssid_lc.clone(),
            security_observed: n.security_observed.clone(),
            pmf: PmfStatus::Unknown,
            wps_observed: n.wps_observed,
            is_authorized,
            previous_security: None,
            eap_evidence_observed: n.eapol_frames > 0,
        });

        for f in &findings {
            let sev = severity_str(&f.severity);
            let nat = nature_str(&f.nature);
            let conf = f.confidence as f64;
            let _ = client.execute(
                "INSERT INTO security_assessments (org_id, bssid, ssid, rule_id, finding_nature, severity, confidence, title, explanation, remediation, detected_at)
                 VALUES ($1::text::uuid, $2, $3, $4, $5, $6, $7::float8::numeric, $8, $9, $10, now())
                 ON CONFLICT (org_id, bssid, rule_id) DO UPDATE SET
                   finding_nature = EXCLUDED.finding_nature,
                   severity = EXCLUDED.severity,
                   confidence = EXCLUDED.confidence,
                   detected_at = now()",
                &[&org_id, &bssid_lc, &n.ssid, &f.rule_id, &nat, &sev, &conf, &f.title, &f.explanation, &f.remediation],
            );
            assessments_upserted += 1;
        }

        // 2. WIDS Evaluation
        let wids_alerts = evaluate_wids_observation(
            &WidsObservation {
                sensor_id: sensor_name.to_string(),
                ssid: n.ssid.clone(),
                bssid: bssid_lc.clone(),
                channel: n.channel.unwrap_or(0).max(0) as u16,
                security: n.security_observed.clone(),
                rssi_dbm: n.rssi_dbm.unwrap_or(-70),
                timestamp_sec: now_sec,
                is_authorized_bssid: is_auth_bssid,
                is_known_org_ssid: is_known_ssid,
                deauth_frames_in_window: n.deauth_frames.max(0) as u32,
                beacon_frames_in_window: n.beacon_frames.max(0) as u32,
            },
            None,
            None,
        );

        for a in &wids_alerts {
            let sev = severity_str(&a.severity);
            let conf = a.confidence as f64;
            let ev_json = json!(a.evidence);
            let _ = client.execute(
                "INSERT INTO wids_alerts (org_id, network_id, rule_id, category, severity, confidence, dedup_key, explanation, evidence)
                 VALUES ($1::text::uuid, $2::text::uuid, $3, $4, $5, $6::float8::numeric, $7, $8, $9)
                 ON CONFLICT (org_id, dedup_key) WHERE status = 'open' DO UPDATE SET
                   last_triggered_at = now(),
                   evidence = EXCLUDED.evidence",
                &[&org_id, &network_id, &a.rule_id, &a.category, &sev, &conf, &a.dedup_key, &a.explanation, &ev_json],
            );
            alerts_upserted += 1;
        }

        // 3. Risk Score
        let risk = calculate_risk_score(&RiskInput {
            network_criticality: if is_authorized { 1.3 } else { 0.8 },
            exposure_factor: 1.0,
            findings,
        });
        let score_f64 = risk.score as f64;
        let factors_json = json!({ "unique_findings": risk.deduplicated_count });
        let _ = client.execute("DELETE FROM risk_scores WHERE org_id = $1::text::uuid AND network_id = $2::text::uuid", &[&org_id, &network_id]);
        let _ = client.execute(
            "INSERT INTO risk_scores (org_id, network_id, score, factors, explanation, mitigation_summary)
             VALUES ($1::text::uuid, $2::text::uuid, $3::float8::numeric, $4, $5, $6)",
            &[&org_id, &network_id, &score_f64, &factors_json, &risk.explanation, &risk.top_mitigation],
        );
    }

    Ok(json!({
        "networks_upserted": networks_upserted,
        "assessments_upserted": assessments_upserted,
        "alerts_upserted": alerts_upserted
    }))
}

fn json_response(status: u16, body: Value) -> Response<std::io::Cursor<Vec<u8>>> {
    let bytes = serde_json::to_vec(&body).unwrap_or_else(|_| b"{}".to_vec());
    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
    Response::from_data(bytes).with_status_code(StatusCode(status)).with_header(header)
}

fn text_response(status: u16, content_type: &str, body: String) -> Response<std::io::Cursor<Vec<u8>>> {
    let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap();
    Response::from_string(body).with_status_code(StatusCode(status)).with_header(header)
}

fn sanitize_filename(name: &str) -> Option<String> {
    let base = Path::new(name).file_name()?.to_str()?;
    let clean: String = base
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if clean.to_lowercase().ends_with(".pcap") || clean.to_lowercase().ends_with(".pcapng") {
        Some(clean)
    } else {
        None
    }
}

fn main() {
    let host = env::var("OPENSEC_API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("OPENSEC_API_PORT").unwrap_or_else(|_| "8080".to_string());
    let mode = env::var("OPENSEC_DEFAULT_MODE").unwrap_or_else(|_| "passive_only".to_string());
    let init_ks = env::var("OPENSEC_KILL_SWITCH_ON_BOOT").map(|v| v == "true").unwrap_or(false);
    let pcap_vault = PathBuf::from(env::var("OPENSEC_PCAP_VAULT").unwrap_or_else(|_| "/var/lib/opensec/pcaps".to_string()));
    let analyzer_url = env::var("PCAP_ANALYZER_URL").unwrap_or_else(|_| "http://opensec-pcap-analyzer:8090".to_string());
    let kill_switch = Arc::new(AtomicBool::new(init_ks));

    let addr = format!("{host}:{port}");
    let server = Server::http(&addr).expect("failed to bind tiny_http server");
    println!(r#"{{"service":"opensec-api","event":"listening","addr":"{addr}","mode":"{mode}"}}"#);

    for mut req in server.incoming_requests() {
        let method = req.method().clone();
        let full_url = req.url().to_string();
        let (route, filters) = parse_query_filters(&full_url);
        let ks = kill_switch.load(Ordering::SeqCst);

        if method == Method::Get && (route == "/health" || route == "/api/health") {
            let db_ok = connect_db().is_ok();
            let _ = req.respond(json_response(
                200,
                json!({"status": "ok", "service": "opensec-api", "mode": mode, "kill_switch_active": ks, "db_connected": db_ok}),
            ));
            continue;
        }

        if method == Method::Post && route == "/api/v1/kill-switch/enable" {
            kill_switch.store(true, Ordering::SeqCst);
            if let Ok(mut db) = connect_db() {
                if let Ok(org_id) = ensure_schema(&mut db) {
                    log_audit(&mut db, &org_id, "kill_switch_enabled", "system", "global", json!({}));
                }
            }
            let _ = req.respond(json_response(200, json!({"kill_switch_active": true, "status": "capture_halted"})));
            continue;
        }

        if method == Method::Post && route == "/api/v1/kill-switch/disable" {
            kill_switch.store(false, Ordering::SeqCst);
            if let Ok(mut db) = connect_db() {
                if let Ok(org_id) = ensure_schema(&mut db) {
                    log_audit(&mut db, &org_id, "kill_switch_disabled", "system", "global", json!({}));
                }
            }
            let _ = req.respond(json_response(200, json!({"kill_switch_active": false, "status": "passive_ready"})));
            continue;
        }

        let mut db = match connect_db() {
            Ok(c) => c,
            Err(e) => {
                let _ = req.respond(json_response(503, json!({"error": e})));
                continue;
            }
        };
        let org_id = match ensure_schema(&mut db) {
            Ok(id) => id,
            Err(e) => {
                let _ = req.respond(json_response(500, json!({"error": e})));
                continue;
            }
        };

        let resp = match (method, route) {
            (Method::Get, "/api/v1/dashboard/summary") => {
                let sensors_online: i64 = db.query_one("SELECT COUNT(*) FROM sensors WHERE org_id = $1::text::uuid AND status = 'online'", &[&org_id]).map(|r| r.get(0)).unwrap_or(0);
                let networks_total: i64 = db.query_one("SELECT COUNT(*) FROM wireless_networks WHERE org_id = $1::text::uuid", &[&org_id]).map(|r| r.get(0)).unwrap_or(0);
                let clients_observed: i64 = db.query_one("SELECT COUNT(*) FROM wireless_clients WHERE org_id = $1::text::uuid", &[&org_id]).map(|r| r.get(0)).unwrap_or(0);
                let unknown_networks: i64 = db.query_one("SELECT COUNT(*) FROM wireless_networks WHERE org_id = $1::text::uuid AND is_authorized = false", &[&org_id]).map(|r| r.get(0)).unwrap_or(0);
                let open_alerts: i64 = db.query_one("SELECT COUNT(*) FROM wids_alerts WHERE org_id = $1::text::uuid AND status = 'open'", &[&org_id]).map(|r| r.get(0)).unwrap_or(0);
                let captures_analyzed: i64 = db.query_one("SELECT COUNT(*) FROM pcap_captures WHERE org_id = $1::text::uuid AND status = 'analyzed'", &[&org_id]).map(|r| r.get(0)).unwrap_or(0);

                // RF distribution from real networks
                let net_rows = db.query("SELECT COALESCE(band, '2.4GHz'), COALESCE(channel, 0) FROM wireless_networks WHERE org_id = $1::text::uuid", &[&org_id]).unwrap_or_default();
                let rf_samples: Vec<RfSample> = net_rows.into_iter().map(|r| RfSample {
                    band: r.get(0),
                    channel: r.get::<_, i32>(1).max(0) as u16,
                    rssi_dbm: -65,
                }).collect();
                let rf_summary = analyze_rf_environment(&rf_samples);

                json_response(200, json!({
                    "mode": mode,
                    "kill_switch_active": ks,
                    "data_source": "postgresql",
                    "sensors_online": sensors_online,
                    "networks_total": networks_total,
                    "clients_observed": clients_observed,
                    "unknown_networks": unknown_networks,
                    "open_alerts": open_alerts,
                    "captures_analyzed": captures_analyzed,
                    "rf": {
                        "band_distribution": rf_summary.band_distribution,
                        "channel_density": rf_summary.channel_density,
                        "congested_24ghz_channels": rf_summary.congested_24ghz_channels,
                        "note": rf_summary.note
                    }
                }))
            }

            (Method::Get, "/api/v1/scopes") => {
                let rows = db.query("SELECT id::text, name, mode, kill_switch_active, valid_from::text, COALESCE(evidence_ref, '') FROM authorization_scopes WHERE org_id = $1::text::uuid ORDER BY valid_from DESC", &[&org_id]).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "name": r.get::<_, String>(1), "mode": r.get::<_, String>(2),
                    "kill_switch_active": r.get::<_, bool>(3), "valid_from": r.get::<_, String>(4), "evidence_ref": r.get::<_, String>(5)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Post, "/api/v1/scopes") => {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                let v: Value = serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                let name = v["name"].as_str().unwrap_or("Default Passive Audit Scope");
                let evidence = v["evidence_ref"].as_str().unwrap_or("Internal Authorization");
                match db.query_one(
                    "INSERT INTO authorization_scopes (org_id, name, mode, evidence_ref) VALUES ($1::text::uuid, $2, 'passive_only', $3) RETURNING id::text",
                    &[&org_id, &name, &evidence],
                ) {
                    Ok(r) => {
                        let id: String = r.get(0);
                        log_audit(&mut db, &org_id, "scope_created", "authorization_scope", &id, json!({"name": name}));
                        json_response(201, json!({"id": id, "name": name, "mode": "passive_only"}))
                    }
                    Err(e) => json_response(500, json!({"error": format!("{e}")})),
                }
            }

            (Method::Get, "/api/v1/allowlist") => {
                let rows = db.query(
                    "SELECT an.id::text, COALESCE(an.ssid_pattern, ''), COALESCE(an.bssid::text, ''), COALESCE(an.notes, ''), s.name
                     FROM authorized_networks an
                     JOIN authorization_scopes s ON s.id = an.scope_id
                     WHERE s.org_id = $1::text::uuid ORDER BY an.ssid_pattern",
                    &[&org_id],
                ).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "ssid": r.get::<_, String>(1), "bssid": r.get::<_, String>(2),
                    "notes": r.get::<_, String>(3), "scope_name": r.get::<_, String>(4)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Post, "/api/v1/allowlist") => {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                let v: Value = serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                let ssid = v["ssid"].as_str().unwrap_or("").trim().to_string();
                let bssid_opt = v["bssid"].as_str().map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty());
                let notes = v["notes"].as_str().unwrap_or("");

                // Ensure at least one scope exists
                let scope_id: String = match db.query_one("SELECT id::text FROM authorization_scopes WHERE org_id = $1::text::uuid LIMIT 1", &[&org_id]) {
                    Ok(r) => r.get(0),
                    Err(_) => db.query_one("INSERT INTO authorization_scopes (org_id, name, mode) VALUES ($1::text::uuid, 'Default Passive Scope', 'passive_only') RETURNING id::text", &[&org_id]).unwrap().get(0),
                };

                let res = db.query_one(
                    "INSERT INTO authorized_networks (scope_id, ssid_pattern, bssid, notes) VALUES ($1::text::uuid, NULLIF($2, ''), $3::text::macaddr, $4) RETURNING id::text",
                    &[&scope_id, &ssid, &bssid_opt, &notes],
                );
                match res {
                    Ok(r) => {
                        let id: String = r.get(0);
                        if !ssid.is_empty() {
                            let _ = db.execute("UPDATE wireless_networks SET is_authorized = true WHERE org_id = $1::text::uuid AND ssid = $2", &[&org_id, &ssid]);
                        }
                        if let Some(ref b) = bssid_opt {
                            let _ = db.execute("UPDATE wireless_networks SET is_authorized = true WHERE org_id = $1::text::uuid AND bssid = $2::text::macaddr", &[&org_id, b]);
                        }
                        log_audit(&mut db, &org_id, "allowlist_added", "authorized_network", &id, json!({"ssid": ssid, "bssid": bssid_opt}));
                        json_response(201, json!({"id": id, "ssid": ssid, "bssid": bssid_opt}))
                    }
                    Err(e) => json_response(400, json!({"error": format!("{e}")})),
                }
            }

            (Method::Delete, "/api/v1/allowlist") => {
                if let Some(id) = filters.id {
                    let _ = db.execute("DELETE FROM authorized_networks WHERE id = $1::text::uuid", &[&id]);
                    log_audit(&mut db, &org_id, "allowlist_removed", "authorized_network", &id, json!({}));
                    json_response(200, json!({"deleted": id}))
                } else {
                    json_response(400, json!({"error": "missing_id"}))
                }
            }

            (Method::Get, "/api/v1/sensors") => {
                let rows = db.query("SELECT id::text, hostname, deployment_mode, status, COALESCE(kismet_endpoint, ''), COALESCE(last_heartbeat_at::text, '') FROM sensors WHERE org_id = $1::text::uuid ORDER BY hostname", &[&org_id]).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "hostname": r.get::<_, String>(1), "deployment_mode": r.get::<_, String>(2),
                    "status": r.get::<_, String>(3), "kismet_endpoint": r.get::<_, String>(4), "last_heartbeat_at": r.get::<_, String>(5)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Post, "/api/v1/sensors") => {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                let v: Value = serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                let hostname = v["hostname"].as_str().unwrap_or("linux-sensor-01");
                let dep_mode = v["deployment_mode"].as_str().unwrap_or("docker");
                let endpoint = v["kismet_endpoint"].as_str().unwrap_or("http://127.0.0.1:2501");
                let status = v["status"].as_str().unwrap_or("online");
                match db.query_one(
                    "INSERT INTO sensors (org_id, hostname, deployment_mode, status, kismet_endpoint, last_heartbeat_at)
                     VALUES ($1::text::uuid, $2, $3, $4, $5, now()) RETURNING id::text",
                    &[&org_id, &hostname, &dep_mode, &status, &endpoint],
                ) {
                    Ok(r) => {
                        let id: String = r.get(0);
                        log_audit(&mut db, &org_id, "sensor_registered", "sensor", &id, json!({"hostname": hostname}));
                        json_response(201, json!({"id": id, "hostname": hostname, "status": status}))
                    }
                    Err(e) => json_response(400, json!({"error": format!("{e}")})),
                }
            }

            (Method::Get, "/api/v1/networks") => {
                let net_filter = filters.network.unwrap_or_default();
                let rows = db.query(
                    "SELECT w.id::text, w.bssid::text, w.ssid, COALESCE(w.band, ''), COALESCE(w.channel, 0),
                            COALESCE(w.security_observed, 'Unknown'), w.pmf_status, w.wps_observed, w.is_authorized,
                            w.last_seen_at::text, COALESCE(rs.score::float8, 0.0)
                     FROM wireless_networks w
                     LEFT JOIN LATERAL (SELECT score FROM risk_scores WHERE network_id = w.id ORDER BY calculated_at DESC LIMIT 1) rs ON true
                     WHERE w.org_id = $1::text::uuid
                       AND ($2 = '' OR w.ssid ILIKE '%' || $2 || '%' OR w.bssid::text ILIKE '%' || $2 || '%')
                     ORDER BY rs.score DESC NULLS LAST, w.last_seen_at DESC",
                    &[&org_id, &net_filter],
                ).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "bssid": r.get::<_, String>(1), "ssid": r.get::<_, String>(2),
                    "band": r.get::<_, String>(3), "channel": r.get::<_, i32>(4), "security_observed": r.get::<_, String>(5),
                    "pmf_status": r.get::<_, String>(6), "wps_observed": r.get::<_, bool>(7), "is_authorized": r.get::<_, bool>(8),
                    "last_seen_at": r.get::<_, String>(9), "risk_score": r.get::<_, f64>(10)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Get, "/api/v1/clients") => {
                let rows = db.query("SELECT id::text, client_mac, COALESCE(associated_bssid, ''), last_seen_at::text FROM wireless_clients WHERE org_id = $1::text::uuid ORDER BY last_seen_at DESC", &[&org_id]).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "client_mac": r.get::<_, String>(1),
                    "associated_bssid": r.get::<_, String>(2), "last_seen_at": r.get::<_, String>(3)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Get, "/api/v1/alerts") => {
                let sev_f = filters.severity.unwrap_or_default();
                let conf_f = filters.min_confidence.unwrap_or(0.0) as f64;
                let rows = db.query(
                    "SELECT id::text, rule_id, category, severity, confidence::float8, explanation, evidence, status, last_triggered_at::text
                     FROM wids_alerts
                     WHERE org_id = $1::text::uuid
                       AND ($2 = '' OR severity = $2)
                       AND confidence::float8 >= $3
                     ORDER BY last_triggered_at DESC",
                    &[&org_id, &sev_f, &conf_f],
                ).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "rule_id": r.get::<_, String>(1), "category": r.get::<_, String>(2),
                    "severity": r.get::<_, String>(3), "confidence": r.get::<_, f64>(4), "explanation": r.get::<_, String>(5),
                    "evidence": r.get::<_, Value>(6), "status": r.get::<_, String>(7), "last_triggered_at": r.get::<_, String>(8)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Patch, "/api/v1/alerts") => {
                let mut body = String::new();
                let _ = req.as_reader().read_to_string(&mut body);
                let v: Value = serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                let id = v["id"].as_str().unwrap_or("");
                let new_status = v["status"].as_str().unwrap_or("acknowledged");
                match db.execute("UPDATE wids_alerts SET status = $1 WHERE id = $2::text::uuid AND org_id = $3::text::uuid", &[&new_status, &id, &org_id]) {
                    Ok(_) => {
                        log_audit(&mut db, &org_id, "alert_status_updated", "wids_alert", id, json!({"status": new_status}));
                        json_response(200, json!({"id": id, "status": new_status}))
                    }
                    Err(e) => json_response(400, json!({"error": format!("{e}")})),
                }
            }

            (Method::Get, "/api/v1/assessments") => {
                let sev_f = filters.severity.unwrap_or_default();
                let rows = db.query(
                    "SELECT id::text, bssid, ssid, rule_id, finding_nature, severity, confidence::float8, title, explanation, remediation, detected_at::text
                     FROM security_assessments
                     WHERE org_id = $1::text::uuid AND ($2 = '' OR severity = $2)
                     ORDER BY detected_at DESC",
                    &[&org_id, &sev_f],
                ).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "bssid": r.get::<_, String>(1), "ssid": r.get::<_, String>(2),
                    "rule_id": r.get::<_, String>(3), "finding_nature": r.get::<_, String>(4), "severity": r.get::<_, String>(5),
                    "confidence": r.get::<_, f64>(6), "title": r.get::<_, String>(7), "explanation": r.get::<_, String>(8),
                    "remediation": r.get::<_, String>(9), "detected_at": r.get::<_, String>(10)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Get, "/api/v1/risks") => {
                let rows = db.query(
                    "SELECT r.id::text, w.bssid::text, w.ssid, r.score::float8, r.factors, r.explanation, r.mitigation_summary, r.calculated_at::text
                     FROM risk_scores r
                     JOIN wireless_networks w ON w.id = r.network_id
                     WHERE r.org_id = $1::text::uuid
                     ORDER BY r.score DESC",
                    &[&org_id],
                ).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "bssid": r.get::<_, String>(1), "ssid": r.get::<_, String>(2),
                    "score": r.get::<_, f64>(3), "factors": r.get::<_, Value>(4), "explanation": r.get::<_, String>(5),
                    "mitigation_summary": r.get::<_, String>(6), "calculated_at": r.get::<_, String>(7)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Get, "/api/v1/pcaps") => {
                let rows = db.query("SELECT id::text, filename, sha256, size_bytes, total_frames, deauth_frames, eapol_frames, executive_summary, status, analyzed_at::text FROM pcap_captures WHERE org_id = $1::text::uuid ORDER BY analyzed_at DESC", &[&org_id]).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, String>(0), "filename": r.get::<_, String>(1), "sha256": r.get::<_, String>(2),
                    "size_bytes": r.get::<_, i64>(3), "total_frames": r.get::<_, i32>(4), "deauth_frames": r.get::<_, i32>(5),
                    "eapol_frames": r.get::<_, i32>(6), "executive_summary": r.get::<_, String>(7), "status": r.get::<_, String>(8),
                    "analyzed_at": r.get::<_, String>(9)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Post, "/api/v1/pcaps/upload") => {
                let raw_name = filters.filename.unwrap_or_else(|| "capture.pcapng".to_string());
                let Some(safe_name) = sanitize_filename(&raw_name) else {
                    let _ = req.respond(json_response(400, json!({"error": "invalid_pcap_extension"})));
                    continue;
                };
                let mut bytes = Vec::new();
                if let Err(e) = req.as_reader().take(262_144_000).read_to_end(&mut bytes) {
                    let _ = req.respond(json_response(400, json!({"error": format!("read_error: {e}")})));
                    continue;
                }
                if bytes.is_empty() {
                    let _ = req.respond(json_response(400, json!({"error": "empty_file"})));
                    continue;
                }
                let _ = fs::create_dir_all(&pcap_vault);
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
                let stored_name = format!("{ts}_{safe_name}");
                let dest = pcap_vault.join(&stored_name);
                if let Err(e) = fs::write(&dest, &bytes) {
                    let _ = req.respond(json_response(500, json!({"error": format!("vault_write_error: {e}")})));
                    continue;
                }

                // Call opensec-pcap-analyzer (tshark)
                let analyze_res = ureq::post(&format!("{analyzer_url}/analyze"))
                    .send_json(json!({"filename": stored_name}));

                match analyze_res {
                    Ok(resp_body) => {
                        let parsed: Value = resp_body.into_json().unwrap_or_else(|_| json!({}));
                        let tech = &parsed["technical_report"];
                        let nets: Vec<PcapNetworkDto> = serde_json::from_value(tech["networks"].clone()).unwrap_or_default();
                        let pipeline_stats = ingest_networks_pipeline(&mut db, &org_id, "pcap-analyzer", &nets).unwrap_or_else(|_| json!({}));

                        let sha256 = parsed["sha256"].as_str().unwrap_or("");
                        let size_b = parsed["size_bytes"].as_i64().unwrap_or(bytes.len() as i64);
                        let total_f = tech["total_frames"].as_i64().unwrap_or(0) as i32;
                        let deauth_f = tech["deauth_frames"].as_i64().unwrap_or(0) as i32;
                        let eapol_f = tech["eapol_frames"].as_i64().unwrap_or(0) as i32;
                        let exec_sum = parsed["executive_summary"].as_str().unwrap_or("");

                        let _ = db.execute(
                            "INSERT INTO pcap_captures (org_id, filename, sha256, size_bytes, total_frames, deauth_frames, eapol_frames, executive_summary, technical_report)
                             VALUES ($1::text::uuid, $2, $3, $4, $5, $6, $7, $8, $9)",
                            &[&org_id, &stored_name, &sha256, &size_b, &total_f, &deauth_f, &eapol_f, &exec_sum, tech],
                        );
                        log_audit(&mut db, &org_id, "pcap_analyzed", "pcap_capture", &stored_name, json!({"frames": total_f, "networks": nets.len()}));
                        json_response(200, json!({"capture": parsed, "pipeline": pipeline_stats}))
                    }
                    Err(e) => json_response(502, json!({"error": format!("pcap_analyzer_error: {e}")})),
                }
            }

            (Method::Post, "/api/v1/pcaps/purge") => {
                match ureq::post(&format!("{analyzer_url}/purge")).call() {
                    Ok(r) => {
                        let v: Value = r.into_json().unwrap_or_else(|_| json!({}));
                        log_audit(&mut db, &org_id, "pcap_retention_purge", "pcap_vault", "expired", v.clone());
                        json_response(200, v)
                    }
                    Err(e) => json_response(502, json!({"error": format!("{e}")})),
                }
            }

            (Method::Post, "/api/v1/ingest/kismet") => {
                if ks {
                    json_response(423, json!({"error": "kill_switch_active"}))
                } else {
                    let mut body = String::new();
                    let _ = req.as_reader().read_to_string(&mut body);
                    let v: Value = serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                    let sensor = v["sensor_id"].as_str().unwrap_or("kismet-sensor");
                    let nets: Vec<PcapNetworkDto> = serde_json::from_value(v["networks"].clone()).unwrap_or_default();
                    match ingest_networks_pipeline(&mut db, &org_id, sensor, &nets) {
                        Ok(stats) => {
                            log_audit(&mut db, &org_id, "kismet_ingested", "sensor", sensor, stats.clone());
                            json_response(200, stats)
                        }
                        Err(e) => json_response(500, json!({"error": e})),
                    }
                }
            }

            (Method::Get, "/api/v1/audit-logs") => {
                let rows = db.query("SELECT id, action, resource_type, COALESCE(resource_id, ''), metadata, created_at::text FROM audit_logs WHERE org_id = $1::text::uuid ORDER BY id DESC LIMIT 100", &[&org_id]).unwrap_or_default();
                let items: Vec<Value> = rows.into_iter().map(|r| json!({
                    "id": r.get::<_, i64>(0), "action": r.get::<_, String>(1), "resource_type": r.get::<_, String>(2),
                    "resource_id": r.get::<_, String>(3), "metadata": r.get::<_, Value>(4), "created_at": r.get::<_, String>(5)
                })).collect();
                json_response(200, json!({"items": items, "total": items.len()}))
            }

            (Method::Get, "/api/v1/reports/export") => {
                let fmt = filters.format.unwrap_or_else(|| "json".to_string());
                let rows = db.query(
                    "SELECT bssid, ssid, rule_id, finding_nature, severity, title, explanation, remediation FROM security_assessments WHERE org_id = $1::text::uuid ORDER BY severity, bssid",
                    &[&org_id],
                ).unwrap_or_default();

                if fmt == "html" {
                    let mut rows_html = String::new();
                    for r in rows {
                        rows_html.push_str(&format!(
                            "<tr><td>{}</td><td><code>{}</code></td><td><code>{}</code></td><td>{}</td><td><strong>{}</strong></td><td>{}</td><td>{}</td></tr>",
                            r.get::<_, String>(1), r.get::<_, String>(0), r.get::<_, String>(2),
                            r.get::<_, String>(3), r.get::<_, String>(4), r.get::<_, String>(5), r.get::<_, String>(7)
                        ));
                    }
                    let html = format!(
                        "<!DOCTYPE html><html lang=\"es\"><head><meta charset=\"UTF-8\"><title>OpenSec Studio - Informe de Seguridad Wi-Fi</title><style>body{{font-family:system-ui,sans-serif;margin:32px;color:#0f172a}}table{{width:100%;border-collapse:collapse;margin-top:16px}}th,td{{border:1px solid #cbd5e1;padding:8px;text-align:left;font-size:13px}}th{{background:#f1f5f9}}</style></head><body><h1>OpenSec Studio — Informe de Evaluación Inalámbrica</h1><p>Modo: {} | Fuente: PostgreSQL</p><table><thead><tr><th>SSID</th><th>BSSID</th><th>Regla</th><th>Naturaleza</th><th>Severidad</th><th>Hallazgo</th><th>Remediación</th></tr></thead><tbody>{}</tbody></table></body></html>",
                        mode, rows_html
                    );
                    text_response(200, "text/html; charset=utf-8", html)
                } else if fmt == "csv" {
                    let mut csv = String::from("bssid,ssid,rule_id,nature,severity,title,remediation\n");
                    for r in rows {
                        let line = format!(
                            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                            r.get::<_, String>(0), r.get::<_, String>(1).replace('"', "\"\""),
                            r.get::<_, String>(2), r.get::<_, String>(3), r.get::<_, String>(4),
                            r.get::<_, String>(5).replace('"', "\"\""), r.get::<_, String>(7).replace('"', "\"\"")
                        );
                        csv.push_str(&line);
                    }
                    text_response(200, "text/csv; charset=utf-8", csv)
                } else if fmt == "sarif" {
                    let results: Vec<Value> = rows.into_iter().map(|r| {
                        let sev = r.get::<_, String>(4);
                        let level = if sev == "critical" || sev == "high" { "error" } else if sev == "medium" { "warning" } else { "note" };
                        json!({
                            "ruleId": r.get::<_, String>(2),
                            "level": level,
                            "message": { "text": format!("{} ({} / {}): {}", r.get::<_, String>(5), r.get::<_, String>(0), r.get::<_, String>(1), r.get::<_, String>(6)) }
                        })
                    }).collect();
                    json_response(200, json!({
                        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
                        "version": "2.1.0",
                        "runs": [{
                            "tool": { "driver": { "name": "OpenSec Studio Wireless Platform", "version": "0.1.0" } },
                            "results": results
                        }]
                    }))
                } else {
                    let items: Vec<Value> = rows.into_iter().map(|r| json!({
                        "bssid": r.get::<_, String>(0), "ssid": r.get::<_, String>(1), "rule_id": r.get::<_, String>(2),
                        "nature": r.get::<_, String>(3), "severity": r.get::<_, String>(4), "title": r.get::<_, String>(5),
                        "explanation": r.get::<_, String>(6), "remediation": r.get::<_, String>(7)
                    })).collect();
                    json_response(200, json!({"findings": items}))
                }
            }

            _ => json_response(404, json!({"error": "not_found"})),
        };

        let _ = req.respond(resp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_filters_and_filename() {
        let (route, f) = parse_query_filters("/api/v1/pcaps/upload?filename=lab.pcapng&severity=high");
        assert_eq!(route, "/api/v1/pcaps/upload");
        assert_eq!(f.filename.as_deref(), Some("lab.pcapng"));
        assert_eq!(f.severity.as_deref(), Some("high"));
        assert_eq!(sanitize_filename("../evil.pcap"), Some("evil.pcap".to_string()));
        assert_eq!(sanitize_filename("payload.exe"), None);
    }
}
