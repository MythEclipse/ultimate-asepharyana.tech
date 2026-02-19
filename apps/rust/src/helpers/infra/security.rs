//! Security utilities.

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a secure random hex string.
pub fn random_hex(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect()
}

/// Generate a CSRF token.
pub fn generate_csrf_token() -> String {
    random_hex(32)
}

/// Generate a session ID.
pub fn generate_session_id() -> String {
    random_hex(32)
}

/// Generate an API key.
pub fn generate_api_key() -> String {
    format!("sk_{}", random_hex(24))
}

/// Generate a refresh token.
pub fn generate_refresh_token() -> String {
    format!("rt_{}", random_hex(32))
}

/// Constant-time string comparison (prevent timing attacks).
pub fn secure_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result: u8 = 0;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

use once_cell::sync::Lazy;
/// Simple rate limit check (in-memory, for single instance).
use std::collections::HashMap;
use std::sync::Mutex;

static RATE_LIMITS: Lazy<Mutex<HashMap<String, (u64, u32)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Check rate limit (returns true if within limit).
pub fn check_rate_limit(key: &str, max_requests: u32, window_secs: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut limits = RATE_LIMITS.lock().unwrap();

    if let Some((window_start, count)) = limits.get_mut(key) {
        if now - *window_start > window_secs {
            // Window expired, reset
            *window_start = now;
            *count = 1;
            true
        } else if *count < max_requests {
            *count += 1;
            true
        } else {
            false
        }
    } else {
        limits.insert(key.to_string(), (now, 1));
        true
    }
}

/// Clear rate limit for a key.
pub fn clear_rate_limit(key: &str) {
    let mut limits = RATE_LIMITS.lock().unwrap();
    limits.remove(key);
}

/// Sanitize HTML to prevent XSS.
pub fn sanitize_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Remove potentially dangerous characters.
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect()
}

/// Validate and sanitize filename.
pub fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect::<String>()
        .trim_start_matches('.')
        .to_string()
}

/// Mask sensitive data (show first/last n chars).
pub fn mask_sensitive(data: &str, show_chars: usize) -> String {
    if data.len() <= show_chars * 2 {
        return "*".repeat(data.len());
    }

    let start = &data[..show_chars];
    let end = &data[data.len() - show_chars..];
    let middle = "*".repeat(data.len() - show_chars * 2);

    format!("{}{}{}", start, middle, end)
}
