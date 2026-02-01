//! Environment variable utilities.

use std::env;

/// Get environment variable or panic.
pub fn require(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing required env var: {}", key))
}

/// Get environment variable or default.
pub fn get_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Get environment variable as Option.
pub fn get(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// Get environment variable as i64 or default.
pub fn get_i64(key: &str, default: i64) -> i64 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Get environment variable as u64 or default.
pub fn get_u64(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Get environment variable as bool (true, 1, yes).
pub fn get_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes"))
        .unwrap_or(default)
}

/// Get environment variable as f64 or default.
pub fn get_f64(key: &str, default: f64) -> f64 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Check if running in production.
pub fn is_production() -> bool {
    get_or("RUST_ENV", "development") == "production"
        || get_or("NODE_ENV", "development") == "production"
}

/// Check if running in development.
pub fn is_development() -> bool {
    !is_production()
}

/// Check if debug mode.
pub fn is_debug() -> bool {
    get_bool("DEBUG", false) || cfg!(debug_assertions)
}

/// Get database URL.
pub fn database_url() -> String {
    require("DATABASE_URL")
}

/// Get Redis URL.
pub fn redis_url() -> String {
    get_or("REDIS_URL", "redis://localhost:6379")
}

/// Get port number.
pub fn port() -> u16 {
    get_u64("PORT", 3000) as u16
}

/// Get host.
pub fn host() -> String {
    get_or("HOST", "0.0.0.0")
}

/// Get API key.
pub fn api_key() -> Option<String> {
    get("API_KEY")
}

/// Get JWT secret.
pub fn jwt_secret() -> String {
    get_or("JWT_SECRET", "your-super-secret-key-change-in-production")
}

/// Load .env file if present.
pub fn load_dotenv() {
    let _ = dotenvy::dotenv();
}

/// Set environment variable.
pub fn set(key: &str, value: &str) {
    env::set_var(key, value);
}

/// Remove environment variable.
pub fn remove(key: &str) {
    env::remove_var(key);
}
