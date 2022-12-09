use crate::{DataFull, DataPrices, DataSpot, MarketType};
use crate::model::DataHandy;

#[tokio::test]
#[ignore]
async fn get_from_csv_future_m_test() {
    use crate::csv;

    let (date_start, date_end) = ("2022-08-01 00:00:00", "2022-08-01 23:59:59");
    let data_no_headers: Vec<DataPrices> = csv::get_from_csv(
        MarketType::USDM,
        "DOGEUSDT",
        "csv/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Rows: {}", data_no_headers.len());
    assert!(!data_no_headers.is_empty());

    let (date_start, date_end) = ("2022-11-19 00:00:00", "2022-11-20 23:59:59");
    let data_with_headers: Vec<DataFull> = csv::get_from_csv(
        MarketType::USDM,
        "DOGEUSDT",
        "csv/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Rows: {}", data_with_headers.len());
    assert!(!data_with_headers.is_empty());

    let data_handy: Vec<DataHandy> = csv::get_from_csv(
        MarketType::USDM,
        "DOGEUSDT",
        "csv/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Rows: {}", data_handy.len());
    println!("First row: {:?}", data_handy.first().unwrap());
    assert!(!data_handy.is_empty());
}

#[tokio::test]
// #[ignore]
async fn get_from_csv_spot_test() {
    use crate::csv;

    let (date_start, date_end) = ("2022-08-01 00:00:00", "2022-08-01 23:59:59");
    let data: Vec<DataPrices> = csv::get_from_csv(
        MarketType::SPOT,
        "DOGEUSDT",
        "csv/spot/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Rows: {}", data.len());
    println!("First row: {:?}", data.first().unwrap());
    assert!(!data.is_empty());

    let data: Vec<DataSpot> = csv::get_from_csv(
        MarketType::SPOT,
        "DOGEUSDT",
        "csv/spot/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Rows: {}", data.len());
    println!("First row: {:?}", data.first().unwrap());
    assert!(!data.is_empty());
}
