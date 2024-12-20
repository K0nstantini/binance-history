use crate::error::{Error::*, Result};
use crate::model::Config;
use crate::util;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};
use url::Url;

struct FileData {
    pub csv: PathBuf,
    pub zip: PathBuf,
    pub url: Url,
}

impl FileData {
    fn new(config: &Config, date: DateTime<Utc>) -> Result<Self> {
        let file_name = config.data_type.file_name(&config.symbol, date);
        let file_path = config.path.join(&file_name);

        Ok(Self {
            csv: file_path.with_extension(".csv"),
            zip: file_path.with_extension(".zip"),
            url: config.url()?.join(&format!("{file_name}.zip"))?,
        })
    }
}

pub async fn download_files(config: &Config) -> Result<Vec<PathBuf>> {
    let files: Vec<_> = util::get_dates_from_range(config.from, config.to)
        .into_iter()
        .map(|date| FileData::new(config, date))
        .collect::<Result<Vec<_>>>()?;

    for file in files.iter().filter(|f| !f.csv.exists()) {
        download_file(file).await?;
    }
    Ok(files.into_iter().map(|f| f.csv).collect())
}

async fn download_file(file: &FileData) -> Result<()> {
    download_to_zip(file).await?;
    if let Err(e) = extract_zip(file) {
        let _ = fs::remove_file(&file.zip);
        return Err(e);
    }
    Ok(())
}

async fn download_to_zip(file: &FileData) -> Result<()> {
    let response = reqwest::get(file.url.as_ref()).await?;
    let body = match response.status() {
        StatusCode::OK => response.bytes().await?,
        status => return Err(DownloadError(file.url.to_string(), status.to_string()))
    };

    let mut out = File::create(&file.zip)?;
    io::copy(&mut body.as_ref(), &mut out)?;
    Ok(())
}

fn extract_zip(file: &FileData) -> Result<()> {
    let zip = File::open(&file.zip)?;
    let mut archive = zip::ZipArchive::new(zip)?;

    if archive.is_empty() {
        return Err(EmptyArchiveError(file.url.to_string()));
    }

    let mut archive_entry = archive.by_index(0)?;
    let mut outfile = File::create(&file.csv)?;

    io::copy(&mut archive_entry, &mut outfile)?;
    fs::remove_file(&file.zip)?;
    Ok(())
}