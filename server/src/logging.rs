use std::fs::OpenOptions;
use std::io;
use std::io::LineWriter;
use std::path::Path;

use camino::Utf8PathBuf;
use slog::{Drain, Level, Logger, o};

#[derive(Debug)]
pub enum EventLogging {
    File { path: Utf8PathBuf },
}

impl EventLogging {
    pub fn to_logger(&self) -> Result<Logger, io::Error> {
        match self {
            EventLogging::File { path } => {
                let mut open_options = std::fs::OpenOptions::new();
                open_options.write(true).create(true).append(true);

                let drain = log_drain_for_file(&open_options, Path::new(path))?;
                Ok(async_root_logger(drain))
            }
        }
    }
}

fn log_drain_for_file(
    open_options: &OpenOptions,
    path: &Path,
) -> Result<slog::Fuse<slog_json::Json<LineWriter<std::fs::File>>>, io::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Buffer writes to the file around newlines to minimize syscalls.
    let file = LineWriter::new(open_options.open(path)?);

    Ok(slog::Fuse(slog_json::Json::new(file).build()))
}

fn async_root_logger<T>(drain: T) -> slog::Logger
where
    T: slog::Drain + Send + 'static,
    <T as slog::Drain>::Err: std::fmt::Debug,
{
    let level_drain = slog::LevelFilter(drain, Level::Info).fuse();
    let async_drain = slog_async::Async::new(level_drain)
        .chan_size(1024)
        .build()
        .fuse();
    slog::Logger::root(async_drain, o!())
}
