use super::episode::EpisodeResource;
use super::misc::{CustomFormatResource, Language, QualityModel};
use crate::schema::series::SeriesResource;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use struct_iterable::Iterable;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Default,
    Ascending,
    Descending,
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Ascending => write!(f, "ascending"),
            SortDirection::Descending => write!(f, "descending"),
            SortDirection::Default => write!(f, "default"),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DownloadProtocol {
    Unknown,
    Usenet,
    Torrent,
}

impl Display for DownloadProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadProtocol::Torrent => write!(f, "torrent"),
            DownloadProtocol::Usenet => write!(f, "usenet"),
            DownloadProtocol::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum QueueStatus {
    Unknown,
    Queued,
    Paused,
    Downloading,
    Completed,
    Failed,
    Warning,
    Delay,
    DownloadClientUnavailable,
    Fallback,
}

impl Display for QueueStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueStatus::Completed => write!(f, "completed"),
            QueueStatus::Unknown => write!(f, "unknown"),
            QueueStatus::Queued => write!(f, "queued"),
            QueueStatus::Paused => write!(f, "paused"),
            QueueStatus::Downloading => write!(f, "downloading"),
            QueueStatus::Failed => write!(f, "failed"),
            QueueStatus::Warning => write!(f, "warning"),
            QueueStatus::Delay => write!(f, "delay"),
            QueueStatus::DownloadClientUnavailable => write!(f, "downloadClientUnavailable"),
            QueueStatus::Fallback => write!(f, "fallback"),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TrackedDownloadStatus {
    Ok,
    Warning,
    Error,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TrackedDownloadState {
    Downloading,
    ImportBlocked,
    ImportPending,
    Importing,
    Imported,
    FailedPending,
    Failed,
    Ignored,
}
#[derive(Iterable, Default)]
pub struct GetQueueQuery {
    page: Option<Box<str>>,
    page_size: Option<Box<str>>,
    sort_key: Option<Box<str>>,
    sort_direction: Option<Box<str>>,
    include_unknown_series_items: Option<Box<str>>,
    include_series: Option<Box<str>>,
    include_episode: Option<Box<str>>,
    series_ids: Option<Box<str>>,
    protocol: Option<Box<str>>,
    languages: Option<Box<str>>,
    quality: Option<Box<str>>,
    status: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueResourcePagingResource {
    pub page: i32,
    pub page_size: i32,
    pub sort_key: Box<str>,
    pub sort_direction: SortDirection,
    pub total_records: i32,
    pub records: Option<Box<[QueueResource]>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueResource {
    pub id: i32,
    pub series_id: Option<i32>,
    pub episode_id: Option<i32>,
    pub season_number: Option<i32>,
    pub series: Option<SeriesResource>,
    pub episode: Option<EpisodeResource>,
    pub languages: Option<Box<[Language]>>,
    pub quality: QualityModel,
    pub custom_formats: Option<Box<[CustomFormatResource]>>,
    pub custom_format_score: i32,
    pub size: f64,
    pub title: Option<Box<str>>,
    //TODO: deprecated once #7395 is merged into sonarr master - use camel case
    #[serde(rename = "sizeleft")]
    pub size_left: f64,
    //TODO: deprecated once #7395 is merged into sonarr master - use camel case
    #[serde(rename = "timeleft")]
    pub time_left: Option<Box<str>>,
    pub estimated_completion_time: Option<Box<str>>,
    pub added: Option<Box<str>>,
    pub status: QueueStatus,
    pub tracked_download_status: TrackedDownloadStatus,
    pub tracked_download_state: TrackedDownloadState,
    pub status_messages: Option<Box<[TrackedDownloadStatusMessage]>>,
    pub error_message: Option<Box<str>>,
    pub download_id: Option<Box<str>>,
    pub protocol: DownloadProtocol,
    pub download_client: Option<Box<str>>,
    pub download_client_has_post_import_category: bool,
    pub indexer: Option<Box<str>>,
    pub output_path: Option<Box<str>>,
    pub episode_has_file: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackedDownloadStatusMessage {
    pub title: Option<Box<str>>,
    pub messages: Box<[Box<str>]>,
}

impl GetQueueQuery {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(Box::from(page.to_string()));
        self
    }
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(Box::from(page_size.to_string()));
        self
    }
    pub fn sort_key(mut self, sort_key: &str) -> Self {
        self.sort_key = Some(Box::from(sort_key.to_owned()));
        self
    }
    pub fn sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(Box::from(sort_direction.to_string()));
        self
    }
    pub fn include_unknown_series_items(mut self, flag: bool) -> Self {
        self.include_unknown_series_items = Some(Box::from(flag.to_string()));
        self
    }
    pub fn include_series(mut self, flag: bool) -> Self {
        self.include_series = Some(Box::from(flag.to_string()));
        self
    }
    pub fn include_episode(mut self, flag: bool) -> Self {
        self.include_episode = Some(Box::from(flag.to_string()));
        self
    }
    pub fn series_ids(mut self, ids: &[i32]) -> Self {
        self.series_ids = Some(Box::from(
            ids.iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
        ));
        self
    }
    pub fn protocol(mut self, protocol: DownloadProtocol) -> Self {
        self.protocol = Some(Box::from(protocol.to_string()));
        self
    }
    pub fn languages(mut self, languages: &[i32]) -> Self {
        self.languages = Some(Box::from(
            languages
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
        ));
        self
    }
    pub fn quality(mut self, quality: &[i32]) -> Self {
        self.quality = Some(Box::from(
            quality
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
        ));
        self
    }
    pub fn status(mut self, status: QueueStatus) -> Self {
        self.status = Some(Box::from(status.to_string()));
        self
    }
}

#[derive(Iterable, Default)]
pub struct DeleteQueueQuery {
    remove_from_client: Option<Box<str>>,
    blocklist: Option<Box<str>>,
    skip_redownload: Option<Box<str>>,
    change_category: Option<Box<str>>,
}

impl DeleteQueueQuery {
    pub fn builder() -> Self {
        DeleteQueueQuery::default()
    }
    pub fn remove_from_client(mut self, flag: bool) -> Self {
        self.remove_from_client = Some(Box::from(flag.to_string()));
        self
    }
    pub fn blocklist(mut self, flag: bool) -> Self {
        self.blocklist = Some(Box::from(flag.to_string()));
        self
    }
    pub fn skip_redownload(mut self, flag: bool) -> Self {
        self.skip_redownload = Some(Box::from(flag.to_string()));
        self
    }
    pub fn change_category(mut self, flag: bool) -> Self {
        self.change_category = Some(Box::from(flag.to_string()));
        self
    }
}
