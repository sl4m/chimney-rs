use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct PreflightOptions {
    pub serial_num: String,
    pub hostname: String,
    pub os_version: String,
    pub os_build: String,
    pub model_identifier: Option<String>,
    pub santa_version: String,
    pub primary_user: Option<String>,
    pub binary_rule_count: Option<u32>,
    pub certificate_rule_count: Option<u32>,
    pub compiler_rule_count: Option<u32>,
    pub transitive_rule_count: Option<u32>,
    pub teamid_rule_count: Option<u32>,
    pub signingid_rule_count: Option<u32>,
    pub cdhash_rule_count: Option<u32>,
    pub client_mode: ClientMode,
    #[serde(default)]
    pub request_clean_sync: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct Preflight {
    #[serde(default)]
    pub enable_bundles: bool,
    #[serde(default)]
    pub enable_transitive_rules: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub batch_size: Option<u32>,
    #[serde(default = "full_sync_interval_default")]
    pub full_sync_interval: u32,
    pub client_mode: ClientMode,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub allowed_path_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blocked_path_regex: Option<String>,
    #[serde(default)]
    pub block_usb_mount: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub remount_usb_mode: Option<String>,
    #[serde(default = "sync_type_default")]
    pub sync_type: SyncType,
    pub override_file_access_action: OverrideFileAccessAction,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ClientMode {
    Lockdown,
    Monitor,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub enum SyncType {
    #[serde(rename = "CLEAN")]
    Clean,
    #[serde(rename = "CLEAN_ALL")]
    CleanAll,
    #[serde(rename = "NORMAL")]
    Normal,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub enum OverrideFileAccessAction {
    #[serde(rename = "AUDIT_ONLY")]
    AuditOnly,
    #[serde(rename = "DISABLE")]
    Disable,
    #[serde(rename = "NONE")]
    None,
}

fn full_sync_interval_default() -> u32 {
    600
}

fn sync_type_default() -> SyncType {
    SyncType::Normal
}
