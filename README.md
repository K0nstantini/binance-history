# Binance history

[![Crates.io](https://img.shields.io/crates/v/binance-history.svg)](https://crates.io/crates/binance-history)
[![Documentation](https://docs.rs/hashbrown/badge.svg)](https://docs.rs/binance-history)

**A Rust library for downloading and parsing historical data from Binance, supporting spot trades, futures, and candlestick data.**

---

## Additional Resources

- [Documentation](https://github.com/binance/binance-public-data)
- [Binance public data](https://data.binance.vision/)

## Features
- Download spot trades, futures trades, and candlestick data (klines).
- Save data locally in CSV format.
- Define custom data structures for advanced use cases.

## Installation

Add the following line to your `Cargo.toml` file:

```
binance-history = "0.2.0"
```

## Usage

### Basic usage

```rust
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

```

### Using custom struct

You can define a custom structure to parse data specific to your application's requirements.

```rust
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

fn main() -> bh::Result<()> {
    // Path for saving CSV files
    let path = "csv";

    // Time range for fetching data
    let from = "2024-12-01 00:00:00Z".parse().expect("Invalid start date format");
    let to = "2024-12-01 23:59:59Z".parse().expect("Invalid end date format");

    // Fetching aggregated trades for USD-M futures
    let agg_trades: Vec<AggregatedPrice> = bh::get_trades("ETHUSDT", from, to, path)?;
    println!("First aggregated trade: {:?}", agg_trades.first());

    Ok(())
}
```

## License

This project is dual-licensed under MIT and Apache 2.0.
