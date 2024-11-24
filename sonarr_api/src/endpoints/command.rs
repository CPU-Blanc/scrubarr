use crate::{error::SonarrResult, Sonarr};
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RefreshSeriesPayload {
    name: String,
    series_id: i32,
}

impl Sonarr {
    pub async fn refresh_series(&self, series_id: &i32) -> SonarrResult<()> {
        let url = self.build_url("/api/v3/command")?;

        self.client
            .post(url)
            .json(&RefreshSeriesPayload {
                name: String::from("RefreshSeries"),
                series_id: *series_id,
            })
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(())
    }
}
