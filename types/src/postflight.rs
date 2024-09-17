use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct PostflightOptions {
    pub rules_received: Option<u32>,
    pub rules_processed: Option<u32>,
}
