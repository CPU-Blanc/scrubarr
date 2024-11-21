use std::{env, time::{Duration, Instant}};
use std::collections::HashSet;
use tokio::time::sleep;
use sonarr_api::{
    queue::{
        QueueResource,
        QueueStatus,
        GetQueueQuery,
        DeleteQueueQuery
    },
    Sonarr
};

use log::{LevelFilter, info, debug, trace, error, warn};
use log4rs::{
    append::{console::ConsoleAppender},
    encode::pattern::PatternEncoder,
    config::{Appender, Config, Root},
    filter::threshold::ThresholdFilter
};

#[tokio::main]
async fn main() {
    let mut unknown_log = false;
    let log_level_str = env::var("SCRUBARR_LOG_LEVEL").unwrap_or(String::from("info"));
    let log_level = match log_level_str.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => {
            unknown_log = true;
            LevelFilter::Info
        }
    };

    let encoder = PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {D({f}:{L} - )}{m}{n})}");
    let stdout = ConsoleAppender::builder().encoder(Box::new(encoder)).build();
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build("stdout", Box::new(stdout))
        )
        .build(Root::builder().appender("stdout").build(log_level))
        .expect("error building logger");
    let _ = log4rs::init_config(config).expect("error building logger");

    if unknown_log {
        warn!("unknown log level '{log_level_str}' specified");
    };

    info!("Scrubarr v{}", env!("CARGO_PKG_VERSION"));

    let port = if env::var("SCRUBARR_OMIT_PORT").unwrap_or(String::from("false")).parse::<bool>().expect("error parsing omit as bool") {
        debug!("SCRUBARR_OMIT_PORT set - skipping port field");
        None
    } else {
        match env::var("SCRUBARR_SONARR_PORT") {
            Err(_) => {
                debug!("SCRUBARR_SONARR_PORT var missing - using default sonarr port: 8989");
                Some(8989)
            },
            Ok(input) => {
                let x = Some(input.parse::<u16>().expect("error parsing port as int"));
                trace!("mapping to port {}", input);
                x
            }
        }
    };

    let sonarr = sonarr_api::new(
        &env::var("SCRUBARR_SONARR_KEY").expect("no API key provided"),
        &env::var("SCRUBARR_SONARR_URL").unwrap_or(String::from("http://localhost")),
        env::var("SCRUBARR_SONARR_BASE_URL").ok().as_deref(),
        port
    ).expect("error creating Sonarr client");

    let config_interval = Duration::from_secs(
        env::var("SCRUBARR_INTERVAL")
            .unwrap_or(String::from("600"))
            .parse()
            .expect("error parsing INTERVAL as int")
    );

    let mut start_time;

    loop {
        start_time = Instant::now();
        debug!("starting queue check");
        let (tbas, replaced) = process_queue(&sonarr).await;

        trace!("tbas - {:?} - replaced - {:?}", tbas, replaced);

        let (mut success, mut failed): (i32,i32) = (0,0);

        for id in tbas.into_iter().flatten() {
            if let Err(e) = sonarr.refresh_series(&id).await {
                error!("error refreshing series {id} - {e}");
                failed += 1;
            } else {
                success += 1;
            }
        };
        
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
        .page_size(30)
        .include_series(true)
        .status(QueueStatus::Warning);

    match client.get_queue(query).await {
        Err(e) => { 
            error!("error fetching sonarr queue - {e}");
            Box::new([])
        },
        Ok(resource) => {
            resource.records
        }
    }
}

async fn process_queue(client: &Sonarr) -> (HashSet<Option<i32>>, Vec<i32>){
    let mut tbas = HashSet::new();
    let mut replaced = Vec::new();

    'item: for item in get_queue(client).await {
        for statuses in item.status_messages {
            for message in statuses.messages {
                if message.contains("Episode has a TBA title and recently aired") {
                    if tbas.insert(item.series_id) {
                        debug!("found TBA in series [{id}] '{name}'", id = item.series_id.unwrap_or(0), name = item.series.unwrap().title.unwrap_or(Box::from("")));
                    };
                    continue 'item;
                }
                if message.contains("Not a Custom Format upgrade for existing episode file(s)") {
                    replaced.push(item.id);
                    continue 'item;
                };
            };
        };
    };

    (tbas, replaced)
}