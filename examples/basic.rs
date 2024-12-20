use binance_history as bh;
use binance_history::{COINMKline, SpotTrade, USDMAggTrade};

fn main() -> bh::Result<()> {
    // Path for saving CSV files
    let path = "csv";

    // Time range for fetching data
    let from = "2024-12-01 00:00:00Z".parse().expect("Invalid start date format");
    let to = "2024-12-01 23:59:59Z".parse().expect("Invalid end date format");

    // Fetching spot trades
    let trades: Vec<SpotTrade> = bh::get_trades("BTCUSDT", from, to, path)?;
    println!("First spot trade: {:?}", trades.first());

    // Fetching aggregated trades for USD-M futures
    let agg_trades: Vec<USDMAggTrade> = bh::get_trades("BTCUSDT", from, to, path)?;
    println!("First aggregated trade: {:?}", agg_trades.first());

    // Fetching candlesticks (klines) for COIN-M futures
    let klines: Vec<COINMKline> = bh::get_klines("BTCUSD_PERP", "1h", from, to, path)?;
    println!("First kline: {:?}", klines.first());

    Ok(())
}
