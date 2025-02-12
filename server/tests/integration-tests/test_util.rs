use std::fmt::{Display, Formatter, Result};
use std::io::prelude::*;

use camino::Utf8PathBuf;
use dropshot::test_util::TestContext as DropshotTestContext;
use dropshot::test_util::{log_file_for_test, LogContext};
use dropshot::ServerContext;
use dropshot::{
    Body, ConfigDropshot, ConfigLogging, ConfigLoggingIfExists, ConfigLoggingLevel, HandlerTaskMode,
};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use http::header::{CONTENT_ENCODING, CONTENT_TYPE};
use http::Method;
use serde::Serialize;
use slog::{o, Logger};

use chimney_server::{api, ClientConfig, ConfigStore, Context, EventLogging};

pub const DEFAULT_CONFIG_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/tomls/client-tomls/good");

pub enum MachineId {
    One,
    Two,
}

impl Display for MachineId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MachineId::One => write!(f, "machine-id-1234"),
            MachineId::Two => write!(f, "machine-id-5678"),
        }
    }
}

pub enum EventLogMode {
    None,
    Persist,
}

pub struct TestContext<Context: ServerContext> {
    pub inner: dropshot::test_util::TestContext<Context>,
    pub event_logctx: Option<EventLogContext>,
}

impl TestContext<Context> {
    pub fn new(test_name: &str, config_path: &str, event_logging: EventLogMode) -> Self {
        let (event_logctx, event_log) = match event_logging {
            EventLogMode::None => (None, None),
            EventLogMode::Persist => {
                let ctx = EventLogContext::new(&format!("{}_events", test_name));
                let log = ctx.log.new(o!());
                (Some(ctx), Some(log))
            }
        };
        let store = ConfigStore::from_path(config_path).unwrap();
        let context = Context { event_log, store };
        let default_handler_task_mode = HandlerTaskMode::Detached;
        let default_request_body_max_bytes = chimney_server::DEFAULT_REQUEST_BODY_MAX_BYTES;
        let config_dropshot: ConfigDropshot = ConfigDropshot {
            default_handler_task_mode,
            default_request_body_max_bytes,
            ..Default::default()
        };
        let logctx = create_log_context(test_name);
        let log = logctx.log.new(o!());
        let inner = DropshotTestContext::new(api(), context, &config_dropshot, Some(logctx), log);
        TestContext {
            inner,
            event_logctx,
        }
    }

    pub fn config_for(&self, machine_id: &str) -> ClientConfig {
        self.inner.server.app_private().store.config_for(machine_id)
    }

    pub fn events_as_string(&self) -> Vec<String> {
        if let Some(ctx) = &self.event_logctx {
            if let Some(log_path) = &ctx.log_path {
                let log_contents = std::fs::read_to_string(log_path).unwrap();
                let results = log_contents
                    .split('\n')
                    .filter(|line| !line.is_empty())
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>();
                return results;
            }
        }
        vec![]
    }

    pub async fn teardown(self) {
        if let Some(ctx) = self.event_logctx {
            ctx.cleanup_successful();
        }
        self.inner.teardown().await;
    }
}

pub struct EventLogContext {
    pub log: Logger,
    pub log_path: Option<Utf8PathBuf>,
}

impl EventLogContext {
    fn new(test_name: &str) -> Self {
        let log_path = log_file_for_test(test_name);
        eprintln!("event log file: {}", log_path);
        let log_config = EventLogging::File {
            path: log_path.clone(),
        };
        let log = log_config.to_logger().unwrap();
        EventLogContext {
            log,
            log_path: Some(log_path.clone()),
        }
    }

    fn cleanup_successful(self) {
        if let Some(ref log_path) = self.log_path {
            std::fs::remove_file(log_path).unwrap();
        }
    }
}

pub enum ContentEncoding {
    Deflate,
    Gzip,
}

impl Display for ContentEncoding {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ContentEncoding::Deflate => write!(f, "deflate"),
            ContentEncoding::Gzip => write!(f, "gzip"),
        }
    }
}

pub fn build_request(
    request_body: &str,
    encoding: &ContentEncoding,
    uri: hyper::Uri,
) -> hyper::Request<Body> {
    let encoded = encode(request_body, encoding);
    hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, encoding.to_string())
        .body(Body::with_content(encoded))
        .expect("invalid request")
}

pub fn encode(request_body: &str, encoding: &ContentEncoding) -> EncodedBody {
    match encoding {
        ContentEncoding::Deflate => zlib_encode_str(request_body),
        ContentEncoding::Gzip => gzip_encode_str(request_body),
    }
}

type EncodedBody = Vec<u8>;

pub fn zlib_encode<T: Serialize>(request: &T) -> EncodedBody {
    let serialized = serde_json::to_string(request).unwrap();
    zlib_encode_str(&serialized)
}

pub fn zlib_encode_str(serialized: &str) -> EncodedBody {
    let mut zlib = ZlibEncoder::new(vec![], Compression::default());
    zlib.write_all(serialized.as_bytes()).unwrap();
    zlib.finish().unwrap()
}

pub fn gzip_encode<T: Serialize>(request: &T) -> EncodedBody {
    let serialized = serde_json::to_string(request).unwrap();
    gzip_encode_str(&serialized)
}

pub fn gzip_encode_str(serialized: &str) -> EncodedBody {
    let mut gz = GzEncoder::new(vec![], Compression::default());
    gz.write_all(serialized.as_bytes()).unwrap();
    gz.finish().unwrap()
}

pub fn create_log_context(test_name: &str) -> LogContext {
    let log_config = ConfigLogging::File {
        level: ConfigLoggingLevel::Debug,
        path: "UNUSED".into(),
        if_exists: ConfigLoggingIfExists::Fail,
    };
    LogContext::new(test_name, &log_config)
}
