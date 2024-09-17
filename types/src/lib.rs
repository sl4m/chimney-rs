#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod event_upload;
mod logging;
mod postflight;
mod preflight;
mod rule_download;

pub use event_upload::{Event, EventUploadOptions};
pub use postflight::PostflightOptions;
pub use preflight::{Preflight, PreflightOptions};
pub use rule_download::{Rule, Rules};

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Empty {}
