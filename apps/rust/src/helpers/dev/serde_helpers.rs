//! Custom serde helpers and utilities.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize Option<T> as empty string when None.
pub mod option_empty_string {
    use super::*;

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match value {
            Some(v) => v.serialize(serializer),
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let value: Option<T> = Option::deserialize(deserializer)?;
        Ok(value)
    }
}

/// Deserialize string to number.
pub mod string_to_number {
    use super::*;
    use std::str::FromStr;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + Deserialize<'de>,
        T::Err: std::fmt::Display,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        s.parse::<T>().map_err(D::Error::custom)
    }
}

/// Deserialize string or number to i64.
pub mod flexible_i64 {
    use super::*;
    use serde_json::Value;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Number(n) => n.as_i64().ok_or_else(|| D::Error::custom("invalid number")),
            Value::String(s) => s.parse().map_err(D::Error::custom),
            _ => Err(D::Error::custom("expected number or string")),
        }
    }
}

/// Deserialize empty string as None.
pub mod empty_string_as_none {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) if s.is_empty() => Ok(None),
            Some(s) => Ok(Some(s)),
            None => Ok(None),
        }
    }
}

/// Serialize bool as "true"/"false" string.
pub mod bool_as_string {
    use super::*;

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if *value { "true" } else { "false" })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(true),
            "false" | "0" | "no" => Ok(false),
            _ => Err(D::Error::custom("expected boolean string")),
        }
    }
}

/// Serialize DateTime as ISO string.
pub mod datetime_iso {
    use super::*;
    use chrono::{DateTime, Utc};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(D::Error::custom)
    }
}

/// Deserialize comma-separated string to Vec.
pub mod comma_separated {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(s.split(',').map(|s| s.trim().to_string()).collect())
        }
    }

    pub fn serialize<S>(values: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&values.join(","))
    }
}

/// Default to empty vec if null.
pub fn default_empty_vec<T>() -> Vec<T> {
    Vec::new()
}

/// Default to empty string.
pub fn default_empty_string() -> String {
    String::new()
}

/// Default to false.
pub fn default_false() -> bool {
    false
}

/// Default to true.
pub fn default_true() -> bool {
    true
}

/// Default to zero.
pub fn default_zero() -> i64 {
    0
}
