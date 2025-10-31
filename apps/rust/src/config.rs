// Loads and exposes all environment variables as a static, dynamic HashMap using dotenv and once_cell.
// This approach is fully dynamic: all env vars are available at runtime via CONFIG_MAP.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;

// Static config map, loaded once at startup
pub static CONFIG_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // Always load .env when loading config
    match dotenvy::dotenv() {
        Ok(path) => tracing::info!("Loaded environment from {:?}", path),
        Err(e) => tracing::warn!("Could not load .env file: {}", e),
    }

    let mut map = HashMap::new();
    for (key_os, value_os) in env::vars_os() {
        let key = key_os.to_string_lossy().to_string();
        let value = value_os.to_string_lossy().to_string();

        // Skip variables with control characters to avoid issues
        if value.contains('\r') || value.contains('\n') || value.contains('\x1b') {
            tracing::info!("{} = <skipped/control chars>", key);
            continue;
        }

        tracing::info!("{} = {}", key, value);
        map.insert(key, value);
    }
    map
});

// Usage example:
// let db_url = CONFIG_MAP.get("DATABASE_URL");
