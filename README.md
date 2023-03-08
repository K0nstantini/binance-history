# Binance history

Downloading and parsing historical data from the Binance exchange

- [Description](https://github.com/binance/binance-public-data)
- [Binance public data](https://data.binance.vision/)

## Installation

Add the following line to your `Cargo.toml` file:

```
binance-history = "0.1.0"
```

Or for the latest github version:

```
binance-history = { git = "https://github.com/K0nstantini/binance-history" }
```

## Usage

```rust
use binance_history::{COINMKlines, SpotTrades, USDMAggTrades};

#[tokio::main]
async fn main() {
    let symbol = "BTCUSDT";
    let path = "csv";
    let from = "2023-02-01 00:00:00";
    let to = "2023-02-02 23:59:59";

    let spot_trades: Vec<SpotTrades> = binance_history::get(symbol, None, from, to, path).await.unwrap();
    let usdm_agg_trades: Vec<USDMAggTrades> = binance_history::get(symbol, None, from, to, path).await.unwrap();
    let coinm_klines: Vec<COINMKlines> = binance_history::get("BTCUSD_PERP", Some("1h"), from, to, path).await.unwrap();

    assert!(!spot_trades.is_empty());
    assert!(!usdm_agg_trades.is_empty());
    assert!(!coinm_klines.is_empty());
}
```

### Using custom struct

```rust
use binance_history::{BinanceData, DataType, MarketType};
use serde::Deserialize;

#[derive(Deserialize)]
struct Prices {
    time: i64,
    price: f64,
}

impl BinanceData for Prices {
    fn types() -> (MarketType, DataType) {
        (MarketType::USDM, DataType::Trades)
    }

    fn time(&self) -> i64 {
        self.time
    }
}

#[tokio::main]
async fn main() {
    let symbol = "ETHUSDT";
    let path = "csv";
    let from = "2023-02-01 00:00:00";
    let to = "2023-02-02 23:59:59";

    let prices: Vec<Prices> = binance_history::get(symbol, None, from, to, path).await.unwrap();
    assert!(!prices.is_empty());
}
```