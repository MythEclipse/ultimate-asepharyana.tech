//! Form Request Validation.
//!
//! Reusable validation rules for request data.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::form_request::{FormRequest, ValidationRules, validate};
//!
//! let rules = ValidationRules::new()
//!     .required("email")
//!     .email("email")
//!     .min_length("password", 8);
//!
//! let errors = validate(&data, &rules);
//! ```

use serde_json::Value;
use std::collections::HashMap;

/// Validation error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub rule: String,
}

/// Validation result.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add(&mut self, field: &str, rule: &str, message: &str) {
        self.errors.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            rule: rule.to_string(),
        });
    }

    pub fn errors_for(&self, field: &str) -> Vec<&ValidationError> {
        self.errors.iter().filter(|e| e.field == field).collect()
    }

    pub fn first_error(&self) -> Option<&ValidationError> {
        self.errors.first()
    }

    pub fn to_json(&self) -> Value {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for error in &self.errors {
            map.entry(error.field.clone())
                .or_default()
                .push(error.message.clone());
        }
        serde_json::to_value(map).unwrap_or(Value::Null)
    }
}

/// Validation rule.
#[derive(Debug, Clone)]
pub enum Rule {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(i64),
    Max(i64),
    Regex(String),
    In(Vec<String>),
    NotIn(Vec<String>),
    Confirmed(String),
    Numeric,
    Alpha,
    AlphaNumeric,
    Date,
    Uuid,
}

/// Validation rules builder.
#[derive(Debug, Clone, Default)]
pub struct ValidationRules {
    rules: HashMap<String, Vec<Rule>>,
}

impl ValidationRules {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, field: &str, rule: Rule) -> &mut Self {
        self.rules.entry(field.to_string()).or_default().push(rule);
        self
    }

    pub fn required(&mut self, field: &str) -> &mut Self {
        self.add_rule(field, Rule::Required)
    }

    pub fn email(&mut self, field: &str) -> &mut Self {
        self.add_rule(field, Rule::Email)
    }

    pub fn url(&mut self, field: &str) -> &mut Self {
        self.add_rule(field, Rule::Url)
    }

    pub fn min_length(&mut self, field: &str, len: usize) -> &mut Self {
        self.add_rule(field, Rule::MinLength(len))
    }

    pub fn max_length(&mut self, field: &str, len: usize) -> &mut Self {
        self.add_rule(field, Rule::MaxLength(len))
    }

    pub fn min(&mut self, field: &str, val: i64) -> &mut Self {
        self.add_rule(field, Rule::Min(val))
    }

    pub fn max(&mut self, field: &str, val: i64) -> &mut Self {
        self.add_rule(field, Rule::Max(val))
    }

    pub fn in_list(&mut self, field: &str, values: Vec<&str>) -> &mut Self {
        self.add_rule(
            field,
            Rule::In(values.into_iter().map(String::from).collect()),
        )
    }

    pub fn confirmed(&mut self, field: &str, confirmation_field: &str) -> &mut Self {
        self.add_rule(field, Rule::Confirmed(confirmation_field.to_string()))
    }

    pub fn numeric(&mut self, field: &str) -> &mut Self {
        self.add_rule(field, Rule::Numeric)
    }

    pub fn uuid(&mut self, field: &str) -> &mut Self {
        self.add_rule(field, Rule::Uuid)
    }
}

/// Validate data against rules.
pub fn validate(data: &Value, rules: &ValidationRules) -> ValidationResult {
    let mut result = ValidationResult::new();

    for (field, field_rules) in &rules.rules {
        let value = data.get(field);

        for rule in field_rules {
            if let Some(error) = validate_rule(field, value, rule, data) {
                result.errors.push(error);
            }
        }
    }

    result
}

fn validate_rule(
    field: &str,
    value: Option<&Value>,
    rule: &Rule,
    data: &Value,
) -> Option<ValidationError> {
    match rule {
        Rule::Required => {
            if value.is_none()
                || value == Some(&Value::Null)
                || value
                    .map(|v| v.as_str().map(|s| s.is_empty()).unwrap_or(false))
                    .unwrap_or(true)
            {
                return Some(ValidationError {
                    field: field.to_string(),
                    rule: "required".to_string(),
                    message: format!("The {} field is required.", field),
                });
            }
        }
        Rule::Email => {
            if let Some(s) = value.and_then(|v| v.as_str()) {
                if !s.contains('@') || !s.contains('.') {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "email".to_string(),
                        message: format!("The {} must be a valid email.", field),
                    });
                }
            }
        }
        Rule::MinLength(len) => {
            if let Some(s) = value.and_then(|v| v.as_str()) {
                if s.len() < *len {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "min_length".to_string(),
                        message: format!("The {} must be at least {} characters.", field, len),
                    });
                }
            }
        }
        Rule::MaxLength(len) => {
            if let Some(s) = value.and_then(|v| v.as_str()) {
                if s.len() > *len {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "max_length".to_string(),
                        message: format!("The {} must not exceed {} characters.", field, len),
                    });
                }
            }
        }
        Rule::Min(min) => {
            if let Some(n) = value.and_then(|v| v.as_i64()) {
                if n < *min {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "min".to_string(),
                        message: format!("The {} must be at least {}.", field, min),
                    });
                }
            }
        }
        Rule::Max(max) => {
            if let Some(n) = value.and_then(|v| v.as_i64()) {
                if n > *max {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "max".to_string(),
                        message: format!("The {} must not exceed {}.", field, max),
                    });
                }
            }
        }
        Rule::In(allowed) => {
            if let Some(s) = value.and_then(|v| v.as_str()) {
                if !allowed.contains(&s.to_string()) {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "in".to_string(),
                        message: format!("The {} must be one of: {}", field, allowed.join(", ")),
                    });
                }
            }
        }
        Rule::Confirmed(confirm_field) => {
            let confirm_value = data.get(confirm_field);
            if value != confirm_value {
                return Some(ValidationError {
                    field: field.to_string(),
                    rule: "confirmed".to_string(),
                    message: format!("The {} confirmation does not match.", field),
                });
            }
        }
        Rule::Numeric => {
            if let Some(v) = value {
                if !v.is_number()
                    && v.as_str()
                        .map(|s| s.parse::<f64>().is_err())
                        .unwrap_or(true)
                {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "numeric".to_string(),
                        message: format!("The {} must be numeric.", field),
                    });
                }
            }
        }
        Rule::Uuid => {
            if let Some(s) = value.and_then(|v| v.as_str()) {
                if uuid::Uuid::parse_str(s).is_err() {
                    return Some(ValidationError {
                        field: field.to_string(),
                        rule: "uuid".to_string(),
                        message: format!("The {} must be a valid UUID.", field),
                    });
                }
            }
        }
        _ => {}
    }

    None
}
