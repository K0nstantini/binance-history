use chrono::{Datelike, DateTime, Utc};
use crate::MarketType;

#[derive(Debug)]
pub struct FileData {
    pub csv: String,
    pub zip: String,
    pub url: String,
}

impl FileData {
    pub fn new(market: MarketType, symbol: &str, path: &str, date: DateTime<Utc>) -> Self {
        let (y, m, d) = (date.year(), date.month(), date.day());
        let name = format!("{}-trades-{}-{:02}-{:02}", symbol, y, m, d);
        let csv = format!("{}{}.csv", path, name);
        let zip = format!("{}{}.zip", path, name);
        let url = format!("{}{}/{}.zip", market.url(), symbol, name);

        FileData { csv, zip, url }
    }
}
