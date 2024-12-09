use log::{debug, error, info, trace};
use sonarr_api::queue::{DeleteQueueQuery, GetQueueQuery, QueueResource, QueueStatus};
use sonarr_api::Sonarr;
use std::collections::HashSet;

pub(super) struct SonarrInstance {
    pub(super) idx: u8,
    pub(super) sonarr: Sonarr,
}

impl SonarrInstance {
    pub(super) async fn process(&self) {
        let (tbas, replaced) = self.process_queue().await;
        let idx = self.idx;

        trace!(
            "[sonarr-{idx}]: tbas - {:?} - replaced - {:?}",
            tbas,
            replaced
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

        if !replaced.is_empty() {
            let query = DeleteQueueQuery::builder().remove_from_client(true);
            if let Err(e) = self.sonarr.queue_delete_bulk(&replaced, query).await {
                error!("[sonarr-{idx}]: error performing bulk delete - {e}");
            } else {
                info!("[sonarr-{idx}]: bulk deleted {} items", replaced.len());
            };
        };
    }

    async fn process_queue(&self) -> (HashSet<Option<i32>>, Vec<i32>) {
        let mut tbas = HashSet::new();
        let mut replaced = Vec::new();

        'item: for item in self.get_queue().await {
            for statuses in item.status_messages.unwrap_or_default() {
                for message in statuses.messages {
                    if message.contains("Episode has a TBA title and recently aired") {
                        if tbas.insert(item.series_id) {
                            debug!(
                                "[sonarr-{idx}]: found TBA in series [{id}] '{name}'",
                                id = item.series_id.unwrap_or_default(),
                                name = item.series.unwrap().title.unwrap_or_default(),
                                idx = self.idx,
                            );
                        };
                        continue 'item;
                    }
                    if message.contains("Not a Custom Format upgrade for existing episode file(s)")
                    {
                        replaced.push(item.id);
                        continue 'item;
                    };
                }
            }
        }

        (tbas, replaced)
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
