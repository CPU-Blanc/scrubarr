use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Language {
    pub id: i32,
    pub name: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
pub struct QualityModel {
    pub quality: Quality,
    pub revision: Revision,
}

#[derive(Deserialize, Debug)]
pub struct Quality {
    pub id: i32,
    pub name: Option<Box<str>>,
    pub source: QualitySource,
    pub resolution: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum QualitySource {
    Unknown,
    Television,
    TelevisionRaw,
    Web,
    WebRip,
    Dvd,
    Bluray,
    BlurayRaw,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    pub version: i32,
    pub real: i32,
    pub is_repack: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatResource {
    pub id: i32,
    pub name: Option<Box<str>>,
    pub include_custom_format_when_renaming: Option<bool>,
    pub specifications: Option<Box<[CustomFormatSpecificationSchema]>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatSpecificationSchema {
    pub id: i32,
    pub name: Option<Box<str>>,
    pub implementation: Option<Box<str>>,
    pub implementation_name: Option<Box<str>>,
    pub info_link: Option<Box<str>>,
    pub negate: bool,
    pub required: bool,
    pub fields: Box<[Field]>,
    // pub presets: //Return type for this field is not specified. TODO: return to this
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub order: i32,
    pub name: Option<Box<str>>,
    pub label: Option<Box<str>>,
    pub unit: Option<Box<str>>,
    pub help_text: Option<Box<str>>,
    pub help_text_warning: Option<Box<str>>,
    pub help_link: Option<Box<str>>,
    // pub value: //Return type for this field is not specified. TODO: return to this
    pub r#type: Option<Box<str>>,
    pub advanced: bool,
    pub select_options: Box<[SelectOption]>,
    pub select_options_provider_action: Option<Box<str>>,
    pub section: Option<Box<str>>,
    pub hidden: Option<Box<str>>,
    pub privacy: PrivacyLevel,
    pub placeholder: Option<Box<str>>,
    pub is_float: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectOption {
    pub value: i32,
    pub name: Option<Box<str>>,
    pub order: i32,
    pub hint: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PrivacyLevel {
    Normal,
    Password,
    ApiKey,
    UserName,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ReleaseType {
    Unknown,
    SingleEpisode,
    MultiEpisode,
    SeasonPack,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfoResource {
    pub id: i32,
    pub audio_bitrate: i64,
    pub audio_channels: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaCover {
    pub cover_type: MediaCoverTypes,
    pub url: Option<Box<str>>,
    pub remote_url: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MediaCoverTypes {
    Unknown,
    Poster,
    Banner,
    Fanart,
    Screenshot,
    Headshot,
    Clearlogo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NewItemMonitorTypes {
    All,
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MonitorTypes {
    Unknown,
    All,
    Future,
    Missing,
    Existing,
    FirstSeason,
    LastSeason,
    LatestSeason,
    Pilot,
    Recent,
    MonitorSpecials,
    UnmonitorSpecials,
    None,
    Skip,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    pub votes: i32,
    pub value: f64,
}
