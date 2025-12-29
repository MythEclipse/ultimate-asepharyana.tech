//! API Resources / Transformers.
//!
//! Transform models for API output with field selection and relationships.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::resource::{Resource, ResourceCollection};
//!
//! struct UserResource;
//! impl Resource for UserResource {
//!     type Model = User;
//!     fn transform(model: &Self::Model) -> serde_json::Value {
//!         json!({ "id": model.id, "name": model.name })
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Resource trait for transforming models to API output.
pub trait Resource {
    /// The model type being transformed.
    type Model;

    /// Transform a single model to JSON.
    fn transform(model: &Self::Model) -> Value;

    /// Transform with additional data.
    fn transform_with(model: &Self::Model, _extra: &HashMap<String, Value>) -> Value {
        Self::transform(model)
    }

    /// Get field whitelist (if any).
    fn fields() -> Option<Vec<&'static str>> {
        None
    }

    /// Get hidden fields.
    fn hidden() -> Vec<&'static str> {
        vec![]
    }
}

/// Resource collection for paginated results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCollection<T> {
    pub data: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<CollectionMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<CollectionLinks>,
}

/// Collection metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMeta {
    pub current_page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// Collection links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl<T> ResourceCollection<T> {
    /// Create a simple collection without pagination.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            meta: None,
            links: None,
        }
    }

    /// Create a paginated collection.
    pub fn paginated(data: Vec<T>, page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as u64;
        Self {
            data,
            meta: Some(CollectionMeta {
                current_page: page,
                per_page,
                total,
                total_pages,
            }),
            links: None,
        }
    }

    /// Add links.
    pub fn with_links(mut self, base_url: &str, page: u64, total_pages: u64) -> Self {
        let first = Some(format!("{}?page=1", base_url));
        let last = Some(format!("{}?page={}", base_url, total_pages));
        let prev = if page > 1 {
            Some(format!("{}?page={}", base_url, page - 1))
        } else {
            None
        };
        let next = if page < total_pages {
            Some(format!("{}?page={}", base_url, page + 1))
        } else {
            None
        };

        self.links = Some(CollectionLinks {
            first,
            last,
            prev,
            next,
        });
        self
    }
}

/// Transform a collection of models using a resource.
pub fn collection<R: Resource>(models: &[R::Model]) -> Vec<Value> {
    models.iter().map(R::transform).collect()
}

/// Transform a single model using a resource.
pub fn item<R: Resource>(model: &R::Model) -> Value {
    R::transform(model)
}

/// Filter fields from a JSON value.
pub fn only(value: Value, fields: &[&str]) -> Value {
    if let Value::Object(map) = value {
        let filtered: serde_json::Map<String, Value> = map
            .into_iter()
            .filter(|(k, _)| fields.contains(&k.as_str()))
            .collect();
        Value::Object(filtered)
    } else {
        value
    }
}

/// Remove fields from a JSON value.
pub fn except(value: Value, fields: &[&str]) -> Value {
    if let Value::Object(map) = value {
        let filtered: serde_json::Map<String, Value> = map
            .into_iter()
            .filter(|(k, _)| !fields.contains(&k.as_str()))
            .collect();
        Value::Object(filtered)
    } else {
        value
    }
}

/// Merge additional data into a JSON object.
pub fn merge(mut value: Value, extra: Value) -> Value {
    if let (Value::Object(ref mut map1), Value::Object(map2)) = (&mut value, extra) {
        for (k, v) in map2 {
            map1.insert(k, v);
        }
    }
    value
}

/// Wrap data in a standard API envelope.
pub fn envelope(data: Value) -> Value {
    json!({ "data": data })
}

/// Wrap data with success status.
pub fn success(data: Value) -> Value {
    json!({
        "success": true,
        "data": data
    })
}

/// Wrap error response.
pub fn error(message: &str, code: Option<&str>) -> Value {
    let mut resp = json!({
        "success": false,
        "error": { "message": message }
    });
    if let Some(c) = code {
        resp["error"]["code"] = Value::String(c.to_string());
    }
    resp
}
