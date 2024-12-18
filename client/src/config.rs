use clap::{Parser, ValueEnum};
use config::{ConfigError, Environment, File};
use directories_next::ProjectDirs;
use dotenv::dotenv;
use is_empty::IsEmpty;
use log::{error, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, path::PathBuf};
use url::Url;
use zeroize::Zeroizing;

// TODO: Deprecate
#[derive(Parser, Debug, IsEmpty)]
#[command(version, about)]
pub(super) struct Args {
    /// Sonarr API key
    pub(super) api_key: Option<String>,

    /// Set log Level
    #[arg(value_enum, short, long)]
    pub(super) log_level: Option<Level>,

    /// Sonarr URL
    #[arg(short, long)]
    pub(super) url: Option<Url>,

    /// Sonarr URL base
    #[arg(short, long)]
    pub(super) base_path: Option<String>,

    /// Queue scan interval (in seconds)
    #[arg(short, long)]
    pub(super) interval: Option<u64>,

    /// Enable verbose http logging
    #[arg(short, long)]
    pub(super) verbose: Option<bool>,
}

#[derive(ValueEnum, Clone, Debug, Default, Serialize, Deserialize)]
pub(super) enum Level {
    Trace,
    Debug,
    #[default]
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

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Sonarr {
    pub(super) base: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_some")]
    pub(super) key: Option<Zeroizing<Box<str>>>,
    pub(super) url: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Config {
    pub(super) log_level: LevelFilter,
    pub(super) interval: u64,
    pub(super) sonarr: HashMap<u8, Sonarr>,
    pub(super) verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info,
            interval: 600,
            sonarr: HashMap::from([(1, Sonarr::default())]),
            verbose: Default::default(),
        }
    }
}

impl Default for Sonarr {
    fn default() -> Self {
        Self {
            base: None,
            key: None,
            url: Url::parse("http://localhost:8989").unwrap(),
        }
    }
}

impl Config {
    pub(super) fn new(path: &str) -> Result<Self, ConfigError> {
        dotenv().ok();
        //TODO: deprecate
        rewrite_env("SCRUBARR_SONARR_URL", "SCRUBARR_SONARR_1_URL");
        rewrite_env("SCRUBARR_SONARR_KEY", "SCRUBARR_SONARR_1_KEY");
        rewrite_env("SCRUBARR_SONARR_BASE_PATH", "SCRUBARR_SONARR_1_BASE");

        config::Config::builder()
            .add_source(Environment::with_prefix("SCRUBARR").separator("_"))
            .add_source(File::with_name(path).required(false))
            .set_default(
                "log_level",
                env::var("SCRUBARR_LOG_LEVEL").unwrap_or(String::from("INFO")),
            )? // Hack, because _ is a separator...
            .set_default("verbose", "false")?
            .set_default("interval", "600")?
            .set_default("sonarr.1.url", "http://localhost:8989")?
            .build()?
            .try_deserialize()
    }

    pub(super) fn merge(&mut self, mut args: Args) {
        let sonarr = self.sonarr.entry(1).or_default();
        if args.api_key.is_some() {
            sonarr.key = Some(Zeroizing::new(Box::from(args.api_key.take().unwrap())))
        };
        if let Some(level) = args.log_level {
            self.log_level = LevelFilter::from(level);
        };
        if let Some(url) = args.url {
            sonarr.url = url;
        };
        if let Some(base_path) = args.base_path {
            sonarr.base = Some(Box::from(base_path));
        };
        if let Some(interval) = args.interval {
            self.interval = std::cmp::max(interval, 300);
        };
        if let Some(verbose) = args.verbose {
            self.verbose = verbose;
        };
    }
}

fn rewrite_env(original: &str, new: &str) {
    if let Ok(value) = env::var(original) {
        env::remove_var(original);
        env::set_var(new, value);
    };
}

pub(super) fn get_config_path() -> PathBuf {
    if let Ok(path) = env::var("X_SCRUBARR_CONFIG") {
        PathBuf::from(path)
    } else if let Some(dir) = ProjectDirs::from("", "", "Scrubarr") {
        dir.config_dir().to_path_buf().join("settings.json")
    } else {
        env::current_exe()
            .expect("error locating exe directory")
            .parent()
            .unwrap()
            .join("settings.json")
    }
}

pub(super) fn write_config_file(config_path: &PathBuf, config: &Config) {
    let parent_dir = config_path.parent().unwrap();

    if !parent_dir.exists() {
        if let Err(e) = fs::create_dir_all(parent_dir) {
            error!(
                "failed to create configuration directory at {dir} - {e}",
                dir = parent_dir.to_str().unwrap()
            );
            return;
        };
    };

    if !&config_path.is_file() {
        if let Err(e) = fs::write(
            config_path,
            serde_json::to_string_pretty(&config).expect("error parsing json"),
        ) {
            error!("error writing configuration to file - {e}");
            return;
        };
        warn!("configuration file generated from CLI arguments. The Sonarr API key is not inserted into the file automatically \
                - either edit the file, or pass the key via environmental variable to continue support");
    };
}
