//! Input validation helpers.

use once_cell::sync::Lazy;
use regex::Regex;

/// Email validation regex.
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

/// URL validation regex.
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap()
});

/// Phone validation regex (international format).
static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap()
});

/// UUID v4 validation regex.
static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap()
});

/// Slug validation regex (lowercase, hyphens, no spaces).
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap()
});

/// Validate email format.
pub fn is_email(s: &str) -> bool {
    EMAIL_REGEX.is_match(s)
}

/// Validate URL format.
pub fn is_url(s: &str) -> bool {
    URL_REGEX.is_match(s)
}

/// Validate phone number format.
pub fn is_phone(s: &str) -> bool {
    PHONE_REGEX.is_match(s)
}

/// Validate UUID v4 format.
pub fn is_uuid(s: &str) -> bool {
    UUID_REGEX.is_match(s)
}

/// Validate slug format.
pub fn is_slug(s: &str) -> bool {
    SLUG_REGEX.is_match(s)
}

/// Check if string is not empty.
pub fn is_not_empty(s: &str) -> bool {
    !s.trim().is_empty()
}

/// Check minimum length.
pub fn min_length(s: &str, min: usize) -> bool {
    s.len() >= min
}

/// Check maximum length.
pub fn max_length(s: &str, max: usize) -> bool {
    s.len() <= max
}

/// Check length is within range.
pub fn length_between(s: &str, min: usize, max: usize) -> bool {
    s.len() >= min && s.len() <= max
}

/// Check if string contains only alphanumeric characters.
pub fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric())
}

/// Check if string contains only ASCII characters.
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}

/// Check if string is numeric only.
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_numeric())
}

/// Check if number is in range.
pub fn in_range<T: PartialOrd>(val: T, min: T, max: T) -> bool {
    val >= min && val <= max
}

/// Validate password strength (min 8 chars, has upper, lower, digit).
pub fn is_strong_password(s: &str) -> bool {
    s.len() >= 8
        && s.chars().any(|c| c.is_uppercase())
        && s.chars().any(|c| c.is_lowercase())
        && s.chars().any(|c| c.is_numeric())
}

/// Validation result builder.
#[derive(Debug, Default)]
pub struct Validator {
    errors: Vec<String>,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a validation check.
    pub fn check(mut self, condition: bool, message: impl Into<String>) -> Self {
        if !condition {
            self.errors.push(message.into());
        }
        self
    }

    /// Check if validation passed.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get all errors.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Get first error.
    pub fn first_error(&self) -> Option<&String> {
        self.errors.first()
    }

    /// Convert to Result.
    pub fn validate(self) -> Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_email() {
        assert!(is_email("test@example.com"));
        assert!(!is_email("invalid"));
    }

    #[test]
    fn test_validator() {
        let result = Validator::new()
            .check(is_email("test@example.com"), "Invalid email")
            .check(min_length("password123", 8), "Password too short")
            .validate();
        assert!(result.is_ok());
    }
}
