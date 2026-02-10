use std::vec::Vec;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct EventUploadOptions {
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Event {
    pub file_sha256: String,
    pub file_path: String,
    pub file_name: String,
    pub executing_user: Option<String>,
    pub execution_time: Option<f64>,
    #[serde(default)]
    pub loggedin_users: Vec<String>,
    #[serde(default)]
    pub current_sessions: Vec<String>,
    pub decision: Decision,
    pub file_bundle_id: Option<String>,
    pub file_bundle_path: Option<String>,
    pub file_bundle_executable_rel_path: Option<String>,
    pub file_bundle_name: Option<String>,
    pub file_bundle_version: Option<String>,
    pub file_bundle_version_string: Option<String>,
    pub file_bundle_hash: Option<String>,
    pub file_bundle_hash_millis: Option<u32>,
    pub file_bundle_binary_count: Option<u32>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
    pub parent_name: Option<String>,
    pub quarantine_data_url: Option<String>,
    pub quarantine_referer_url: Option<String>,
    pub quarantine_timestamp: Option<f64>,
    pub quarantine_agent_bundle_id: Option<String>,
    #[serde(default)]
    pub signing_chain: Vec<SigningChainObject>,
    pub signing_id: Option<String>,
    pub team_id: Option<String>,
    pub cdhash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub enum Decision {
    #[serde(rename = "ALLOW_BINARY")]
    AllowBinary,
    #[serde(rename = "ALLOW_CERTIFICATE")]
    AllowCertificate,
    #[serde(rename = "ALLOW_CDHASH")]
    AllowCdHash,
    #[serde(rename = "ALLOW_SCOPE")]
    AllowScope,
    #[serde(rename = "ALLOW_SIGNINGID")]
    AllowSigningId,
    #[serde(rename = "ALLOW_TEAMID")]
    AllowTeamId,
    #[serde(rename = "ALLOW_UNKNOWN")]
    AllowUnknown,
    #[serde(rename = "BLOCK_BINARY")]
    BlockBinary,
    #[serde(rename = "BLOCK_CERTIFICATE")]
    BlockCertificate,
    #[serde(rename = "BLOCK_CDHASH")]
    BlockCdHash,
    #[serde(rename = "BLOCK_SCOPE")]
    BlockScope,
    #[serde(rename = "BLOCK_SIGNINGID")]
    BlockSigningId,
    #[serde(rename = "BLOCK_TEAMID")]
    BlockTeamId,
    #[serde(rename = "BLOCK_UNKNOWN")]
    BlockUnknown,
    #[serde(rename = "BUNDLE_BINARY")]
    BundleBinary,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct SigningChainObject {
    pub sha256: String,
    pub cn: String,
    pub org: String,
    pub ou: String,
    pub valid_from: u32,
    pub valid_until: u32,
}
