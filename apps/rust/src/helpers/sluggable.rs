//! Sluggable trait for auto-generating URL slugs.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::sluggable::{Sluggable, slugify};
//!
//! let slug = slugify("Hello World! This is a Test");
//! // Returns: "hello-world-this-is-a-test"
//!
//! impl Sluggable for Post {
//!     fn slug_source(&self) -> &str { &self.title }
//! }
//! ```

use std::collections::HashMap;

/// Sluggable trait for models.
pub trait Sluggable {
    /// Get the source field for slug generation.
    fn slug_source(&self) -> &str;

    /// Generate slug from source.
    fn generate_slug(&self) -> String {
        slugify(self.slug_source())
    }

    /// Generate unique slug with suffix.
    fn generate_unique_slug(&self, existing: &[String]) -> String {
        let base = slugify(self.slug_source());
        unique_slug(&base, existing)
    }
}

/// Convert a string to a URL-safe slug.
pub fn slugify(input: &str) -> String {
    let mut result = String::new();
    let mut last_was_separator = true; // Avoid leading separators

    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && (c.is_whitespace() || c == '-' || c == '_') {
            result.push('-');
            last_was_separator = true;
        }
        // Skip other characters (punctuation, etc.)
    }

    // Remove trailing separator
    while result.ends_with('-') {
        result.pop();
    }

    result
}

/// Generate a unique slug by appending a number if needed.
pub fn unique_slug(base: &str, existing: &[String]) -> String {
    let slug = slugify(base);

    if !existing.contains(&slug) {
        return slug;
    }

    let mut counter = 2;
    loop {
        let candidate = format!("{}-{}", slug, counter);
        if !existing.contains(&candidate) {
            return candidate;
        }
        counter += 1;
        if counter > 1000 {
            // Safety limit
            return format!("{}-{}", slug, uuid::Uuid::new_v4());
        }
    }
}

/// Transliterate common accented characters.
pub fn transliterate(input: &str) -> String {
    let mappings: HashMap<char, &str> = [
        ('á', "a"),
        ('à', "a"),
        ('ä', "a"),
        ('â', "a"),
        ('ã', "a"),
        ('é', "e"),
        ('è', "e"),
        ('ë', "e"),
        ('ê', "e"),
        ('í', "i"),
        ('ì', "i"),
        ('ï', "i"),
        ('î', "i"),
        ('ó', "o"),
        ('ò', "o"),
        ('ö', "o"),
        ('ô', "o"),
        ('õ', "o"),
        ('ú', "u"),
        ('ù', "u"),
        ('ü', "u"),
        ('û', "u"),
        ('ñ', "n"),
        ('ç', "c"),
        ('ß', "ss"),
        ('æ', "ae"),
        ('œ', "oe"),
    ]
    .into_iter()
    .collect();

    input
        .chars()
        .map(|c| {
            mappings
                .get(&c.to_lowercase().next().unwrap_or(c))
                .map(|s| s.to_string())
                .unwrap_or_else(|| c.to_string())
        })
        .collect()
}

/// Slugify with transliteration.
pub fn slugify_unicode(input: &str) -> String {
    slugify(&transliterate(input))
}

/// Truncate slug to max length.
pub fn truncate_slug(slug: &str, max_length: usize) -> String {
    if slug.len() <= max_length {
        return slug.to_string();
    }

    // Find last hyphen before max_length
    let truncated = &slug[..max_length];
    if let Some(pos) = truncated.rfind('-') {
        truncated[..pos].to_string()
    } else {
        truncated.to_string()
    }
}

/// Macro to implement Sluggable for a model.
#[macro_export]
macro_rules! impl_sluggable {
    ($model:ty, $field:ident) => {
        impl $crate::helpers::sluggable::Sluggable for $model {
            fn slug_source(&self) -> &str {
                &self.$field
            }
        }
    };
}
