-- OpenSec Studio - Wireless Security Platform Initial Schema (PostgreSQL 16+)
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS organizations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  retention_days_pcap INT NOT NULL DEFAULT 7,
  retention_days_telemetry INT NOT NULL DEFAULT 90,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  role TEXT NOT NULL CHECK (role IN ('admin', 'analyst', 'auditor', 'viewer')),
  is_active BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS authorization_scopes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  mode TEXT NOT NULL DEFAULT 'passive_only' CHECK (mode IN ('passive_only')),
  kill_switch_active BOOLEAN NOT NULL DEFAULT false,
  approved_by UUID REFERENCES users(id),
  valid_from TIMESTAMPTZ NOT NULL DEFAULT now(),
  valid_until TIMESTAMPTZ,
  evidence_ref TEXT
);

CREATE TABLE IF NOT EXISTS authorized_networks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  scope_id UUID NOT NULL REFERENCES authorization_scopes(id) ON DELETE CASCADE,
  ssid_pattern TEXT,
  bssid MACADDR,
  band_filter TEXT,
  notes TEXT
);

CREATE TABLE IF NOT EXISTS sensors (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  hostname TEXT NOT NULL,
  deployment_mode TEXT NOT NULL CHECK (deployment_mode IN ('docker', 'native_systemd')),
  status TEXT NOT NULL DEFAULT 'offline' CHECK (status IN ('online', 'offline', 'degraded', 'killed')),
  last_heartbeat_at TIMESTAMPTZ,
  kismet_endpoint TEXT
);

CREATE TABLE IF NOT EXISTS wireless_adapters (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  sensor_id UUID NOT NULL REFERENCES sensors(id) ON DELETE CASCADE,
  interface_name TEXT NOT NULL,
  mac_address MACADDR,
  vendor TEXT,
  chipset TEXT,
  driver TEXT,
  firmware_version TEXT,
  supports_monitor_mode BOOLEAN NOT NULL DEFAULT false,
  supported_bands TEXT[] NOT NULL DEFAULT '{}',
  available_channels INT[] NOT NULL DEFAULT '{}',
  reg_domain TEXT,
  limitations JSONB NOT NULL DEFAULT '{}'::jsonb,
  probed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS wireless_networks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  bssid MACADDR NOT NULL,
  ssid TEXT NOT NULL DEFAULT '',
  oui_vendor TEXT,
  band TEXT CHECK (band IN ('2.4GHz', '5GHz', '6GHz')),
  channel INT,
  security_observed TEXT,
  pmf_status TEXT NOT NULL DEFAULT 'unknown' CHECK (pmf_status IN ('required', 'capable', 'disabled', 'unknown')),
  wps_observed BOOLEAN NOT NULL DEFAULT false,
  is_authorized BOOLEAN NOT NULL DEFAULT false,
  first_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE (org_id, bssid, ssid)
);

CREATE TABLE IF NOT EXISTS wids_alerts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  sensor_id UUID REFERENCES sensors(id) ON DELETE SET NULL,
  network_id UUID REFERENCES wireless_networks(id) ON DELETE SET NULL,
  rule_id TEXT NOT NULL,
  category TEXT NOT NULL,
  severity TEXT NOT NULL CHECK (severity IN ('critical', 'high', 'medium', 'low', 'info')),
  confidence NUMERIC(4,3) NOT NULL CHECK (confidence >= 0 AND confidence <= 1),
  dedup_key TEXT NOT NULL,
  explanation TEXT NOT NULL,
  evidence JSONB NOT NULL DEFAULT '{}'::jsonb,
  first_triggered_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  last_triggered_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'acknowledged', 'false_positive', 'resolved'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_wids_alerts_dedup ON wids_alerts(org_id, dedup_key) WHERE status = 'open';

CREATE TABLE IF NOT EXISTS risk_scores (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  network_id UUID NOT NULL REFERENCES wireless_networks(id) ON DELETE CASCADE,
  score NUMERIC(5,2) NOT NULL CHECK (score >= 0 AND score <= 100),
  factors JSONB NOT NULL DEFAULT '{}'::jsonb,
  explanation TEXT NOT NULL,
  mitigation_summary TEXT NOT NULL,
  calculated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS audit_logs (
  id BIGSERIAL PRIMARY KEY,
  org_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
  user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  action TEXT NOT NULL,
  resource_type TEXT NOT NULL,
  resource_id TEXT,
  metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
