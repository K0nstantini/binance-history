const USD_M_ADDR: &str = "https://data.binance.vision/data/futures/um/daily/trades/";
const SPOT_ADDR: &str = "https://data.binance.vision/data/spot/daily/trades/";

#[derive(Copy, Clone, Debug)]
pub enum MarketType { USDM, SPOT }

impl MarketType {
    pub(crate) fn url(&self) -> &'static str {
        match self {
            MarketType::USDM => USD_M_ADDR,
            MarketType::SPOT => SPOT_ADDR
        }
    }
}
