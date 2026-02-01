//! UUID utilities.

use uuid::Uuid;

/// Generate a new UUID v4.
pub fn new_v4() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a new UUID v4 without hyphens.
pub fn new_v4_simple() -> String {
    Uuid::new_v4().simple().to_string()
}

/// Parse a UUID string.
pub fn parse(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s)
}

/// Check if string is valid UUID.
pub fn is_valid(s: &str) -> bool {
    Uuid::parse_str(s).is_ok()
}

/// Convert to hyphenated format.
pub fn to_hyphenated(s: &str) -> Option<String> {
    Uuid::parse_str(s).ok().map(|u| u.hyphenated().to_string())
}

/// Convert to simple format (no hyphens).
pub fn to_simple(s: &str) -> Option<String> {
    Uuid::parse_str(s).ok().map(|u| u.simple().to_string())
}

/// Generate a nil UUID (all zeros).
pub fn nil() -> String {
    Uuid::nil().to_string()
}

/// Check if UUID is nil.
pub fn is_nil(s: &str) -> bool {
    parse(s).map(|u| u.is_nil()).unwrap_or(false)
}

/// Extract timestamp from UUID v7 (if applicable).
pub fn timestamp_v7(s: &str) -> Option<u64> {
    parse(s).ok().and_then(|u| {
        if u.get_version() == Some(uuid::Version::SortRand) {
            u.get_timestamp().map(|ts| {
                let (secs, _) = ts.to_unix();
                secs
            })
        } else {
            None
        }
    })
}

/// Create a UUID namespace for v5.
pub fn namespace(ns: &str) -> Option<Uuid> {
    match ns.to_lowercase().as_str() {
        "dns" => Some(Uuid::NAMESPACE_DNS),
        "url" => Some(Uuid::NAMESPACE_URL),
        "oid" => Some(Uuid::NAMESPACE_OID),
        "x500" => Some(Uuid::NAMESPACE_X500),
        _ => Uuid::parse_str(ns).ok(),
    }
}

/// Generate a short ID (first 8 chars of UUID).
pub fn short_id() -> String {
    Uuid::new_v4().simple().to_string()[..8].to_string()
}

/// Generate a medium ID (first 12 chars of UUID).
pub fn medium_id() -> String {
    Uuid::new_v4().simple().to_string()[..12].to_string()
}
