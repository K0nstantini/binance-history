use chrono::{Datelike, DateTime, Utc};

use crate::Config;

#[derive(Debug)]
pub struct FileData {
    pub csv: String,
    pub zip: String,
    pub url: String,
}

impl FileData {
    pub fn new(config: &Config, symbol: &str, date: DateTime<Utc>) -> Self {
        let (y, m, d) = (date.year(), date.month(), date.day());
        let name = format!("{}-trades-{}-{:02}-{:02}", symbol, y, m, d);
        let csv = format!("{}{}.csv", config.path, name);
        let zip = format!("{}{}.zip", config.path, name);
        let url = format!("{}/{}.zip", config.path(symbol), name);

        FileData { csv, zip, url }
    }
}
