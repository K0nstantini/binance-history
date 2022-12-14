use std::fmt;

use serde::{de, Deserialize, Deserializer};
use serde::de::Visitor;

use crate::Side;

impl<'de> Deserialize<'de> for Side {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_bool(SideVisitor)
    }
}

struct SideVisitor;

impl<'de> Visitor<'de> for SideVisitor {
    type Value = Side;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a bool'")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Self::Value, E> {
        let res = if value { Side::Sell } else { Side::Buy };
        Ok(res)
    }
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "True" | "true" => Ok(true),
        "False" | "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["True", "true", "False", "false"])),
    }
}
