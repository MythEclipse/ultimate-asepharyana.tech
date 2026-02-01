//! URL building and manipulation utilities.

use std::collections::HashMap;

/// URL builder for constructing URLs with query parameters.
#[derive(Debug, Clone)]
pub struct UrlBuilder {
    base: String,
    path_segments: Vec<String>,
    query_params: Vec<(String, String)>,
}

impl UrlBuilder {
    /// Create a new URL builder with base URL.
    pub fn new(base: impl Into<String>) -> Self {
        let base = base.into();
        let base = base.trim_end_matches('/').to_string();
        Self {
            base,
            path_segments: Vec::new(),
            query_params: Vec::new(),
        }
    }

    /// Add a path segment.
    pub fn path(mut self, segment: impl Into<String>) -> Self {
        self.path_segments.push(segment.into());
        self
    }

    /// Add multiple path segments.
    pub fn paths(mut self, segments: &[&str]) -> Self {
        for seg in segments {
            self.path_segments.push(seg.to_string());
        }
        self
    }

    /// Add a query parameter.
    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// Add optional query parameter (only if Some).
    pub fn query_opt(self, key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        match value {
            Some(v) => self.query(key, v),
            None => self,
        }
    }

    /// Add multiple query parameters from HashMap.
    pub fn query_map(mut self, params: HashMap<String, String>) -> Self {
        for (k, v) in params {
            self.query_params.push((k, v));
        }
        self
    }

    /// Build the final URL string.
    pub fn build(self) -> String {
        let mut url = self.base;

        // Add path segments
        for segment in self.path_segments {
            url.push('/');
            url.push_str(&urlencoding::encode(&segment));
        }

        // Add query params
        if !self.query_params.is_empty() {
            url.push('?');
            let params: Vec<String> = self
                .query_params
                .into_iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(&k), urlencoding::encode(&v)))
                .collect();
            url.push_str(&params.join("&"));
        }

        url
    }
}

/// Encode a string for use in URL.
pub fn encode(s: &str) -> String {
    urlencoding::encode(s).to_string()
}

/// Decode a URL-encoded string.
pub fn decode(s: &str) -> Result<String, std::string::FromUtf8Error> {
    urlencoding::decode(s).map(|s| s.to_string())
}

/// Parse query string into HashMap.
pub fn parse_query(query: &str) -> HashMap<String, String> {
    let query = query.trim_start_matches('?');
    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((decode(key).ok()?, decode(value).ok()?))
        })
        .collect()
}

/// Extract domain from URL.
pub fn extract_domain(url: &str) -> Option<String> {
    let url = url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    url.split('/').next().map(|s| s.to_string())
}

/// Join URL paths safely.
pub fn join_paths(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

/// Check if URL is absolute.
pub fn is_absolute(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Make URL absolute.
pub fn make_absolute(url: &str, base: &str) -> String {
    if is_absolute(url) {
        url.to_string()
    } else {
        join_paths(base, url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_builder() {
        let url = UrlBuilder::new("https://api.example.com")
            .path("users")
            .path("123")
            .query("page", "1")
            .query("limit", "10")
            .build();
        assert_eq!(url, "https://api.example.com/users/123?page=1&limit=10");
    }

    #[test]
    fn test_parse_query() {
        let params = parse_query("?name=John&age=30");
        assert_eq!(params.get("name"), Some(&"John".to_string()));
    }
}
