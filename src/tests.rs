use std::fmt::Debug;

use crate::csv;
use crate::model::data::{BinanceData, COINMAggTrades, COINMKlines, COINMTrades, SpotAggTrades, SpotKlines, SpotTrades, USDMAggTrades, USDMKlines, USDMTrades};

fn symbols() -> Vec<&'static str> {
    vec![
        "ADAUSDT",
        "DOGEUSDT",
        "NEARUSDT",
    ]
}


fn coinm_symbols() -> Vec<&'static str> {
    vec![
        "BTCUSD_PERP",
        "ETHUSD_PERP",
        "XRPUSD_PERP",
    ]
}

fn klines() -> Vec<&'static str> {
    vec!["1m", "4h", "1d"]
}

fn dates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("2022-03-01 00:00:00", "2022-03-02 23:59:59"),
        ("2022-10-11 00:00:00", "2022-10-13 18:00:00"),
        ("2023-01-01 00:00:00", "2023-01-05 23:59:59"),
    ]
}

async fn trades_test<T: BinanceData + Debug>(symbols: &[&str], path: &str) {
    for symbol in symbols {
        for (from, to) in dates() {
            let result: Vec<T> = csv::get(symbol, None, from, to, path)
                .await
                .unwrap();

            assert!(!result.is_empty())
        }
    }
}

async fn klines_test<T: BinanceData + Debug>(symbols: &[&str], path: &str) {
    for symbol in symbols {
        for (from, to) in dates() {
            for kline in klines() {
                let result: Vec<T> = csv::get(symbol, Some(kline), from, to, path)
                    .await
                    .unwrap();

                assert!(!result.is_empty())
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn spot_trades_test() {
    trades_test::<SpotTrades>(&symbols(), "csv/spot/trades/").await;
}

#[tokio::test]
#[ignore]
async fn spot_agg_trades_test() {
    trades_test::<SpotAggTrades>(&symbols(), "csv/spot/agg_trades/").await;
}

#[tokio::test]
#[ignore]
async fn spot_kline_test() {
    klines_test::<SpotKlines>(&symbols(), "csv/spot/klines/").await;
}

#[tokio::test]
#[ignore]
async fn usdm_trades_test() {
    trades_test::<USDMTrades>(&symbols(), "csv/usdm/trades/").await;
}

#[tokio::test]
#[ignore]
async fn usdm_agg_trades_test() {
    trades_test::<USDMAggTrades>(&symbols(), "csv/usdm/agg_trades/").await;
}

#[tokio::test]
#[ignore]
async fn usdm_kline_test() {
    klines_test::<USDMKlines>(&symbols(), "csv/usdm/klines/").await;
}

#[tokio::test]
async fn coinm_trades_test() {
    trades_test::<COINMTrades>(&coinm_symbols(), "csv/coinm/trades/").await;
}

#[tokio::test]
async fn coinm_agg_trades_test() {
    trades_test::<COINMAggTrades>(&coinm_symbols(), "csv/coinm/agg_trades/").await;
}

#[tokio::test]
async fn coinm_kline_test() {
    klines_test::<COINMKlines>(&coinm_symbols(), "csv/coinm/klines/").await;
}
