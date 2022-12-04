#[tokio::test]
async fn get_from_csv_test() {
    use crate::csv;

    let (date_start, date_end) = ("2022-08-01 00:00:00", "2022-08-01 23:59:59");
    let data_no_headers = csv::get_from_csv(
        "DOGEUSDT",
        "csv/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Got rows: {}", data_no_headers.len());

    let (date_start, date_end) = ("2022-11-19 00:00:00", "2022-11-20 23:59:59");
    let data_with_headers = csv::get_from_csv(
        "DOGEUSDT",
        "csv/",
        date_start,
        date_end,
    ).await.unwrap();
    println!("Got rows: {}", data_with_headers.len());

    assert!(!data_with_headers.is_empty());
}
