//! JSON utilities.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

/// Parse JSON string to type.
pub fn parse<T: DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serialize type to JSON string.
pub fn stringify<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serialize type to pretty JSON string.
pub fn stringify_pretty<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

/// Merge two JSON objects (second overwrites first).
pub fn merge(base: Value, overlay: Value) -> Value {
    match (base, overlay) {
        (Value::Object(mut base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                base_map.insert(key, value);
            }
            Value::Object(base_map)
        }
        (_, overlay) => overlay,
    }
}

/// Deep merge two JSON objects.
pub fn deep_merge(base: Value, overlay: Value) -> Value {
    match (base, overlay) {
        (Value::Object(mut base_map), Value::Object(overlay_map)) => {
            for (key, overlay_value) in overlay_map {
                let base_value = base_map.remove(&key).unwrap_or(Value::Null);
                base_map.insert(key, deep_merge(base_value, overlay_value));
            }
            Value::Object(base_map)
        }
        (_, overlay) => overlay,
    }
}

/// Extract a value at a path (e.g., "data.items.0.name").
pub fn get_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for key in path.split('.') {
        current = match current {
            Value::Object(map) => map.get(key)?,
            Value::Array(arr) => {
                let idx: usize = key.parse().ok()?;
                arr.get(idx)?
            }
            _ => return None,
        };
    }
    Some(current)
}

/// Extract string at path.
pub fn get_str<'a>(value: &'a Value, path: &str) -> Option<&'a str> {
    get_path(value, path).and_then(|v| v.as_str())
}

/// Extract i64 at path.
pub fn get_i64(value: &Value, path: &str) -> Option<i64> {
    get_path(value, path).and_then(|v| v.as_i64())
}

/// Extract bool at path.
pub fn get_bool(value: &Value, path: &str) -> Option<bool> {
    get_path(value, path).and_then(|v| v.as_bool())
}

/// Extract array at path.
pub fn get_array<'a>(value: &'a Value, path: &str) -> Option<&'a Vec<Value>> {
    get_path(value, path).and_then(|v| v.as_array())
}

/// Create a JSON object with key-value pairs.
#[macro_export]
macro_rules! json_object {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = serde_json::Map::new();
        $(
            map.insert($key.to_string(), serde_json::json!($value));
        )*
        serde_json::Value::Object(map)
    }};
}

/// Check if value is empty (null, empty string, empty array, empty object).
pub fn is_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    }
}

/// Remove null values from object.
pub fn remove_nulls(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let filtered: Map<String, Value> = map
                .into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, remove_nulls(v)))
                .collect();
            Value::Object(filtered)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(remove_nulls).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_path() {
        let value = json!({
            "data": {
                "items": [{"name": "test"}]
            }
        });
        assert_eq!(get_str(&value, "data.items.0.name"), Some("test"));
    }
}
