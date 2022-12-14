use crate::model::market_type::klines::KlineInterval;

mod trades;
mod agg_trades;
mod klines;
mod common;

pub use common::*;
pub use trades::*;
pub use agg_trades::*;
pub use klines::*;

#[derive(Copy, Clone, Debug)]
pub enum MarketType { USDM, COINM, SPOT }

impl MarketType {
    pub(crate) fn path(&self) -> &'static str {
        match self {
            MarketType::USDM => "futures/um",
            MarketType::COINM => "futures/cm",
            MarketType::SPOT => "spot"
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DataType {
    AggTrades,
    Kines(KlineInterval),
    Trades,
}

impl DataType {
    pub(crate) fn path(&self, symbol: &str) -> String {
        match self {
            DataType::AggTrades => format!("aggTrades/{}", symbol),
            DataType::Kines(i) => format!("klines/{}/{}", symbol, i.path_in_url()),
            DataType::Trades => format!("trades/{}", symbol),
        }
    }
}


