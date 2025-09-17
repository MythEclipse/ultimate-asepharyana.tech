// URL constants and dynamic environment-based URLs using CONFIG_MAP

use crate::config::CONFIG_MAP;

pub const ANIMEAPI: &str = "https://anime.asepharyana.tech";
pub const BASE_URL: &str = "http://127.0.0.1:4090";

// Get Komik URL from environment config
pub fn get_komik_url() -> Option<String> {
  CONFIG_MAP.get("NEXT_PUBLIC_KOMIK").cloned()
}

// Get production URL from environment config, fallback to default
pub fn get_production_url() -> String {
  CONFIG_MAP.get("NEXT_PUBLIC_PROD")
    .cloned()
    .unwrap_or_else(|| "https://asepharyana.tech".to_string())
}

// Get Komik2 URL from environment config
pub fn get_komik2_url() -> String {
  CONFIG_MAP.get("KOMIK2_BASE_URL")
    .cloned()
    .unwrap_or_else(|| "https://komiku.org".to_string())
}

// Get Komik2 API URL from environment config
pub fn get_komik2_api_url() -> String {
  CONFIG_MAP.get("KOMIK2_API_URL")
    .cloned()
    .unwrap_or_else(|| "https://api.komiku.org".to_string())
}
