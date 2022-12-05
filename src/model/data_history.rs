use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub trait DataHistory: DeserializeOwned {
    fn time(&self) -> i64;
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DataPrices {
    pub time: i64,
    pub price: Decimal,
}

impl DataHistory for DataPrices {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DataShort {
    pub time: i64,
    pub price: Decimal,
    #[serde(rename(deserialize = "quote_qty"))]
    pub volume: Decimal,
}

impl DataHistory for DataShort {
    fn time(&self) -> i64 {
        self.time
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DataFull {
    pub id: u64,
    pub time: i64,
    pub price: Decimal,
    #[serde(rename(deserialize = "qty"))]
    pub size: Decimal,
    #[serde(rename(deserialize = "quote_qty"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "is_buyer_maker"))]
    pub buyer_maker: bool,
}

impl DataHistory for DataFull {
    fn time(&self) -> i64 {
        self.time
    }
}