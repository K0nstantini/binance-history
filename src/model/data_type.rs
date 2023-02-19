use chrono::{Datelike, DateTime, Utc};

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub enum DataType {
    AggTrades,
    Kines,
    Trades,
}

impl DataType {
    pub fn into_internal(self, interval: Option<&str>) -> Result<InternalDataType> {
        let result = match self {
            DataType::AggTrades => InternalDataType::AggTrades,
            DataType::Kines =>
                match interval {
                    Some(i) => InternalDataType::Kines(i.to_string()),
                    None => return Err(Error::MissedKlinesInterval)
                }
            DataType::Trades => InternalDataType::Trades
        };
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub enum InternalDataType {
    AggTrades,
    Kines(String),
    Trades,
}

impl InternalDataType {
    pub(crate) fn path(&self, symbol: &str) -> String {
        match self {
            InternalDataType::AggTrades => format!("aggTrades/{symbol}"),
            InternalDataType::Kines(i) => format!("klines/{symbol}/{i}"),
            InternalDataType::Trades => format!("trades/{symbol}"),
        }
    }

    pub(crate) fn file_name(&self, symbol: &str, date: DateTime<Utc>) -> String {
        let (y, m, d) = (date.year(), date.month(), date.day());
        match self {
            InternalDataType::AggTrades => format!("{symbol}-aggTrades-{y}-{m:02}-{d:02}"),
            InternalDataType::Kines(i) => format!("{symbol}-{i}-{y}-{m:02}-{d:02}"),
            InternalDataType::Trades => format!("{symbol}-trades-{y}-{m:02}-{d:02}"),
        }
    }
}
