use crate::csv;
use crate::model::data::*;
use chrono::{DateTime, Utc};
use std::fmt::Debug;

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

fn dates() -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
    let date = |s: &str| s.parse::<DateTime<Utc>>().unwrap();
    vec![
        (date("2022-03-01T00:00:00Z"), date("2022-03-02T00:00:00Z")),
        (date("2022-10-11T00:00:00Z"), date("2022-10-13T00:00:00Z")),
        (date("2023-01-01T00:00:00Z"), date("2023-01-05T00:00:00Z")),
        (date("2024-12-01T00:00:00Z"), date("2024-12-04T00:00:00Z")),
    ]
}

async fn trades_test<T: BinanceData + Debug>(symbols: &[&str], path: &str) {
    for symbol in symbols {
        for (from, to) in dates() {
            let result: Vec<T> = csv::get_trades(symbol, from, to, path)
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
                let result: Vec<T> = csv::get_klines(symbol, kline, from, to, path)
                    .await
                    .unwrap();

                assert!(!result.is_empty())
            }
        }
    }
}

#[tokio::test]
async fn spot_trades_test() {
    trades_test::<SpotTrade>(&symbols(), "csv/spot/trades/").await;
}

#[tokio::test]
async fn spot_agg_trades_test() {
    trades_test::<SpotAggTrade>(&symbols(), "csv/spot/agg_trades/").await;
}

#[tokio::test]
async fn spot_kline_test() {
    klines_test::<SpotKline>(&symbols(), "csv/spot/klines/").await;
}

#[tokio::test]
async fn usdm_trades_test() {
    trades_test::<USDMTrade>(&symbols(), "csv/usdm/trades/").await;
}

#[tokio::test]
async fn usdm_agg_trades_test() {
    trades_test::<USDMAggTrade>(&symbols(), "csv/usdm/agg_trades/").await;
}

#[tokio::test]
async fn usdm_kline_test() {
    klines_test::<USDMKline>(&symbols(), "csv/usdm/klines/").await;
}

#[tokio::test]
async fn coinm_trades_test() {
    trades_test::<COINMTrade>(&coinm_symbols(), "csv/coinm/trades/").await;
}

#[tokio::test]
async fn coinm_agg_trades_test() {
    trades_test::<COINMAggTrade>(&coinm_symbols(), "csv/coinm/agg_trades/").await;
}

#[tokio::test]
async fn coinm_kline_test() {
    klines_test::<COINMKline>(&coinm_symbols(), "csv/coinm/klines/").await;
}
