const USD_M_ADDR: &str = "https://data.binance.vision/data/futures/um/daily/trades/";
const COIN_M_ADDR: &str = "https://data.binance.vision/data/futures/cm/daily/trades/";
const SPOT_ADDR: &str = "https://data.binance.vision/data/spot/daily/trades/";

#[derive(Copy, Clone, Debug)]
pub enum MarketType { USDM, COINM, SPOT }

impl MarketType {
    pub(crate) fn url(&self) -> &'static str {
        match self {
            MarketType::USDM => USD_M_ADDR,
            MarketType::COINM => COIN_M_ADDR,
            MarketType::SPOT => SPOT_ADDR
        }
    }
    pub(crate) fn headers(&self) -> csv::StringRecord {
        let headers = match self {
            MarketType::USDM => vec!["id", "price", "qty", "quote_qty", "time", "is_buyer_maker"],
            MarketType::COINM => todo!(),
            MarketType::SPOT => vec!["id", "price", "qty", "quote_qty", "time", "is_buyer_maker", "is_best_match"],
        };
        csv::StringRecord::from(headers)
    }
}
