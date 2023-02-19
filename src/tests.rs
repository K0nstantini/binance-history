use crate::{COINMTrades, Config, DataHistory, DataInterval, DataType, MarketType, PricesTrades, SpotTrades, USDMTrades};
use crate::data::{COINMTrades, DataHistory, SpotTrades, USDMTrades};
use crate::data_type::DataType;
use crate::error::Result;

#[allow(dead_code)]
async fn get_default<T: DataHistory>(config: &Config) -> Result<Vec<T>> {
    use crate::csv;

    csv::get_from_csv::<T>(
        &config,
        "DOGEUSDT",
        "2022-08-01 00:00:00",
        "2022-08-01 23:59:59",
    ).await
}

// ====================================== TRADES =======================================================

#[tokio::test]
async fn trades_usdm_daily_test() {
    use crate::csv;

    let config = Config {
        market_type: MarketType::USDM,
        interval: DataInterval::Daily,
        data_type: DataType::Trades,
        path: "csv/usdm/trades/".to_string(),
    };

    let data_no_headers: Vec<PricesTrades> = get_default(&config).await.unwrap();
    assert!(!data_no_headers.is_empty());

    let data_with_headers: Vec<USDMTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2022-11-19 00:00:00",
        "2022-11-20 23:59:59",
    ).await.unwrap();
    assert!(!data_with_headers.is_empty());
}

#[tokio::test]
async fn trades_coinm_daily_test() {
    let config = Config {
        market_type: MarketType::COINM,
        interval: DataInterval::Daily,
        data_type: DataType::Trades,
        path: "csv/coinm/trades/".to_string(),
    };

    let data: Vec<PricesTrades> = get_default(&config).await.unwrap();
    assert!(!data.is_empty());

    let data: Vec<COINMTrades> = get_default(&config).await.unwrap();
    assert!(!data.is_empty());
}

#[tokio::test]
async fn trades_spot_daily_test() {
    let config = Config {
        market_type: MarketType::SPOT,
        interval: DataInterval::Daily,
        data_type: DataType::Trades,
        path: "csv/spot/trades/".to_string(),
    };

    let data: Vec<PricesTrades> = get_default(&config).await.unwrap();
    assert!(!data.is_empty());

    let data: Vec<SpotTrades> = get_default(&config).await.unwrap();
    assert!(!data.is_empty());
}
