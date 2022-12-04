use std::path::Path;

use chrono::{DateTime, TimeZone, Utc};

use crate::date::DateRange;
use crate::download;
use crate::models::{FileData, RawDataHistory};

use super::error::*;

pub async fn get_from_csv(symbol: &str, path: &str, from: &str, to: &str) -> Result<Vec<RawDataHistory>> {
    let date_time = |str| Utc.datetime_from_str(str, "%Y-%m-%d %H:%M:%S");
    let (from, to) = (date_time(from)?, date_time(to)?);

    let files: Vec<FileData> = DateRange(from, to)
        .into_iter()
        .map(|d: DateTime<Utc>| FileData::new(symbol, path, d))
        .collect();

    for file in &files {
        if !Path::new(&file.csv).exists() {
            download::download_data(file).await?;
        }
    }

    let from_milli = from.timestamp_millis();
    let to_milli = to.timestamp_millis();

    let mut result = Vec::new();

    for file in &files {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(&file.csv)?;

        for record in reader.deserialize() {
            match record? {
                RawDataHistory { time, .. } if time < from_milli => continue,
                RawDataHistory { time, .. } if time > to_milli => break,
                r => result.push(r)
            };
        }
    }
    Ok(result)
}
