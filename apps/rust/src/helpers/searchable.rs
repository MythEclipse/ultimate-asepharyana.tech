//! Searchable trait for simple full-text search.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::searchable::{Searchable, SearchQuery, search};
//!
//! impl Searchable for User {
//!     fn searchable_fields() -> Vec<&'static str> {
//!         vec!["name", "email", "bio"]
//!     }
//! }
//!
//! let query = SearchQuery::new("john")
//!     .fields(vec!["name", "email"]);
//! ```

use sea_orm::{ColumnTrait, Condition};
use serde::{Deserialize, Serialize};

/// Searchable trait for models.
pub trait Searchable {
    /// Get searchable field names.
    fn searchable_fields() -> Vec<&'static str>;

    /// Get default search weight for a field (1-10).
    fn field_weight(_field: &str) -> u32 {
        1
    }
}

/// Search query options.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search term.
    pub term: String,
    /// Fields to search (empty = all).
    pub fields: Vec<String>,
    /// Minimum match score.
    pub min_score: f32,
    /// Fuzzy matching.
    pub fuzzy: bool,
}

impl SearchQuery {
    pub fn new(term: &str) -> Self {
        Self {
            term: term.to_string(),
            fields: Vec::new(),
            min_score: 0.0,
            fuzzy: false,
        }
    }

    pub fn fields(mut self, fields: Vec<&str>) -> Self {
        self.fields = fields.into_iter().map(String::from).collect();
        self
    }

    pub fn min_score(mut self, score: f32) -> Self {
        self.min_score = score;
        self
    }

    pub fn fuzzy(mut self) -> Self {
        self.fuzzy = true;
        self
    }
}

/// Search result with scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub item: T,
    pub score: f32,
    pub highlights: Vec<SearchHighlight>,
}

/// Search highlight.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHighlight {
    pub field: String,
    pub snippet: String,
}

/// Simple text matching score.
pub fn calculate_score(text: &str, query: &str, fuzzy: bool) -> f32 {
    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();

    if text_lower == query_lower {
        return 1.0;
    }

    if text_lower.contains(&query_lower) {
        // Position-based scoring
        let pos = text_lower.find(&query_lower).unwrap_or(0);
        let pos_score = 1.0 - (pos as f32 / text.len() as f32);
        return 0.5 + (pos_score * 0.3);
    }

    if fuzzy {
        // Simple fuzzy: word overlap
        let text_words: Vec<&str> = text_lower.split_whitespace().collect();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();

        let mut matches = 0;
        for qw in &query_words {
            for tw in &text_words {
                if tw.contains(qw) || qw.contains(tw) {
                    matches += 1;
                    break;
                }
            }
        }

        if !query_words.is_empty() {
            return matches as f32 / query_words.len() as f32 * 0.5;
        }
    }

    0.0
}

/// Generate highlight snippet.
pub fn generate_highlight(text: &str, query: &str, context_len: usize) -> Option<String> {
    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();

    if let Some(pos) = text_lower.find(&query_lower) {
        let start = pos.saturating_sub(context_len);
        let end = (pos + query.len() + context_len).min(text.len());

        let mut snippet = String::new();
        if start > 0 {
            snippet.push_str("...");
        }
        snippet.push_str(&text[start..end]);
        if end < text.len() {
            snippet.push_str("...");
        }

        return Some(snippet);
    }

    None
}

/// Search in a vector of strings.
pub fn search_in_vec(items: &[String], query: &SearchQuery) -> Vec<(usize, f32)> {
    let mut results: Vec<(usize, f32)> = items
        .iter()
        .enumerate()
        .filter_map(|(i, text)| {
            let score = calculate_score(text, &query.term, query.fuzzy);
            if score >= query.min_score {
                Some((i, score))
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    results
}

/// Build LIKE conditions for search.
pub fn build_like_condition<C: ColumnTrait>(columns: Vec<C>, term: &str) -> Condition {
    let pattern = format!("%{}%", term);
    let mut condition = Condition::any();

    for col in columns {
        condition = condition.add(col.contains(&pattern));
    }

    condition
}

/// Macro to implement searchable for entity.
#[macro_export]
macro_rules! impl_searchable {
    ($entity:ty, $($field:ident),+) => {
        impl $crate::helpers::searchable::Searchable for $entity {
            fn searchable_fields() -> Vec<&'static str> {
                vec![$(stringify!($field)),+]
            }
        }
    };
}
