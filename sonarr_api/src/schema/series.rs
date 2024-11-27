use super::misc::{Language, MediaCover, MonitorTypes, NewItemMonitorTypes, Ratings};
use super::season::SeasonResource;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesResource {
    pub id: i32,
    pub title: Option<Box<str>>,
    pub alternate_titles: Option<Box<[AlternateTitleResource]>>,
    pub sort_title: Option<Box<str>>,
    pub status: SeriesStatusType,
    pub ended: bool,
    pub profile_name: Option<Box<str>>,
    pub overview: Option<Box<str>>,
    pub next_airing: Option<Box<str>>,
    pub previous_airing: Option<Box<str>>,
    pub network: Option<Box<str>>,
    pub air_time: Option<Box<str>>,
    pub images: Option<Box<[MediaCover]>>,
    pub original_language: Language,
    pub remote_poster: Option<Box<str>>,
    pub seasons: Option<Box<[SeasonResource]>>,
    pub year: i32,
    pub path: Option<Box<str>>,
    pub quality_profile_id: i32,
    pub season_folder: bool,
    pub monitored: bool,
    pub monitor_new_items: NewItemMonitorTypes,
    pub use_scene_numbering: bool,
    pub runtime: i32,
    pub tvdb_id: i32,
    pub tv_rage_id: i32,
    pub tv_maze_id: i32,
    pub tmdb_id: i32,
    pub first_aired: Option<Box<str>>,
    pub last_aired: Option<Box<str>>,
    pub series_type: SeriesTypes,
    pub clean_title: Option<Box<str>>,
    pub imdb_id: Option<Box<str>>,
    pub title_slug: Option<Box<str>>,
    pub root_folder_path: Option<Box<str>>,
    pub folder: Option<Box<str>>,
    pub certification: Option<Box<str>>,
    pub genres: Option<Box<[Box<str>]>>,
    pub tags: Option<Box<[i32]>>,
    pub added: Box<str>,
    pub add_options: Option<AddSeriesOptions>,
    pub ratings: Ratings,
    pub statistics: Option<SeriesStatisticsResource>,
    pub episodes_changed: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTitleResource {
    pub title: Option<Box<str>>,
    pub season_number: Option<i32>,
    pub scene_season_number: Option<i32>,
    pub scene_origin: Option<Box<str>>,
    pub comment: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SeriesTypes {
    Standard,
    Daily,
    Anime,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesOptions {
    pub ignore_episodes_with_files: bool,
    pub ignore_episodes_without_files: bool,
    pub monitor: MonitorTypes,
    pub search_for_missing_episodes: bool,
    pub search_for_cutoff_unmet_episodes: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesStatisticsResource {
    season_count: i32,
    episode_file_count: i32,
    episode_count: i32,
    total_episode_count: i32,
    size_on_disk: i64,
    release_groups: Option<Box<[Box<str>]>>,
    percent_of_episodes: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SeriesStatusType {
    Continuing,
    Ended,
    Upcoming,
    Deleted,
}
