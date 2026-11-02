//! Serde helpers for flexible type handling.

use serde::{Deserialize, Deserializer};

pub fn string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => Ok(s),
        StringOrInt::Int(i) => Ok(i.to_string()),
    }
}

/// Deserializes a value that may be a string, an integer, or absent.
///
/// Some Librus endpoints send `Weight` as `3` for one resource and `"3"` for
/// another, and omit it for categories that do not count towards an average.
pub fn string_or_int_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }

    match Option::<StringOrInt>::deserialize(deserializer)? {
        Some(StringOrInt::String(s)) => Ok(Some(s)),
        Some(StringOrInt::Int(i)) => Ok(Some(i.to_string())),
        None => Ok(None),
    }
}
