use chrono::{Datelike, DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;


const DOWNLOAD_ADDR: &str = "https://data.binance.vision/data/futures/um/daily/trades/";

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct RawDataHistory {
    pub id: u64,
    pub time: i64,
    pub price: Decimal,
    #[serde(rename(deserialize = "qty"))]
    pub size: Decimal,
    #[serde(rename(deserialize = "quote_qty"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "is_buyer_maker"))]
    pub buyer_maker: bool,
}


#[derive(Clone, Debug)]
pub struct FileData {
    pub name: String,
    pub path: String,
    pub csv: String,
    pub zip: String,
    pub url: String,
}

impl FileData {
    pub fn new(symbol: &str, path: &str, date: DateTime<Utc>) -> Self {
        let (y, m, d) = (date.year(), date.month(), date.day());
        let name = format!("{}-trades-{}-{:02}-{:02}", symbol, y, m, d);
        let path = path.to_string();
        let csv = format!("{}{}.csv", path, name);
        let zip = format!("{}{}.zip", path, name);
        let url = format!("{}{}/{}.zip", DOWNLOAD_ADDR, symbol, name);

        FileData { name, path, csv, zip, url }
    }
}
