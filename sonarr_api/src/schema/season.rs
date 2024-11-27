use super::misc::MediaCover;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeasonResource {
    pub season_number: i32,
    pub monitored: bool,
    pub statistics: Option<SeasonStatisticsResource>,
    pub images: Option<Box<[MediaCover]>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeasonStatisticsResource {
    next_airing: Option<Box<str>>,
    previous_airing: Option<Box<str>>,
    episode_file_count: i32,
    episode_count: i32,
    total_episode_count: i32,
    size_on_disk: i64,
    release_groups: Option<Box<[Box<str>]>>,
    percent_of_episodes: f64,
}
