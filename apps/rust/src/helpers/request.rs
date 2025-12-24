//! HTTP request helpers.

use axum::http::{HeaderMap, HeaderValue};

/// Extract client IP from headers (X-Forwarded-For, X-Real-IP).
pub fn client_ip(headers: &HeaderMap) -> Option<String> {
    // Try X-Forwarded-For first
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            if let Some(ip) = value.split(',').next() {
                return Some(ip.trim().to_string());
            }
        }
    }

    // Try X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            return Some(value.to_string());
        }
    }

    // Try CF-Connecting-IP (Cloudflare)
    if let Some(cf_ip) = headers.get("cf-connecting-ip") {
        if let Ok(value) = cf_ip.to_str() {
            return Some(value.to_string());
        }
    }

    None
}

/// Extract User-Agent from headers.
pub fn user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

/// Extract Accept-Language from headers.
pub fn accept_language(headers: &HeaderMap) -> Option<String> {
    headers
        .get("accept-language")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

/// Extract Authorization bearer token.
pub fn bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(String::from)
}

/// Extract content type.
pub fn content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

/// Check if request is JSON.
pub fn is_json(headers: &HeaderMap) -> bool {
    content_type(headers)
        .map(|ct| ct.contains("application/json"))
        .unwrap_or(false)
}

/// Check if request is form data.
pub fn is_form(headers: &HeaderMap) -> bool {
    content_type(headers)
        .map(|ct| {
            ct.contains("application/x-www-form-urlencoded") || ct.contains("multipart/form-data")
        })
        .unwrap_or(false)
}

/// Extract referer.
pub fn referer(headers: &HeaderMap) -> Option<String> {
    headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

/// Extract origin.
pub fn origin(headers: &HeaderMap) -> Option<String> {
    headers
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

/// Extract request ID.
pub fn request_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-request-id")
        .or_else(|| headers.get("x-correlation-id"))
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

/// Check if request accepts gzip.
pub fn accepts_gzip(headers: &HeaderMap) -> bool {
    headers
        .get("accept-encoding")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("gzip"))
        .unwrap_or(false)
}

/// Create header value.
pub fn header_value(s: &str) -> HeaderValue {
    HeaderValue::from_str(s).unwrap_or_else(|_| HeaderValue::from_static(""))
}

/// Parse quality value from Accept header (e.g., "text/html;q=0.9").
pub fn parse_accept_quality(accept: &str) -> Vec<(String, f32)> {
    accept
        .split(',')
        .filter_map(|part| {
            let mut parts = part.trim().split(';');
            let mime = parts.next()?.trim().to_string();
            let quality = parts
                .find_map(|p| {
                    let p = p.trim();
                    if p.starts_with("q=") {
                        p[2..].parse().ok()
                    } else {
                        None
                    }
                })
                .unwrap_or(1.0);
            Some((mime, quality))
        })
        .collect()
}
