use crate::error::Result;
use crate::model::Config;
use crate::{download, util, BinanceData};
use chrono::{DateTime, Utc};
use std::path::Path;

/// Fetches trade data for a specified symbol within a given time range.
///
/// This function retrieves historical trade data (e.g., aggregated trades, spot trades)
/// for the specified symbol and saves the corresponding CSV file in the provided path.
///
/// # Type Parameters
///
/// * `T` - A type that implements the [`BinanceData`] trait, representing the specific
///   trade data format to retrieve (e.g., spot trades or USD-M aggregated trades).
/// * `P` - A type that implements [`AsRef<Path>`], representing the directory path where
///   the CSV files will be stored.
///
/// # Arguments
///
/// * `symbol` - The trading pair symbol (e.g., `"BTCUSDT"`).
/// * `from` - The start of the time range for the data request, as a [`DateTime<Utc>`].
/// * `to` - The end of the time range for the data request, as a [`DateTime<Utc>`].
/// * `path` - The directory path to save the CSV files.
///
/// # Returns
///
/// A vector of parsed trade data of type `T`.
pub fn get_trades<T: BinanceData, P: AsRef<Path>>(
    symbol: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    path: P,
) -> Result<Vec<T>> {
    get_data::<T, P>(symbol, None, from, to, path)
}

/// Fetches candlestick (kline) data for a specified symbol within a given time range.
///
/// This function retrieves historical kline (candlestick) data for the specified symbol and
/// interval, saving the corresponding CSV file in the provided path.
///
/// # Type Parameters
///
/// * `T` - A type that implements the [`BinanceData`] trait, representing the specific
///   kline data format to retrieve.
/// * `P` - A type that implements [`AsRef<Path>`], representing the directory path where
///   the CSV files will be stored.
///
/// # Arguments
///
/// * `symbol` - The trading pair symbol (e.g., `"BTCUSDT"`).
/// * `interval` - The candlestick interval (e.g., `"1m"`, `"1h"`, `"1d"`).
/// * `from` - The start of the time range for the data request, as a [`DateTime<Utc>`].
/// * `to` - The end of the time range for the data request, as a [`DateTime<Utc>`].
/// * `path` - The directory path to save the CSV files.
///
/// # Returns
///
/// A vector of parsed kline data of type `T`.
pub fn get_klines<T: BinanceData, P: AsRef<Path>>(
    symbol: &str,
    interval: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    path: P,
) -> Result<Vec<T>> {
    get_data::<T, P>(symbol, Some(interval), from, to, path)
}

fn get_data<T: BinanceData, P: AsRef<Path>>(
    symbol: &str,
    interval: Option<&str>,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    path: P,
) -> Result<Vec<T>> {
    util::check_directory_exists(&path)?;

    let config = Config::new::<T, P>(symbol, from, to, path, interval)?;
    load_records_from_csv(&config)
}

fn load_records_from_csv<T: BinanceData>(config: &Config) -> Result<Vec<T>> {
    let files = download::download_files(config)?;
    let mut result = Vec::new();
    for file in &files {
        let data = process_file::<T>(file, config)?;
        result.extend(data);
    }
    Ok(result)
}

fn process_file<T: BinanceData>(file: &Path, config: &Config) -> Result<Vec<T>> {
    let mut result = Vec::new();
    let from_milli = config.from.timestamp_millis();
    let to_milli = config.to.timestamp_millis();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)?;
    let mut raw_record = csv::StringRecord::new();
    let headers = config.csv_headers();

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
    Ok(result)
}
