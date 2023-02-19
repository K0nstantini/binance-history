use chrono::{
    DateTime,
    serde::ts_milliseconds,
    Utc,
};
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{DataHistory, DataType, InternalDataType, MarketType};
use crate::data_type::DataType;
use crate::deserialize_bool;

pub trait DataHistory: DeserializeOwned {
    fn types() -> (MarketType, DataType);
    fn time(&self) -> i64;
}

// ========== SPOT ==============================

#[derive(Clone, Debug, Deserialize)]
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
    fn types() -> (MarketType, DataType) {
        (MarketType::SPOT, DataType::Trades)
    }
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotAggTrades {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub first_trade_id: u64,
    pub last_trade_id: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
    pub best_match: Decimal,
}

impl DataHistory for SpotAggTrades {
    fn types() -> (MarketType, DataType) {
        (MarketType::SPOT, DataType::AggTrades)
    }
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotKlines {
    #[serde(with = "ts_milliseconds")]
    pub open_time: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub close_time: DateTime<Utc>,
    pub quote_asset_volume: Decimal,
    pub trades_count: u64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
}

impl DataHistory for SpotKlines {
    fn types() -> (MarketType, DataType) {
        (MarketType::SPOT, DataType::Kines)
    }
    fn time(&self) -> i64 {
        self.open_time.timestamp_millis()
    }
}

// ========== USDM ==============================

#[derive(Clone, Debug, Deserialize)]
pub struct USDMTrades {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub volume: Decimal,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

impl DataHistory for USDMTrades {
    fn types() -> (MarketType, DataType) {
        (MarketType::USDM, DataType::Trades)
    }
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct USDMAggTrades {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub first_trade_id: u64,
    pub last_trade_id: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

impl DataHistory for USDMAggTrades {
    fn types() -> (MarketType, DataType) {
        (MarketType::USDM, DataType::AggTrades)
    }
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct USDMKlines {
    #[serde(with = "ts_milliseconds")]
    pub open_time: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub close_time: DateTime<Utc>,
    pub quote_asset_volume: Decimal,
    pub trades_count: u64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
}

impl DataHistory for USDMKlines {
    fn types() -> (MarketType, DataType) {
        (MarketType::USDM, DataType::Kines)
    }
    fn time(&self) -> i64 {
        self.open_time.timestamp_millis()
    }
}

// ========== COINM ==============================

#[derive(Clone, Debug, Deserialize)]
pub struct COINMTrades {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub base_size: Decimal,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

impl DataHistory for COINMTrades {
    fn types() -> (MarketType, DataType) {
        (MarketType::COINM, DataType::Trades)
    }
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct COINMAggTrades {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub first_trade_id: u64,
    pub last_trade_id: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

impl DataHistory for COINMAggTrades {
    fn types() -> (MarketType, DataType) {
        (MarketType::COINM, DataType::AggTrades)
    }
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct COINMKlines {
    #[serde(with = "ts_milliseconds")]
    pub open_time: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub close_time: DateTime<Utc>,
    pub base_asset_volume: Decimal,
    pub trades_count: u64,
    pub taker_buy_volume: Decimal,
    pub taker_buy_base_asset_volume: Decimal,
}

impl DataHistory for COINMKlines {
    fn types() -> (MarketType, DataType) {
        (MarketType::COINM, DataType::Kines)
    }
    fn time(&self) -> i64 {
        self.open_time.timestamp_millis()
    }
}
