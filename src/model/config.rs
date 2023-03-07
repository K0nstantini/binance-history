use crate::{BinanceData, util};
use crate::error::Result;
use crate::model::{DataInterval, MarketType};
use crate::model::data_type::InternalDataType;

const URL: &str = "https://data.binance.vision/data";

#[derive(Clone, Debug)]
pub struct Config {
    pub market_type: MarketType,
    pub interval: DataInterval,
    pub data_type: InternalDataType,
    pub path: String,
}

impl Config {
    pub fn new<T: BinanceData>(path: &str, interval: Option<&str>) -> Result<Self> {
        let path = match path.ends_with('/') {
            true => path.to_string(),
            false => format!("{path}/")
        };
        util::check_path(&path)?;

        let config = Config {
            market_type: T::types().0,
            interval: DataInterval::Daily,
            data_type: T::types().1.into_internal(interval)?,
            path,
        };
        Ok(config)
    }

    pub fn path(&self, symbol: &str) -> String {
        format!(
            "{}/{}/{}/{}",
            URL,
            self.market_type.path(),
            self.interval.path(),
            self.data_type.path(symbol),
        )
    }

    pub fn headers(&self) -> csv::StringRecord {
        let headers = match self.market_type {
            MarketType::SPOT => match self.data_type {
                InternalDataType::AggTrades => vec![
                    "id",
                    "price",
                    "size",
                    "first_trade_id",
                    "last_trade_id",
                    "time",
                    "buyer_maker",
                    "best_match",
                ],
                InternalDataType::Kines(_) => vec![
                    "open_time",
                    "open",
                    "high",
                    "low",
                    "close",
                    "volume",
                    "close_time",
                    "quote_asset_volume",
                    "trades_count",
                    "taker_buy_base_asset_volume",
                    "taker_buy_quote_asset_volume",
                    "ignore",
                ],
                InternalDataType::Trades => vec![
                    "id",
                    "price",
                    "size",
                    "volume",
                    "time",
                    "buyer_maker",
                    "best_match",
                ]
            }
            MarketType::USDM => match self.data_type {
                InternalDataType::AggTrades => vec![
                    "id",
                    "price",
                    "size",
                    "first_trade_id",
                    "last_trade_id",
                    "time",
                    "buyer_maker",
                ],
                InternalDataType::Kines(_) => vec![
                    "open_time",
                    "open",
                    "high",
                    "low",
                    "close",
                    "volume",
                    "close_time",
                    "quote_asset_volume",
                    "trades_count",
                    "taker_buy_base_asset_volume",
                    "taker_buy_quote_asset_volume",
                    "ignore",
                ],
                InternalDataType::Trades => vec![
                    "id",
                    "price",
                    "size",
                    "volume",
                    "time",
                    "buyer_maker",
                ]
            },
            MarketType::COINM => match self.data_type {
                InternalDataType::AggTrades => vec![
                    "id",
                    "price",
                    "size",
                    "first_trade_id",
                    "last_trade_id",
                    "time",
                    "buyer_maker",
                ],
                InternalDataType::Kines(_) => vec![
                    "open_time",
                    "open",
                    "high",
                    "low",
                    "close",
                    "volume",
                    "close_time",
                    "base_asset_volume",
                    "trades_count",
                    "taker_buy_volume",
                    "taker_buy_base_asset_volume",
                    "ignore",
                ],
                InternalDataType::Trades => vec![
                    "id",
                    "price",
                    "size",
                    "base_size",
                    "time",
                    "buyer_maker",
                ]
            },
        };
        csv::StringRecord::from(headers)
    }
}