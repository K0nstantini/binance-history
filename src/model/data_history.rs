use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds::deserialize as from_ms;
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::Side;

use super::deserialize::deserialize_bool;

pub trait DataHistory: DeserializeOwned {
    fn time(&self) -> i64;
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DataPrices {
    pub time: i64,
    pub price: Decimal,
}

impl DataHistory for DataPrices {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DataFull {
    pub time: i64,
    pub price: Decimal,
    #[serde(rename(deserialize = "qty"))]
    pub size: Decimal,
    #[serde(rename(deserialize = "quote_qty"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "is_buyer_maker"))]
    pub buyer_maker: bool,
}

impl DataHistory for DataFull {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DataHandy {
    #[serde(deserialize_with = "from_ms")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    #[serde(rename(deserialize = "qty"))]
    pub size: Decimal,
    #[serde(rename(deserialize = "quote_qty"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "is_buyer_maker"))]
    pub side: Side,
}

impl DataHistory for DataHandy {
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}

// ================================ SPOT ===============================

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DataSpot {
    pub time: i64,
    pub price: Decimal,
    #[serde(rename(deserialize = "qty"))]
    pub size: Decimal,
    #[serde(rename(deserialize = "quote_qty"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "is_buyer_maker"), deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
    #[serde(rename(deserialize = "is_best_match"), deserialize_with = "deserialize_bool")]
    pub best_match: bool,
}

impl DataHistory for DataSpot {
    fn time(&self) -> i64 {
        self.time
    }
}
