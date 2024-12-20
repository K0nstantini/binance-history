use super::{DataType, MarketType};
use crate::util::deserialize_bool;
use chrono::{
    serde::ts_milliseconds,
    DateTime,
    Utc,
};
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize};

pub trait BinanceData: DeserializeOwned {
    fn types() -> (MarketType, DataType);
    fn time(&self) -> i64;
}

// ========== SPOT ==============================

#[derive(Clone, Debug, Deserialize)]
pub struct SpotTrade {
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

#[derive(Clone, Debug, Deserialize)]
pub struct SpotAggTrade {
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

#[derive(Clone, Debug, Deserialize)]
pub struct SpotKline {
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

// ========== USDM ==============================

#[derive(Clone, Debug, Deserialize)]
pub struct USDMTrade {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub volume: Decimal,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct USDMAggTrade {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub first_trade_id: u64,
    pub last_trade_id: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct USDMKline {
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

// ========== COINM ==============================

#[derive(Clone, Debug, Deserialize)]
pub struct COINMTrade {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub base_size: Decimal,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct COINMAggTrade {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub price: Decimal,
    pub size: Decimal,
    pub first_trade_id: u64,
    pub last_trade_id: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub buyer_maker: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct COINMKline {
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

macro_rules! impl_binance_data {
    ($struct:ty, $market:expr, $data_type:expr, $time_field:ident) => {
        impl BinanceData for $struct {
            fn types() -> (MarketType, DataType) {
                ($market, $data_type)
            }
            fn time(&self) -> i64 {
                self.$time_field.timestamp_millis()
            }
        }
    };
}

impl_binance_data!(SpotTrade, MarketType::SPOT, DataType::Trades, time);
impl_binance_data!(SpotAggTrade, MarketType::SPOT, DataType::AggTrades, time);
impl_binance_data!(SpotKline, MarketType::SPOT, DataType::Klines, open_time);

impl_binance_data!(USDMTrade, MarketType::USDM, DataType::Trades, time);
impl_binance_data!(USDMAggTrade, MarketType::USDM, DataType::AggTrades, time);
impl_binance_data!(USDMKline, MarketType::USDM, DataType::Klines, open_time);

impl_binance_data!(COINMTrade, MarketType::COINM, DataType::Trades, time);
impl_binance_data!(COINMAggTrade, MarketType::COINM, DataType::AggTrades, time);
impl_binance_data!(COINMKline, MarketType::COINM, DataType::Klines, open_time);
