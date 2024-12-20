use super::{DataInterval, InternalDataType, MarketType};
use crate::error::Result;
use crate::BinanceData;
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};
use url::Url;

const BASE_URL: &str = "https://data.binance.vision/data/";

pub struct Config {
    pub symbol: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub market_type: MarketType,
    pub interval: DataInterval,
    pub data_type: InternalDataType,
    pub path: PathBuf,
}

impl Config {
    pub fn new<T: BinanceData, P: AsRef<Path>>(
        symbol: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        path: P,
        interval: Option<&str>,
    ) -> Result<Self> {
        let config = Config {
            symbol: symbol.to_string(),
            from,
            to,
            market_type: T::types().0,
            interval: DataInterval::Daily,
            data_type: T::types().1.into_internal(interval)?,
            path: path.as_ref().to_path_buf(),
        };
        Ok(config)
    }

    pub fn url(&self) -> Result<Url> {
        let base = Url::parse(BASE_URL)?;
        let url = base
            .join(self.market_type.path())?
            .join(self.interval.path())?
            .join(&self.data_type.path(&self.symbol))?;
        Ok(url)
    }
}

impl Config {
    pub fn csv_headers(&self) -> csv::StringRecord {
        match self.market_type {
            MarketType::SPOT => self.spot_headers(),
            MarketType::USDM => self.usdm_headers(),
            MarketType::COINM => self.coinm_headers(),
        }
    }

    fn spot_headers(&self) -> csv::StringRecord {
        match self.data_type {
            InternalDataType::AggTrades => csv::StringRecord::from(&[
                "id", "price", "size", "first_trade_id", "last_trade_id", "time", "buyer_maker", "best_match",
            ][..]),
            InternalDataType::Klines(_) => csv::StringRecord::from(&[
                "open_time", "open", "high", "low", "close", "volume", "close_time",
                "quote_asset_volume", "trades_count", "taker_buy_base_asset_volume",
                "taker_buy_quote_asset_volume", "ignore",
            ][..]),
            InternalDataType::Trades => csv::StringRecord::from(&[
                "id", "price", "size", "volume", "time", "buyer_maker", "best_match",
            ][..]),
        }
    }

    fn usdm_headers(&self) -> csv::StringRecord {
        match self.data_type {
            InternalDataType::AggTrades => csv::StringRecord::from(&[
                "id", "price", "size", "first_trade_id", "last_trade_id", "time", "buyer_maker",
            ][..]),
            InternalDataType::Klines(_) => csv::StringRecord::from(&[
                "open_time", "open", "high", "low", "close", "volume", "close_time",
                "quote_asset_volume", "trades_count", "taker_buy_base_asset_volume",
                "taker_buy_quote_asset_volume", "ignore",
            ][..]),
            InternalDataType::Trades => csv::StringRecord::from(&[
                "id", "price", "size", "volume", "time", "buyer_maker",
            ][..]),
        }
    }

    fn coinm_headers(&self) -> csv::StringRecord {
        match self.data_type {
            InternalDataType::AggTrades => csv::StringRecord::from(&[
                "id", "price", "size", "first_trade_id", "last_trade_id", "time", "buyer_maker",
            ][..]),
            InternalDataType::Klines(_) => csv::StringRecord::from(&[
                "open_time", "open", "high", "low", "close", "volume", "close_time",
                "base_asset_volume", "trades_count", "taker_buy_volume",
                "taker_buy_base_asset_volume", "ignore",
            ][..]),
            InternalDataType::Trades => csv::StringRecord::from(&[
                "id", "price", "size", "base_size", "time", "buyer_maker",
            ][..]),
        }
    }
}