use config::{ConfigError, Environment, File};
use directories_next::ProjectDirs;
use dotenv::dotenv;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, io, path::PathBuf};
use url::Url;
use zeroize::Zeroizing;

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

pub(super) fn write_config_file(config_path: &PathBuf, config: &Config) -> Result<(), io::Error> {
    let parent_dir = config_path.parent().unwrap();

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    };

    if !&config_path.is_file() {
        fs::write(
            config_path,
            serde_json::to_string_pretty(&config).expect("error parsing json"),
        )?;
    };
    Ok(())
}
