use chrono::{DateTime, Utc};

use crate::model::Config;

#[derive(Debug)]
pub struct FileData {
    pub csv: String,
    pub zip: String,
    pub url: String,
}

impl FileData {
    pub fn new(config: &Config, symbol: &str, date: DateTime<Utc>) -> Self {
        let name = config.data_type.file_name(symbol, date);
        let csv = format!("{}{}.csv", config.path, name);
        let zip = format!("{}{}.zip", config.path, name);
        let url = format!("{}/{}.zip", config.path(symbol), name);
        dbg!(&url);

        FileData { csv, zip, url }
    }
}
