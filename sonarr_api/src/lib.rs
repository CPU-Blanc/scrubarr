#![allow(dead_code)]

mod error;
mod endpoints {
    mod command;
    mod queue;
}
mod schema {
    pub mod episode;
    pub mod misc;
    pub mod queue;
    pub mod season;
    pub mod series;
}

use crate::error::SonarrResult;
use convert_case::{Case, Casing};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Url,
};
pub use schema::{episode, misc, queue, season, series};
use struct_iterable::Iterable;

pub struct Sonarr {
    base_path: Box<str>,
    client: Client,
    host: Box<str>,
}

pub fn new(
    api_key: &str,
    url: &str,
    base_path: Option<&str>,
    verbose: bool,
) -> SonarrResult<Sonarr> {
    let mut headers = HeaderMap::new();
    let mut header_vale = HeaderValue::from_str(api_key)?;
    header_vale.set_sensitive(true);
    headers.insert("X-Api-Key", header_vale);
    let client = Client::builder()
        .default_headers(headers)
        .connection_verbose(verbose)
        .build()?;
    Ok(Sonarr {
        base_path: Box::from(base_path.unwrap_or_default()),
        client,
        host: Box::from(url),
    })
}

impl Sonarr {
    fn build_url(&self, path: &str) -> SonarrResult<Url> {
        let mut url = Url::parse(&self.host)?;
        url.set_path(&format!("{}{path}", &self.base_path));
        Ok(url)
    }
}

fn build_query_string<T: Iterable>(build_query: T) -> Option<String> {
    let mut query = Vec::new();
    for (name, value) in build_query.iter() {
        if let Some(Some(boxed)) = value.downcast_ref::<Option<Box<str>>>() {
            query.push(format!(
                "{parameter}={value}",
                parameter = name.to_case(Case::Camel),
                value = boxed
            ));
        };
    }

    if query.is_empty() {
        None
    } else {
        Some(query.join("&"))
    }
}
