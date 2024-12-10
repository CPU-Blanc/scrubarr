mod config;
mod instance;

use clap::Parser;
use config::{get_config_path, write_config_file, Args, Config};
use instance::SonarrInstance;
use is_empty::IsEmpty;
use log::{debug, error, info, warn, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config as LogConfig, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::{
    process::exit,
    time::{Duration, Instant},
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let used_cli = !args.is_empty();
    let config_path = get_config_path();

    let config = match Config::new(config_path.to_str().unwrap()) {
        Ok(mut loaded) => {
            if used_cli {
                loaded.merge(args);
            };
            build_logger(loaded.log_level);
            loaded
        }
        Err(e) => {
            let mut new_config = Config::default();
            if used_cli {
                new_config.merge(args);
            };
            build_logger(new_config.log_level);
            error!("error loading config - {e}");
            new_config
        }
    };

    info!("Scrubarr v{}", env!("CARGO_PKG_VERSION"));

    if used_cli {
        warn!("use of CLI arguments is depreciated - edit the configuration file ({}) or set environmental variables instead", config_path.to_str().unwrap());
        write_config_file(&config_path, &config);
    };

    let mut clients = Vec::with_capacity(config.sonarr.len());
    for (idx, instance) in config.sonarr {
        if instance.key.is_none() {
            error!("no Sonarr API key configured for instance {idx} - Set it in either the configuration file ({}) or SCRUBARR_SONARR_{idx}_KEY env variable", config_path.to_str().unwrap());
            exit(1);
        };

        clients.push(SonarrInstance {
            idx,
            sonarr: sonarr_api::new(
                instance.key.unwrap().as_ref(),
                instance.url.as_ref(),
                instance.base.as_deref(),
                config.verbose,
            )
            .expect("error creating Sonarr client"),
        });
    }

    let config_interval = Duration::from_secs(std::cmp::max(config.interval, 300));

    let mut start_time;

    loop {
        start_time = Instant::now();
        debug!("starting queue check");

        for instance in &clients {
            debug!("started processing for instance {}", instance.idx);
            instance.process().await;
        }

        info!("rescanning queue in {} seconds", config_interval.as_secs());
        sleep(config_interval - start_time.elapsed()).await;
    }
}

fn build_logger(level: LevelFilter) {
    let encoder = PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {D({f}:{L} - )}{m}{n})}");
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();
    let config = LogConfig::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build("stdout", Box::new(stdout)),
        )
        .build(Root::builder().appender("stdout").build(level))
        .expect("error building logger");
    let _ = log4rs::init_config(config).expect("error building logger");
}
