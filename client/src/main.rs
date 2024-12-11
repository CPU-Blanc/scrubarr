mod config;
mod instance;

use config::{get_config_path, Config};
use instance::SonarrInstance;
use log::{debug, error, info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config as LogConfig, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::{
    env,
    process::exit,
    time::{Duration, Instant},
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let config_path = get_config_path();
    let config = Config::new(config_path.to_str().unwrap()).unwrap_or_else(|e| {
        build_logger(LevelFilter::Info);
        error!("error loading config - {e}");
        exit(1);
    });

    build_logger(config.log_level);

    info!("Scrubarr v{}", env!("CARGO_PKG_VERSION"));

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
