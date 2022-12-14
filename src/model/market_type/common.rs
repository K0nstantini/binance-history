use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub trait DataHistory: DeserializeOwned {
    fn time(&self) -> i64;
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PricesTrades {
    pub time: i64,
    pub price: Decimal,
}

impl DataHistory for PricesTrades {
    fn time(&self) -> i64 {
        self.time
    }
}
