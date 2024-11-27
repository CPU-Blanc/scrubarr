use super::misc::{
    CustomFormatResource, Language, MediaCover, MediaInfoResource, QualityModel, ReleaseType,
};
use super::series::SeriesResource;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeResource {
    pub id: i32,
    pub series_id: i32,
    pub tvdb_id: i32,
    pub episode_file_id: i32,
    pub season_number: i32,
    pub episode_number: i32,
    pub title: Option<Box<str>>,
    pub air_date: Option<Box<str>>,
    pub air_date_utc: Option<Box<str>>,
    pub last_search_time: Option<Box<str>>,
    pub runtime: i32,
    pub finale_type: Option<Box<str>>,
    pub overview: Option<Box<str>>,
    pub episode_file: Option<EpisodeFileResource>,
    pub has_file: bool,
    pub monitored: bool,
    pub absolute_episode_number: Option<i32>,
    pub scene_absolute_episode_number: Option<i32>,
    pub scene_episode_number: Option<i32>,
    pub scene_season_number: Option<i32>,
    pub unverified_scene_numbering: bool,
    pub end_time: Option<Box<str>>,
    pub grab_date: Option<Box<str>>,
    pub series: Option<SeriesResource>,
    pub images: Option<Box<[MediaCover]>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFileResource {
    pub id: i32,
    pub series_id: i32,
    pub season_number: i32,
    pub relative_path: Option<Box<str>>,
    pub path: Option<Box<str>>,
    pub size: i64,
    pub date_added: Box<str>,
    pub scene_name: Option<Box<str>>,
    pub release_group: Option<Box<str>>,
    pub languages: Box<[Language]>,
    pub quality: QualityModel,
    pub custom_formats: Box<[CustomFormatResource]>,
    pub custom_format_score: i32,
    pub indexer_flags: Option<i32>,
    pub release_type: ReleaseType,
    pub media_info: MediaInfoResource,
    pub quality_cutoff_not_met: bool,
}
