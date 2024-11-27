use thiserror::Error;

pub(crate) type SonarrResult<T> = Result<T, SonarrError>;
#[derive(Error, Debug)]
pub enum SonarrError {
    #[error("http request failed: {0}")]
    HttpFailed(#[from] reqwest::Error),
    #[error("url parsing failed: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
}
