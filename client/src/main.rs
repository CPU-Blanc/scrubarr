mod config;

use clap::Parser;
use config::{get_config_path, write_config_file, Args, Config};
use is_empty::IsEmpty;
use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config as LogConfig, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use sonarr_api::{
    queue::{DeleteQueueQuery, GetQueueQuery, QueueResource, QueueStatus},
    Sonarr,
};
use std::{
    collections::HashSet,
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

    if used_cli {
        warn!("use of CLI arguments is depreciated - edit the configuration file ({}) or set environmental variables instead", config_path.to_str().unwrap());
        write_config_file(&config_path, &config);
    };

    if config.sonarr_key.is_none() {
        error!("no Sonarr API key configured - Set it in either the configuration file ({}) or SCRUBARR_SONARR_KEY env variable", config_path.to_str().unwrap());
        exit(1);
    }

    let sonarr = sonarr_api::new(
        config.sonarr_key.unwrap().as_ref(),
        config.sonarr_url.as_ref(),
        config.sonarr_base_path.as_deref(),
        config.verbose,
    )
    .expect("error creating Sonarr client");

    let config_interval = Duration::from_secs(std::cmp::max(config.interval, 300));

    let mut start_time;

    loop {
        start_time = Instant::now();
        debug!("starting queue check");
        let (tbas, replaced) = process_queue(&sonarr).await;

        trace!("tbas - {:?} - replaced - {:?}", tbas, replaced);

        let (mut success, mut failed): (i32, i32) = (0, 0);

        for id in tbas.into_iter().flatten() {
            if let Err(e) = sonarr.refresh_series(&id).await {
                error!("error refreshing series {id} - {e}");
                failed += 1;
            } else {
                success += 1;
            }
        }

        if success.is_positive() || failed.is_positive() {
            info!("refreshed {success} series ({failed} failed)");
        };

        if !replaced.is_empty() {
            let query = DeleteQueueQuery::builder().remove_from_client(true);
            if let Err(e) = sonarr.queue_delete_bulk(&replaced, query).await {
                error!("error performing bulk delete - {e}");
            } else {
                info!("bulk deleted {} items", replaced.len());
            };
        };

        info!("rescanning queue in {} seconds", config_interval.as_secs());
        sleep(config_interval - start_time.elapsed()).await;
    }
}

async fn get_queue(client: &Sonarr) -> Box<[QueueResource]> {
    let query = GetQueueQuery::builder()
        .page_size(1000)
        .include_series(true)
        .status(QueueStatus::Completed);

    match client.get_queue(query).await {
        Err(e) => {
            error!("error fetching sonarr queue - {e}");
            Box::new([])
        }
        Ok(resource) => resource.records.unwrap_or_default(),
    }
}

async fn process_queue(client: &Sonarr) -> (HashSet<Option<i32>>, Vec<i32>) {
    let mut tbas = HashSet::new();
    let mut replaced = Vec::new();

    'item: for item in get_queue(client).await {
        for statuses in item.status_messages.unwrap_or_default() {
            for message in statuses.messages {
                if message.contains("Episode has a TBA title and recently aired") {
                    if tbas.insert(item.series_id) {
                        debug!(
                            "found TBA in series [{id}] '{name}'",
                            id = item.series_id.unwrap_or_default(),
                            name = item.series.unwrap().title.unwrap_or_default()
                        );
                    };
                    continue 'item;
                }
                if message.contains("Not a Custom Format upgrade for existing episode file(s)") {
                    replaced.push(item.id);
                    continue 'item;
                };
            }
        }
    }

    (tbas, replaced)
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
