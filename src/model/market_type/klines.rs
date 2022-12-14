use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
pub enum KlineInterval {
    Second1,
    Minute1,
    Minute3,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour2,
    Hour4,
    Hour6,
    Hour8,
    Hour12,
    Day1,
    Day3,
    Week1,
    Month1,
}

impl KlineInterval {
    pub(crate) fn path_in_url(&self) -> &'static str {
        match self {
            KlineInterval::Second1 => "1s",
            KlineInterval::Minute1 => "1m",
            KlineInterval::Minute3 => "3m",
            KlineInterval::Minute5 => "5m",
            KlineInterval::Minute15 => "15m",
            KlineInterval::Minute30 => "30m",
            KlineInterval::Hour1 => "1h",
            KlineInterval::Hour2 => "2h",
            KlineInterval::Hour4 => "4h",
            KlineInterval::Hour6 => "6h",
            KlineInterval::Hour8 => "8h",
            KlineInterval::Hour12 => "12h",
            KlineInterval::Day1 => "1d",
            KlineInterval::Day3 => "3d",
            KlineInterval::Week1 => "1w",
            KlineInterval::Month1 => "1mo"
        }
    }
}
