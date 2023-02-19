use serde::{de, Deserializer};

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
