//! Import/Export helpers for CSV, JSON data.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::import_export::{CsvExporter, JsonExporter};
//!
//! // Export to CSV
//! let csv = CsvExporter::export(&users)?;
//!
//! // Import from JSON
//! let users: Vec<User> = JsonImporter::import(&json_str)?;
//! ```

use serde::{de::DeserializeOwned, Serialize};

/// Import/Export error.
#[derive(Debug, thiserror::Error)]
pub enum ImportExportError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Invalid format")]
    InvalidFormat,
}

// =============================================================================
// JSON Import/Export
// =============================================================================

/// Export items to JSON string.
pub fn export_json<T: Serialize>(items: &[T]) -> Result<String, ImportExportError> {
    serde_json::to_string_pretty(items).map_err(|e| ImportExportError::ParseError(e.to_string()))
}

/// Export items to JSON file.
pub fn export_json_file<T: Serialize>(items: &[T], path: &str) -> Result<(), ImportExportError> {
    let json = export_json(items)?;
    std::fs::write(path, json).map_err(|e| ImportExportError::IoError(e.to_string()))
}

/// Import from JSON string.
pub fn import_json<T: DeserializeOwned>(json: &str) -> Result<Vec<T>, ImportExportError> {
    serde_json::from_str(json).map_err(|e| ImportExportError::ParseError(e.to_string()))
}

/// Import from JSON file.
pub fn import_json_file<T: DeserializeOwned>(path: &str) -> Result<Vec<T>, ImportExportError> {
    let content =
        std::fs::read_to_string(path).map_err(|e| ImportExportError::IoError(e.to_string()))?;
    import_json(&content)
}

// =============================================================================
// CSV Import/Export (Simple implementation)
// =============================================================================

/// Export items to CSV string.
pub fn export_csv<T: Serialize>(items: &[T]) -> Result<String, ImportExportError> {
    if items.is_empty() {
        return Ok(String::new());
    }

    // Convert to JSON first to get field names
    let json_items: Vec<serde_json::Value> = items
        .iter()
        .map(|item| serde_json::to_value(item))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ImportExportError::ParseError(e.to_string()))?;

    // Get headers from first item
    let headers: Vec<String> = match &json_items[0] {
        serde_json::Value::Object(map) => map.keys().cloned().collect(),
        _ => return Err(ImportExportError::InvalidFormat),
    };

    let mut csv = String::new();

    // Write header
    csv.push_str(&headers.join(","));
    csv.push('\n');

    // Write rows
    for item in &json_items {
        if let serde_json::Value::Object(map) = item {
            let row: Vec<String> = headers
                .iter()
                .map(|h| map.get(h).map(|v| escape_csv_value(v)).unwrap_or_default())
                .collect();
            csv.push_str(&row.join(","));
            csv.push('\n');
        }
    }

    Ok(csv)
}

/// Export to CSV file.
pub fn export_csv_file<T: Serialize>(items: &[T], path: &str) -> Result<(), ImportExportError> {
    let csv = export_csv(items)?;
    std::fs::write(path, csv).map_err(|e| ImportExportError::IoError(e.to_string()))
}

/// Import from CSV string.
pub fn import_csv<T: DeserializeOwned>(csv: &str) -> Result<Vec<T>, ImportExportError> {
    let mut lines = csv.lines();

    // Parse header
    let header_line = lines.next().ok_or(ImportExportError::InvalidFormat)?;
    let headers: Vec<&str> = parse_csv_line(header_line);

    // Parse rows
    let mut items = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let values = parse_csv_line(line);
        let mut map = serde_json::Map::new();

        for (i, header) in headers.iter().enumerate() {
            let value = values.get(i).copied().unwrap_or("");
            map.insert(
                header.to_string(),
                if value.is_empty() {
                    serde_json::Value::Null
                } else if let Ok(n) = value.parse::<i64>() {
                    serde_json::Value::Number(n.into())
                } else if let Ok(f) = value.parse::<f64>() {
                    serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into()))
                } else if value == "true" {
                    serde_json::Value::Bool(true)
                } else if value == "false" {
                    serde_json::Value::Bool(false)
                } else {
                    serde_json::Value::String(value.to_string())
                },
            );
        }

        let item: T = serde_json::from_value(serde_json::Value::Object(map))
            .map_err(|e| ImportExportError::ParseError(e.to_string()))?;
        items.push(item);
    }

    Ok(items)
}

/// Import from CSV file.
pub fn import_csv_file<T: DeserializeOwned>(path: &str) -> Result<Vec<T>, ImportExportError> {
    let content =
        std::fs::read_to_string(path).map_err(|e| ImportExportError::IoError(e.to_string()))?;
    import_csv(&content)
}

fn escape_csv_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => {
            if s.contains(',') || s.contains('"') || s.contains('\n') {
                format!("\"{}\"", s.replace('"', "\"\""))
            } else {
                s.clone()
            }
        }
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        _ => value.to_string(),
    }
}

fn parse_csv_line(line: &str) -> Vec<&str> {
    // Simple CSV parsing (doesn't handle all edge cases)
    let mut fields = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;

    for (i, c) in line.char_indices() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(&line[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    fields.push(&line[start..]);

    // Remove surrounding quotes
    fields
        .into_iter()
        .map(|f| f.trim().trim_matches('"'))
        .collect()
}

// =============================================================================
// NDJSON (Newline Delimited JSON)
// =============================================================================

/// Export to NDJSON.
pub fn export_ndjson<T: Serialize>(items: &[T]) -> Result<String, ImportExportError> {
    let lines: Vec<String> = items
        .iter()
        .map(|item| serde_json::to_string(item))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ImportExportError::ParseError(e.to_string()))?;

    Ok(lines.join("\n"))
}

/// Import from NDJSON.
pub fn import_ndjson<T: DeserializeOwned>(ndjson: &str) -> Result<Vec<T>, ImportExportError> {
    ndjson
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line).map_err(|e| ImportExportError::ParseError(e.to_string()))
        })
        .collect()
}
