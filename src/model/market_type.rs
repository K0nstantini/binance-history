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
