use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesResource {
    pub id: i32,
    pub title: Option<Box<str>>,
    // alternate_titles: Box<[AlternateTitleResource]>, //TODO
    pub sort_title: Option<Box<str>>,
    pub ended: bool,
    pub profile_name: Option<Box<str>>,
    pub overview: Option<Box<str>>,
    pub next_airing: Option<Box<str>>,
    pub previous_airing: Option<Box<str>>,
    pub network: Option<Box<str>>,
    pub air_time: Option<Box<str>>,
    // images: Box<[MediaCover]>,   //TODO
    // original_language: Language, //TODO
    pub remote_poster: Option<Box<str>>,
    // seasons: Box<[SeasonResource]>,  //TODO
    pub year: i32,
    pub season_folder: bool,
    pub monitored: bool,
    // monitor_new_items: NewItemMonitorTypes,  //TODO
    pub use_scene_numbering: bool,
    pub runtime: i32,
    pub tvdb_id: i32,
    pub tv_rage_id: i32,
    pub tv_maze_id: i32,
    pub first_aired: Option<Box<str>>,
    pub last_aired: Option<Box<str>>,
    // series_type: SeriesTypes,    //TODO
    pub clean_title: Option<Box<str>>,
    pub imdb_id: Option<Box<str>>,
    pub title_slug: Option<Box<str>>,
    pub root_folder_path: Option<Box<str>>,
    pub folder: Option<Box<str>>,
    pub certification: Option<Box<str>>,
    pub genres: Box<[Box<str>]>,
    pub tags: Box<[i32]>,
    pub added: Box<str>,
    // add_options: AddSeriesOptions,   //TODO
    // ratings: Ratings,    //TODO
    // statistics: SeriesStatisticsResource,    //TODO
    pub episodes_changed: Option<bool>,
}
