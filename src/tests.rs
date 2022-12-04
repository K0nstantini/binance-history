#[tokio::test]
async fn get_from_csv_test() {
    use crate::csv;

    let data = csv::get_from_csv(
        "DOGEUSDT",
        "csv/",
        "2022-11-10 13:25:00",
        "2022-11-11 16:25:00",
    ).await.unwrap();
    println!("Got rows: {}", data.len());
    assert!(!data.is_empty());
}
