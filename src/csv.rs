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
        let mut reader = csv::ReaderBuilder::new().from_path(&file.csv)?;
        let mut raw_record = csv::StringRecord::new();
        let headers = headers();

        while reader.read_record(&mut raw_record)? {
            let record = raw_record.deserialize(Some(&headers))?;
            match record {
                RawDataHistory { time, .. } if time < from_milli => continue,
                RawDataHistory { time, .. } if time > to_milli => break,
                r => result.push(r)
            };
        }
    }
    Ok(result)
}

fn headers() -> csv::StringRecord {
    csv::StringRecord::from(vec!["id", "price", "qty", "quote_qty", "time", "is_buyer_maker"])
}