use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Rules {
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Rule {
    pub identifier: String,
    pub policy: Policy,
    pub rule_type: RuleType,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub custom_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub custom_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub creation_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_bundle_binary_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_bundle_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub enum Policy {
    #[serde(rename = "ALLOWLIST")]
    Allowlist,
    #[serde(rename = "ALLOWLIST_COMPILER")]
    AllowlistCompiler,
    #[serde(rename = "BLOCKLIST")]
    Blocklist,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "SILENT_BLOCKLIST")]
    SilentBlocklist,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub enum RuleType {
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "CDHASH")]
    CdHash,
    #[serde(rename = "CERTIFICATE")]
    Certificate,
    #[serde(rename = "SIGNINGID")]
    SigningId,
    #[serde(rename = "TEAMID")]
    TeamId,
}
