use crate::error::{Error::DirectoryNotFound, Result};
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{de, Deserializer};
use std::path::Path;

pub fn check_directory_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_ref = path.as_ref();
    if !path_ref.is_dir() {
        return Err(DirectoryNotFound(path_ref.to_string_lossy().to_string()));
    }
    Ok(())
}

pub fn get_dates_from_range(from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<DateTime<Utc>> {
    if from > to {
        return Vec::new();
    }
    let mut current = utc_datetime(from);
    let end_start_of_day = utc_datetime(to);

    let mut dates = vec![current];

    while current < to {
        dates.push(current);
        current += Duration::days(1);
    }

    if to == end_start_of_day {
        dates.push(end_start_of_day);
    }
    dates
}

fn utc_datetime(date: DateTime<Utc>) -> DateTime<Utc> {
    Utc.from_utc_datetime(&date.date_naive().and_hms_opt(0, 0, 0).unwrap())
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "True" | "true" => Ok(true),
        "False" | "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["True", "true", "False", "false"])),
    }
}
