use crate::{
    build_query_string,
    error::SonarrResult,
    queue::DeleteQueueQuery,
    Sonarr,
    schema::queue::{QueueResourcePagingResource, GetQueueQuery}
};

impl Sonarr {
    pub async fn get_queue(&self, query: GetQueueQuery) -> SonarrResult<QueueResourcePagingResource> {
        let mut url = self.build_url("/api/v3/queue")?;
        url.set_query(build_query_string(query).as_deref());

        Ok(
            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json::<QueueResourcePagingResource>()
                .await?
        )
    }
    pub async fn queue_delete_item(&self, id: i32, query: DeleteQueueQuery) -> SonarrResult<()> {
        let mut url = self.build_url(&format!("/api/v3/queue/{id}"))?;
        url.set_query(build_query_string(query).as_deref());

        self.client
            .delete(url)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
    pub async fn queue_delete_bulk(&self, ids: &[i32], query: DeleteQueueQuery) -> SonarrResult<()> {
        let mut url = self.build_url("/api/v3/queue/bulk")?;
        url.set_query(build_query_string(query).as_deref());
        
        self.client
            .delete(url)
            .json(ids)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}