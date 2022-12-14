use crate::{Config, USDMTrades, DataInterval, PricesTrades, SpotTrades, DataType, MarketType};
use crate::model::HandyTrades;

// ====================================== TRADES =======================================================

#[tokio::test]
// #[ignore]
async fn get_from_csv_future_m_trades_test() {
    use crate::csv;

    let config = Config {
        market_type: MarketType::USDM,
        interval: DataInterval::Daily,
        data_type: DataType::Trades,
        path: "csv/usdm/".to_string(),
    };

    let data_no_headers: Vec<PricesTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2022-08-01 00:00:00",
        "2022-08-01 23:59:59",
    ).await.unwrap();
    println!("Rows: {}", data_no_headers.len());
    assert!(!data_no_headers.is_empty());

    let data_with_headers: Vec<USDMTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2022-11-19 00:00:00",
        "2022-11-20 23:59:59",
    ).await.unwrap();
    println!("Rows: {}", data_with_headers.len());
    assert!(!data_with_headers.is_empty());

    let data_handy: Vec<HandyTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2021-11-20 00:00:00",
        "2021-11-20 12:59:59",
    ).await.unwrap();
    println!("Rows: {}", data_handy.len());
    println!("First row: {:?}", data_handy.first().unwrap());
    assert!(!data_handy.is_empty());
}

#[tokio::test]
#[ignore]
async fn get_from_csv_future_c_trades_test() {
    use crate::csv;

    let config = Config {
        market_type: MarketType::COINM,
        interval: DataInterval::Daily,
        data_type: DataType::Trades,
        path: "csv/usdm/".to_string(),
    };

    let data_no_headers: Vec<PricesTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2022-08-01 00:00:00",
        "2022-08-01 23:59:59",
    ).await.unwrap();
    println!("Rows: {}", data_no_headers.len());
    assert!(!data_no_headers.is_empty());

    let data_with_headers: Vec<USDMTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2022-11-19 00:00:00",
        "2022-11-20 23:59:59",
    ).await.unwrap();
    println!("Rows: {}", data_with_headers.len());
    assert!(!data_with_headers.is_empty());

    let data_handy: Vec<HandyTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2021-11-20 00:00:00",
        "2021-11-20 12:59:59",
    ).await.unwrap();
    println!("Rows: {}", data_handy.len());
    println!("First row: {:?}", data_handy.first().unwrap());
    assert!(!data_handy.is_empty());
}

#[tokio::test]
// #[ignore]
async fn get_from_csv_spot_trades_test() {
    use crate::csv;

    let config = Config {
        market_type: MarketType::SPOT,
        interval: DataInterval::Daily,
        data_type: DataType::Trades,
        path: "csv/spot/".to_string(),
    };

    let data: Vec<PricesTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2022-08-01 00:00:00",
        "2022-08-01 23:59:59",
    ).await.unwrap();
    println!("Rows: {}", data.len());
    println!("First row: {:?}", data.first().unwrap());
    assert!(!data.is_empty());

    let data: Vec<SpotTrades> = csv::get_from_csv(
        &config,
        "DOGEUSDT",
        "2021-08-01 00:00:00",
        "2021-08-02 23:59:59",
    ).await.unwrap();
    println!("Rows: {}", data.len());
    println!("First row: {:?}", data.first().unwrap());
    assert!(!data.is_empty());
}
