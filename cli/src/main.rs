#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

use anyhow::anyhow;
use clap::Parser;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let args = ChimneyArgs::parse();
    let config = chimney_server::ServerConfig::from_file(args.config)?;
    let server = chimney_server::start_server(config).await?;

    println!("Starting chimney sync server on {}", server.local_addr());
    server
        .await
        .map_err(|error| anyhow!("Terminating chimney sync server: {}", error))
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct ChimneyArgs {
    /// Path to the server configuration file
    #[arg(short, long)]
    config: Option<String>,
}
