//! URL constants and dynamic environment-based URLs.
//!
//! Note: These URLs are kept as dynamic env lookups because they may vary
//! between deployments and are not critical startup dependencies.

use std::env;

pub const ANIMEAPI: &str = "https://anime.asepharyana.tech";
pub const BASE_URL: &str = "http://127.0.0.1:4090";
pub const OTAKUDESU_BASE_URL: &str = "https://otakudesu.best";

/// Get Komik URL from environment config.
pub fn get_komik_url() -> String {
    env::var("KOMIK2_BASE_URL").unwrap_or_else(|_| "https://komiku.org".to_string())
}

/// Get production URL from environment config.
pub fn get_production_url() -> String {
    env::var("NEXT_PUBLIC_PROD").unwrap_or_else(|_| "https://asepharyana.tech".to_string())
}

/// Get Komik API URL from environment config.
pub fn get_komik_api_url() -> String {
    env::var("KOMIK2_API_URL").unwrap_or_else(|_| "https://api.komiku.org".to_string())
}

/// Get Otakudesu URL from environment config.
pub fn get_otakudesu_url() -> String {
    env::var("OTAKUDESU_BASE_URL").unwrap_or_else(|_| OTAKUDESU_BASE_URL.to_string())
}
