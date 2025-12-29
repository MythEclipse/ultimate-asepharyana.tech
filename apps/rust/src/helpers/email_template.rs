//! Email Templates for HTML emails.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::email_template::{EmailTemplate, TemplateEngine};
//!
//! let engine = TemplateEngine::new();
//! engine.register("welcome", include_str!("templates/welcome.html"));
//!
//! let html = engine.render("welcome", json!({
//!     "name": "John",
//!     "link": "https://example.com/verify"
//! }))?;
//! ```

use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

/// Template error.
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    NotFound(String),
    #[error("Render error: {0}")]
    RenderError(String),
}

/// Email template.
#[derive(Debug, Clone)]
pub struct EmailTemplate {
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}

impl EmailTemplate {
    pub fn new(subject: &str, html: &str) -> Self {
        Self {
            subject: subject.to_string(),
            html: html.to_string(),
            text: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

/// Template engine with simple mustache-like syntax.
pub struct TemplateEngine {
    templates: RwLock<HashMap<String, String>>,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    /// Create a new template engine.
    pub fn new() -> Self {
        Self {
            templates: RwLock::new(HashMap::new()),
        }
    }

    /// Register a template.
    pub fn register(&self, name: &str, template: &str) {
        if let Ok(mut templates) = self.templates.write() {
            templates.insert(name.to_string(), template.to_string());
        }
    }

    /// Render a template with data.
    pub fn render<T: Serialize>(&self, name: &str, data: T) -> Result<String, TemplateError> {
        let templates = self
            .templates
            .read()
            .map_err(|e| TemplateError::RenderError(e.to_string()))?;
        let template = templates
            .get(name)
            .ok_or_else(|| TemplateError::NotFound(name.to_string()))?;

        let value =
            serde_json::to_value(&data).map_err(|e| TemplateError::RenderError(e.to_string()))?;
        Ok(render_template(template, &value))
    }

    /// Render inline template.
    pub fn render_inline<T: Serialize>(template: &str, data: T) -> Result<String, TemplateError> {
        let value =
            serde_json::to_value(&data).map_err(|e| TemplateError::RenderError(e.to_string()))?;
        Ok(render_template(template, &value))
    }
}

/// Simple template rendering - replaces {{key}} with values.
fn render_template(template: &str, data: &Value) -> String {
    let mut result = template.to_string();

    // Find all {{...}} patterns and replace them
    let i = 0;
    while let Some(start) = result[i..].find("{{") {
        let abs_start = i + start;
        if let Some(end) = result[abs_start..].find("}}") {
            let abs_end = abs_start + end;
            let key = result[abs_start + 2..abs_end].trim();

            // Skip conditionals and loops (just remove them for simplicity)
            if key.starts_with('#') || key.starts_with('/') {
                let replacement = "";
                result.replace_range(abs_start..abs_end + 2, replacement);
                continue;
            }

            let value = get_nested_value(data, key);
            result.replace_range(abs_start..abs_end + 2, &value);
        } else {
            break;
        }
    }

    result
}

fn get_value<'a>(data: &'a Value, key: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = data;

    for part in parts {
        current = current.get(part)?;
    }

    Some(current)
}

fn get_nested_value(data: &Value, key: &str) -> String {
    get_value(data, key)
        .map(|v| match v {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            _ => v.to_string(),
        })
        .unwrap_or_default()
}

// =============================================================================
// Pre-built email templates
// =============================================================================

/// Generate a welcome email.
pub fn welcome_email(name: &str, verify_url: &str) -> EmailTemplate {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .button {{ display: inline-block; padding: 12px 24px; background: #007bff; color: white; text-decoration: none; border-radius: 4px; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Welcome, {}!</h1>
        <p>Thank you for joining us. Please verify your email address:</p>
        <p><a href="{}" class="button">Verify Email</a></p>
    </div>
</body>
</html>"#,
        name, verify_url
    );

    EmailTemplate::new(&format!("Welcome, {}!", name), &html)
}

/// Generate a password reset email.
pub fn password_reset_email(name: &str, reset_url: &str) -> EmailTemplate {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .button {{ display: inline-block; padding: 12px 24px; background: #dc3545; color: white; text-decoration: none; border-radius: 4px; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Password Reset</h1>
        <p>Hi {},</p>
        <p>Click the button below to reset your password:</p>
        <p><a href="{}" class="button">Reset Password</a></p>
        <p>This link expires in 1 hour.</p>
    </div>
</body>
</html>"#,
        name, reset_url
    );

    EmailTemplate::new("Password Reset Request", &html)
}

/// Generate a notification email.
pub fn notification_email(title: &str, message: &str) -> EmailTemplate {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>{}</h1>
        <p>{}</p>
    </div>
</body>
</html>"#,
        title, message
    );

    EmailTemplate::new(title, &html)
}
