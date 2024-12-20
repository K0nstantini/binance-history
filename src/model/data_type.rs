use crate::error::{Error, Result};
use chrono::{DateTime, Datelike, Utc};

#[derive(Copy, Clone, Debug)]
pub enum DataType {
    AggTrades,
    Klines,
    Trades,
}

impl DataType {
    pub fn into_internal(self, interval: Option<&str>) -> Result<InternalDataType> {
        match self {
            DataType::AggTrades => Ok(InternalDataType::AggTrades),
            DataType::Klines => interval
                .map(|i| Ok(InternalDataType::Klines(i.to_string())))
                .unwrap_or(Err(Error::MissingKlinesInterval)),
            DataType::Trades => Ok(InternalDataType::Trades),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InternalDataType {
    AggTrades,
    Klines(String),
    Trades,
}

impl InternalDataType {
    pub fn path(&self, symbol: &str) -> String {
        match self {
            InternalDataType::AggTrades => format!("aggTrades/{symbol}/"),
            InternalDataType::Klines(i) => format!("klines/{symbol}/{i}/"),
            InternalDataType::Trades => format!("trades/{symbol}/"),
        }
    }

    pub fn file_name(&self, symbol: &str, date: DateTime<Utc>) -> String {
        format!("{symbol}-{data_type}-{year}-{month:02}-{day:02}",
                symbol = symbol,
                data_type = match self {
                    InternalDataType::AggTrades => "aggTrades",
                    InternalDataType::Klines(i) => i,
                    InternalDataType::Trades => "trades",
                },
                year = date.year(),
                month = date.month(),
                day = date.day(),
        )
    }
}
