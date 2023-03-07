use std::path::Path;

use chrono::{DateTime, TimeZone, Utc};

use crate::{BinanceData, download};
use crate::error::{Error, Result};
use crate::model::{Config, FileData};
use crate::util::date_range::DateRange;

pub async fn get<T: BinanceData>(
    symbol: &str,
    interval: Option<&str>,
    from: &str,
    to: &str,
    path: &str,
) -> Result<Vec<T>> {
    let config = Config::new::<T>(path, interval)?;
    get_from_csv(&config, symbol, from, to).await
}

async fn get_from_csv<T: BinanceData>(config: &Config, symbol: &str, from: &str, to: &str) -> Result<Vec<T>> {
    let date_time = |str| Utc.datetime_from_str(str, "%Y-%m-%d %H:%M:%S");
    let (from, to) = match (date_time(from), date_time(to)) {
        (Ok(from), Ok(to)) => (from, to),
        _ => return Err(Error::InvalidDateFormat)
    };

    let files: Vec<FileData> = DateRange(from, to)
        .into_iter()
        .map(|d: DateTime<Utc>| FileData::new(config, symbol, d))
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
            .has_headers(false)
            .from_path(&file.csv)?;
        let mut raw_record = csv::StringRecord::new();
        let headers = config.headers();

        let mut maybe_header = true;
        while reader.read_record(&mut raw_record)? {
            let record = match raw_record.deserialize::<T>(Some(&headers)) {
                Ok(r) => Ok(r),
                Err(e) => if maybe_header {
                    maybe_header = false;
                    continue;
                } else { Err(e) }
            }?;
            match record.time() {
                t if t < from_milli => continue,
                t if t > to_milli => break,
                _ => result.push(record)
            }
        }
    }
    Ok(result)
}
