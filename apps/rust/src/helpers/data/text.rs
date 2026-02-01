//! Text processing utilities.

use once_cell::sync::Lazy;
use regex::Regex;

/// Remove extra whitespace (multiple spaces, tabs, newlines).
pub fn normalize_whitespace(s: &str) -> String {
    static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    WHITESPACE_REGEX.replace_all(s, " ").trim().to_string()
}

/// Capitalize first letter.
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

/// Convert to camelCase.
pub fn to_camel_case(s: &str) -> String {
    let words: Vec<&str> = s
        .split(|c: char| c == '_' || c == '-' || c.is_whitespace())
        .collect();
    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        if word.is_empty() {
            continue;
        }
        if i == 0 {
            result.push_str(&word.to_lowercase());
        } else {
            result.push_str(&capitalize(&word.to_lowercase()));
        }
    }
    result
}

/// Convert to snake_case.
pub fn to_snake_case(s: &str) -> String {
    static CAMEL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z])([A-Z])").unwrap());
    let s = CAMEL_REGEX.replace_all(s, "${1}_${2}");
    s.replace('-', "_").to_lowercase()
}

/// Convert to kebab-case.
pub fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}

/// Convert to PascalCase.
pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-' || c.is_whitespace())
        .filter(|w| !w.is_empty())
        .map(|w| capitalize(&w.to_lowercase()))
        .collect()
}

/// Convert to CONSTANT_CASE.
pub fn to_constant_case(s: &str) -> String {
    to_snake_case(s).to_uppercase()
}

/// Extract words from text.
pub fn extract_words(s: &str) -> Vec<String> {
    static WORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b\w+\b").unwrap());
    WORD_REGEX
        .find_iter(s)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Count words.
pub fn word_count(s: &str) -> usize {
    extract_words(s).len()
}

/// Truncate text at word boundary.
pub fn truncate_words(s: &str, max_words: usize, suffix: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() <= max_words {
        s.to_string()
    } else {
        format!("{}{}", words[..max_words].join(" "), suffix)
    }
}

/// Wrap text at specified width.
pub fn wrap_text(s: &str, width: usize) -> String {
    let mut result = String::new();
    let mut line_len = 0;

    for word in s.split_whitespace() {
        if line_len + word.len() + 1 > width && line_len > 0 {
            result.push('\n');
            line_len = 0;
        } else if line_len > 0 {
            result.push(' ');
            line_len += 1;
        }
        result.push_str(word);
        line_len += word.len();
    }

    result
}

/// Highlight search terms in text.
pub fn highlight(text: &str, terms: &[&str], before: &str, after: &str) -> String {
    let mut result = text.to_string();
    for term in terms {
        let pattern = regex::escape(term);
        let re = Regex::new(&format!("(?i)({})", pattern)).unwrap();
        result = re
            .replace_all(&result, format!("{}$1{}", before, after))
            .to_string();
    }
    result
}

/// Remove diacritics/accents.
pub fn remove_accents(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ä' | 'ã' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            _ => c,
        })
        .collect()
}

/// Generate Lorem Ipsum text.
pub fn lorem_ipsum(sentences: usize) -> String {
    const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris. \
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum. \
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia.";

    LOREM
        .split(". ")
        .take(sentences)
        .collect::<Vec<_>>()
        .join(". ")
        + "."
}

/// Reverse a string.
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Check if palindrome.
pub fn is_palindrome(s: &str) -> bool {
    let clean: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = clean.to_lowercase();
    lower == lower.chars().rev().collect::<String>()
}
