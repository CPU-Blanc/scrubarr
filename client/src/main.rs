mod config;

use sonarr_api::{
    queue::{DeleteQueueQuery, GetQueueQuery, QueueResource, QueueStatus},
    Sonarr,
};
use std::collections::HashSet;
use std::{
    env,
    time::{Duration, Instant},
};
use tokio::time::sleep;

use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

#[tokio::main]
async fn main() {
    let mut args = config::Args::from_args_and_env();

    let encoder = PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {D({f}:{L} - )}{m}{n})}");
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::from(args.log_level)),
        )
        .expect("error building logger");
    let _ = log4rs::init_config(config).expect("error building logger");

    info!("Scrubarr v{}", env!("CARGO_PKG_VERSION"));

    //TODO: Deprecate port
    let sonarr_url = if let Some(port) = args.port {
        warn!("SCRUBARR_SONARR_PORT is deprecated and will be removed soon. Include the port in SCRUBARR_SONARR_URL");
        args.url
            .set_port(Some(port))
            .expect("failed to map port to url");
        args.url.to_string()
    } else {
        args.url.to_string()
    };

    if args.omit_port.is_some() {
        warn!("SCRUBARR_OMIT_PORT is deprecated and will be removed soon")
    };

    let sonarr = sonarr_api::new(
        &args.api_key,
        &sonarr_url,
        args.base_path.as_deref(),
        args.verbose,
    )
    .expect("error creating Sonarr client");

    let config_interval = Duration::from_secs(std::cmp::max(args.interval, 300));

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
