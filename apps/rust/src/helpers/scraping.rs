//! HTML scraping helpers using scraper crate.

use crate::helpers::{default_backoff, transient};
use crate::infra::proxy::fetch_with_proxy;
use backoff::future::retry;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use tracing::{info, warn};

/// Fetch HTML from URL with retry backoff and proxy support.
pub async fn fetch_html_with_retry(
    url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let backoff = default_backoff();
    let fetch_operation = || async {
        info!("Fetching: {}", url);
        match fetch_with_proxy(url).await {
            Ok(response) => {
                info!("Successfully fetched: {}", url);
                Ok(response.data)
            }
            Err(e) => {
                warn!("Failed to fetch: {}, error: {:?}", url, e);
                Err(transient(e))
            }
        }
    };

    Ok(retry(backoff, fetch_operation).await?)
}

/// Parse HTML string into a document.
pub fn parse_html(html: &str) -> Html {
    Html::parse_document(html)
}

/// Safely create a CSS selector.
pub fn selector(css: &str) -> Option<Selector> {
    Selector::parse(css).ok()
}

/// Extract text content from an element, trimmed.
pub fn text(element: &ElementRef) -> String {
    element.text().collect::<String>().trim().to_string()
}

/// Extract text from first matching element.
pub fn select_text(element: &ElementRef, css: &str) -> Option<String> {
    let sel = selector(css)?;
    element.select(&sel).next().map(|e| text(&e))
}

/// Extract attribute from first matching element.
pub fn select_attr(element: &ElementRef, css: &str, attr: &str) -> Option<String> {
    let sel = selector(css)?;
    element
        .select(&sel)
        .next()
        .and_then(|e| e.value().attr(attr))
        .map(String::from)
}

/// Extract attribute from element.
pub fn attr(element: &ElementRef, name: &str) -> Option<String> {
    element.value().attr(name).map(String::from)
}

/// Select all matching elements.
pub fn select_all<'a>(document: &'a Html, css: &str) -> Vec<ElementRef<'a>> {
    selector(css)
        .map(|s| document.select(&s).collect())
        .unwrap_or_default()
}

/// Extract slug from URL (last path segment).
pub fn extract_slug(url: &str) -> String {
    static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/([^/]+)/?$").unwrap());

    SLUG_REGEX
        .captures(url)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default()
}

/// Remove HTML tags from string.
pub fn strip_tags(html: &str) -> String {
    static TAG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").unwrap());
    TAG_REGEX.replace_all(html, "").trim().to_string()
}

/// Extract number from text.
pub fn extract_number(text: &str) -> Option<i64> {
    static NUM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    NUM_REGEX.find(text).and_then(|m| m.as_str().parse().ok())
}

/// Extract text inside parentheses.
pub fn extract_parentheses(text: &str) -> Option<String> {
    static PAREN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(([^)]+)\)").unwrap());
    PAREN_REGEX
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Builder for scraping elements.
pub struct Scraper<'a> {
    element: ElementRef<'a>,
}

impl<'a> Scraper<'a> {
    pub fn new(element: ElementRef<'a>) -> Self {
        Self { element }
    }

    /// Get text from selector.
    pub fn text(&self, css: &str) -> Option<String> {
        select_text(&self.element, css)
    }

    /// Get text or default.
    pub fn text_or(&self, css: &str, default: &str) -> String {
        self.text(css).unwrap_or_else(|| default.to_string())
    }

    /// Get attribute from selector.
    pub fn attr(&self, css: &str, name: &str) -> Option<String> {
        select_attr(&self.element, css, name)
    }

    /// Get attribute or default.
    pub fn attr_or(&self, css: &str, name: &str, default: &str) -> String {
        self.attr(css, name).unwrap_or_else(|| default.to_string())
    }

    /// Get href from first link.
    pub fn href(&self, css: &str) -> Option<String> {
        self.attr(css, "href")
    }

    /// Get src from first image.
    pub fn src(&self, css: &str) -> Option<String> {
        self.attr(css, "src")
    }
}
