use log::{debug, error, info, trace};
use sonarr_api::queue::{DeleteQueueQuery, GetQueueQuery, QueueResource, QueueStatus};
use sonarr_api::Sonarr;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub(super) struct SonarrInstance {
    pub(super) idx: u8,
    pub(super) sonarr: Sonarr,
}

impl SonarrInstance {
    pub(super) async fn process(&self) {
        let (tbas, to_delete) = self.process_queue().await;
        let idx = self.idx;

        trace!(
            "[sonarr-{idx}]: tbas - {:?} - replaced - {:?}",
            tbas,
            to_delete
        );

        let (mut success, mut failed): (i32, i32) = (0, 0);

        for id in tbas.into_iter().flatten() {
            if let Err(e) = self.sonarr.refresh_series(&id).await {
                error!("[sonarr-{idx}]: error refreshing series {id} - {e}");
                failed += 1;
            } else {
                success += 1;
            }
        }

        if success.is_positive() || failed.is_positive() {
            info!("[sonarr-{idx}]: refreshed {success} series ({failed} failed)");
        };

        if !to_delete.is_empty() {
            let query = DeleteQueueQuery::builder().remove_from_client(true);
            if let Err(e) = self.sonarr.queue_delete_bulk(&to_delete, query).await {
                error!("[sonarr-{idx}]: error performing bulk delete - {e}");
            } else {
                info!("[sonarr-{idx}]: bulk deleted {} items", to_delete.len());
            };
        };
    }

    async fn process_queue(&self) -> (HashSet<Option<i32>>, Vec<i32>) {
        let mut tbas = HashSet::new();
        let mut to_delete = HashSet::new();
        let mut top_score = HashMap::new();

        'item: for item in self.get_queue().await {
            if self.replace(&mut top_score, &mut to_delete, &item) {
                continue;
            };

            for statuses in item.status_messages.unwrap_or_default() {
                let title = statuses.title.unwrap_or_default();

                if title.contains("Episode has a TBA title and recently aired")
                {
                    if tbas.insert(item.series_id) {
                        debug!(
                            "[sonarr-{idx}]: found TBA in series [{id}] '{name}'",
                            id = item.series_id.unwrap_or_default(),
                            name = item.series.unwrap().title.unwrap_or_default(),
                            idx = self.idx,
                        );
                    };
                    continue 'item;
                };

                if title.contains("Not an upgrade for existing episode file(s)") {
                    to_delete.insert(item.id);
                    continue 'item;
                }

                for message in statuses.messages {
                    if message.contains("Not a Custom Format upgrade for existing episode file(s)")
                    {
                        to_delete.insert(item.id);
                        continue 'item;
                    };
                }
            }
        }

        (tbas, to_delete.into_iter().collect())
    }

    fn replace(
        &self,
        top_score: &mut HashMap<Box<str>, (i32, i32)>,
        to_replace: &mut HashSet<i32>,
        item: &QueueResource,
    ) -> bool {
        if item.series_id.is_none() || item.episode_id.is_none() {
            return false;
        };

        let key = Box::from(format!(
            "{}-{}",
            item.series_id.unwrap(),
            item.episode_id.unwrap()
        ));

        let current = top_score
            .entry(key)
            .or_insert((item.id, item.custom_format_score));
        let (id, score) = current;

        match (*score).cmp(&item.custom_format_score) {
            Ordering::Equal => false,
            Ordering::Less => {
                debug!(
                    "[sonarr-{idx}]: item in queue {id} superseded by {} - Score: {score} -> {}",
                    item.id,
                    item.custom_format_score,
                    idx = self.idx
                );
                to_replace.insert(*id);
                *current = (item.id, item.custom_format_score);
                false
            }
            Ordering::Greater => {
                debug!(
                    "[sonarr-{idx}]: item in queue {} superseded by {id} - Score: {} -> {score}",
                    item.id,
                    item.custom_format_score,
                    idx = self.idx
                );
                to_replace.insert(item.id);
                true
            }
        }
    }

    async fn get_queue(&self) -> Box<[QueueResource]> {
        let query = GetQueueQuery::builder()
            .page_size(1000)
            .include_series(true)
            .status(QueueStatus::Completed);

        match self.sonarr.get_queue(query).await {
            Err(e) => {
                error!(
                    "[sonarr-{idx}]: error fetching sonarr queue - {e}",
                    idx = self.idx
                );
                Box::new([])
            }
            Ok(resource) => resource.records.unwrap_or_default(),
        }
    }
}
