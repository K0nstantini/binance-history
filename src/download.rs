use std::{fs, io};
use std::fs::File;

use reqwest::StatusCode;

use crate::error::Error::Download;
use crate::model::FileData;

use super::error::*;

pub async fn download_data(file: &FileData) -> Result<()> {
    download_trades(file).await?;
    extract_file(file)?;
    Ok(())
}

async fn download_trades(file: &FileData) -> Result<()> {
    let response = reqwest::get(&file.url).await?;
    let body = match response.status() {
        StatusCode::OK => response.bytes().await?,
        status => return Err(Download(file.url.clone(), status.to_string()))
    };

    let mut out = File::create(&file.zip)?;
    io::copy(&mut body.as_ref(), &mut out)?;
    Ok(())
}

fn extract_file(file: &FileData) -> Result<()> {
    let zip = File::open(&file.zip)?;
    let mut archive = zip::ZipArchive::new(zip)?;
    let mut contained_zip = archive.by_index(0)?;

    let mut outfile = File::create(&file.csv)?;
    io::copy(&mut contained_zip, &mut outfile)?;
    fs::remove_file(&file.zip)?;
    Ok(())
}