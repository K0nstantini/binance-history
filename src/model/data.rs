use chrono::{
    DateTime,
    serde::ts_milliseconds,
    Utc,
};
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize};

use crate::model::data_type::DataType;
use crate::model::MarketType;
use crate::util::deserialize::deserialize_bool;

pub trait BinanceData: DeserializeOwned {
    fn types() -> (MarketType, DataType);
    fn time(&self) -> i64;
}

// ========== SPOT ==============================

#[derive(Clone, Debug, Deserialize)]
pub struct SpotTrades {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub volume: Decimal,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub best_match: bool,
}

impl BinanceData for SpotTrades {
    fn types() -> (MarketType, DataType) {
        (MarketType::SPOT, DataType::Trades)
    }
    fn time(&self) -> i64 {
        self.time.timestamp_millis()
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
    #[serde(deserialize_with = "deserialize_bool")]
    pub best_match: bool,
}

impl BinanceData for SpotAggTrades {
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

impl BinanceData for SpotKlines {
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

impl BinanceData for USDMTrades {
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

impl BinanceData for USDMAggTrades {
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

impl BinanceData for USDMKlines {
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

impl BinanceData for COINMTrades {
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

impl BinanceData for COINMAggTrades {
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

impl BinanceData for COINMKlines {
    fn types() -> (MarketType, DataType) {
        (MarketType::COINM, DataType::Kines)
    }
    fn time(&self) -> i64 {
        self.open_time.timestamp_millis()
    }
}
