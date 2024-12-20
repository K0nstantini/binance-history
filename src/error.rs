use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {

    #[error("The directory '{0}' does not exist")]
    DirectoryNotFound(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error("The archive '{0}' is empty and cannot be extracted")]
    EmptyArchiveError(String),

    #[error(transparent)]
    Csv(#[from] csv::Error),

    #[error("Failed to download the file from '{0}'. HTTP status code: {1}")]
    DownloadError(String, String),

    #[error("The kline's interval is missing")]
    MissingKlinesInterval,
}