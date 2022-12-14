use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::{DataHistory, Side};
use serde::Deserialize;
use crate::deserialize_bool;
use chrono::serde::ts_milliseconds::deserialize as from_ms;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct USDMTrades {
    pub time: i64,
    pub price: Decimal,
    pub size: Decimal,
    pub volume: Decimal,
    pub buyer_maker: bool,
}

impl DataHistory for USDMTrades {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct COINMTrades {
    pub time: i64,
    pub price: Decimal,
    pub size: Decimal,
    pub base_size: Decimal,
    pub buyer_maker: bool,
}

impl DataHistory for COINMTrades {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct SpotTrades {
    pub time: i64,
    pub price: Decimal,
    pub size: Decimal,
    pub volume: Decimal,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub best_match: bool,
}

impl DataHistory for SpotTrades {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HandyTrades {
    #[serde(deserialize_with = "from_ms")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub volume: Decimal,
    #[serde(rename(deserialize = "buyer_maker"))]
    pub side: Side,
}

impl DataHistory for HandyTrades {
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}
