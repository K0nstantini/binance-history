use binance_history as bh;
use binance_history::{BinanceData, DataType, MarketType};
use serde::Deserialize;

// Custom structure for representing price data
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AggregatedPrice {
    time: i64,
    price: f64,
}

impl BinanceData for AggregatedPrice {
    fn types() -> (MarketType, DataType) {
        (MarketType::USDM, DataType::AggTrades)
    }

    fn time(&self) -> i64 {
        self.time
    }
}

#[tokio::main]
async fn main() -> bh::Result<()> {
    // Path for saving CSV files
    let path = "csv";

    // Time range for fetching data
    let from = "2024-12-01 00:00:00Z".parse().expect("Invalid start date format");
    let to = "2024-12-01 23:59:59Z".parse().expect("Invalid end date format");

    // Fetching aggregated trades for USD-M futures
    let agg_trades: Vec<AggregatedPrice> = bh::get_trades("ETHUSDT", from, to, path).await?;
    println!("First aggregated trade: {:?}", agg_trades.first());

    Ok(())
}