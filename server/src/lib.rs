#![warn(missing_debug_implementations)]

mod api;
pub mod body;
mod config_store;
mod constants;
mod entrypoints;
mod logging;
mod server_config;

use anyhow::anyhow;
use dropshot::{
    ApiDescription, ConfigDropshot, ConfigLogging, ConfigTls, HandlerTaskMode, HttpServer,
    ServerBuilder,
};

pub use config_store::{ClientConfig, ConfigStore};
pub(crate) use constants::CHIMNEY;
pub use logging::EventLogging;
pub use server_config::ServerConfig;

pub const DEFAULT_REQUEST_BODY_MAX_BYTES: usize = 104_857_600; // 100 MB

#[derive(Debug)]
pub struct Context {
    pub event_log: Option<slog::Logger>,
    pub store: ConfigStore,
}

pub(crate) enum SantaSyncServerApiImpl {}

pub async fn start_server(
    server_config: ServerConfig,
) -> Result<HttpServer<Context>, anyhow::Error> {
    let server = {
        let config = ConfigDropshot {
            bind_address: server_config.bind_address,
            default_handler_task_mode: HandlerTaskMode::Detached,
            default_request_body_max_bytes: DEFAULT_REQUEST_BODY_MAX_BYTES,
            log_headers: Default::default(),
        };

        let event_log = server_config
            .event_log_path
            .map(|path| EventLogging::File { path }.to_logger().unwrap());

        let api = api();
        let store = ConfigStore::from_path(server_config.client_config_path)?;
        let context = Context { event_log, store };

        let log = ConfigLogging::File {
            if_exists: server_config.log_mode,
            level: server_config.log_level,
            path: server_config.log_path,
        }
        .to_logger(CHIMNEY)?;

        let tls = server_config.tls_config.map(|config| ConfigTls::AsFile {
            cert_file: config.cert_file.into(),
            key_file: config.key_file.into(),
        });

        ServerBuilder::new(api, context, log)
            .config(config)
            .tls(tls)
            .start()
            .map_err(|error| anyhow!("configuring chimney sync server {:#}", error))?
    };
    Ok(server)
}

pub fn api() -> ApiDescription<Context> {
    crate::api::santa_sync_server_api_mod::api_description::<SantaSyncServerApiImpl>()
        .expect("implementing chimney sync server API endpoints")
}
