use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {

    #[error("The directory '{0}' does not exist")]
    DirectoryExistence(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    Csv(#[from] csv::Error),

    #[error("Invalid date format")]
    InvalidDateFormat,

    #[error("Error while downloading file: {0}. Status code: {1}")]
    Download(String, String),

    #[error("Missed kline's interval")]
    MissedKlinesInterval,
}