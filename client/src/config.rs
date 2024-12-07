use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use log::LevelFilter;
use url::Url;

#[derive(Parser)]
#[command(version, about)]
pub(super) struct Args {
    /// Sonarr API key
    #[arg(env("SCRUBARR_SONARR_KEY"))]
    pub(super) api_key: String,

    /// Set log Level
    #[arg(value_enum, short, long, env="SCRUBARR_LOG_LEVEL", default_value_t=Level::Info)]
    pub(super) log_level: Level,

    /// Sonarr port
    #[arg(short, long, env = "SCRUBARR_SONARR_PORT")]
    pub(super) port: Option<u16>,

    /// Whether to omit the port from the url
    #[arg(long, env = "SCRUBARR_OMIT_PORT")]
    pub(super) omit_port: Option<bool>,

    /// Sonarr URL
    #[arg(
        short,
        long,
        env = "SCRUBARR_SONARR_URL",
        default_value = "http://localhost:8989"
    )]
    pub(super) url: Url,

    /// Sonarr URL base
    #[arg(short, long, env = "SCRUBARR_SONARR_BASE_PATH")]
    pub(super) base_path: Option<String>,

    /// Queue scan interval (in seconds)
    #[arg(short, long, env = "SCRUBARR_INTERVAL", default_value_t = 600)]
    pub(super) interval: u64,

    /// Enable verbose http logging
    #[arg(short, long, env = "SCRUBARR_VERBOSE", default_value_t = false)]
    pub(super) verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub(super) enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<Level> for LevelFilter {
    fn from(value: Level) -> Self {
        match value {
            Level::Trace => LevelFilter::Trace,
            Level::Debug => LevelFilter::Debug,
            Level::Info => LevelFilter::Info,
            Level::Warn => LevelFilter::Warn,
            Level::Error => LevelFilter::Error,
        }
    }
}

impl Args {
    pub(super) fn from_args_and_env() -> Self {
        dotenv().ok();
        Self::parse()
    }
}
