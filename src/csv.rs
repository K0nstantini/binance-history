use std::path::Path;

use chrono::{DateTime, TimeZone, Utc};

use crate::date::DateRange;
use crate::download;
use crate::model::{FileData, MarketType, DataHistory};

use super::error::*;

pub async fn get_from_csv<T: DataHistory>(
    market: MarketType,
    symbol: &str,
    path: &str,
    from: &str,
    to: &str,
) -> Result<Vec<T>> {
    let date_time = |str| Utc.datetime_from_str(str, "%Y-%m-%d %H:%M:%S");
    let (from, to) = (date_time(from)?, date_time(to)?);

    let files: Vec<FileData> = DateRange(from, to)
        .into_iter()
        .map(|d: DateTime<Utc>| FileData::new(market, symbol, path, d))
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
        let headers = market.headers();

        while reader.read_record(&mut raw_record)? {
            let record = raw_record.deserialize::<T>(Some(&headers))?;
            match record.time() {
                t if t < from_milli => continue,
                t if t > to_milli => break,
                _ => result.push(record)
            }
        }
    }
    Ok(result)
}
