//! Cache TTL constants for consistent caching across the application.

/// Very short TTL for highly volatile data (5 minutes)
/// Use for: Real-time data, user presence, active sessions
pub const CACHE_TTL_VERY_SHORT: u64 = 300;

/// Short TTL for frequently changing data (15 minutes)
/// Use for: Trending content, live feeds, dynamic lists
pub const CACHE_TTL_SHORT: u64 = 900;

/// Medium TTL for regular data (1 hour)
/// Use for: User profiles, search results, API responses
pub const CACHE_TTL_MEDIUM: u64 = 3600;

/// Long TTL for mostly static data (6 hours)
/// Use for: Configuration, reference data, translations
pub const CACHE_TTL_LONG: u64 = 21600;

/// Very long TTL for static content (1 day)
/// Use for: Static pages, archived content, historical data
pub const CACHE_TTL_VERY_LONG: u64 = 86400;

/// Image cache TTL (7 days)
/// Use for: Cached images, thumbnails, avatars
pub const CACHE_TTL_IMAGE: u64 = 604800;

/// CDN cache TTL (30 days)
/// Use for: Immutable assets, versioned files
pub const CACHE_TTL_CDN: u64 = 2592000;

/// Get TTL based on cache type
pub fn get_ttl_for(cache_type: CacheType) -> u64 {
    match cache_type {
        CacheType::RealTime => CACHE_TTL_VERY_SHORT,
        CacheType::Volatile => CACHE_TTL_SHORT,
        CacheType::Regular => CACHE_TTL_MEDIUM,
        CacheType::Stable => CACHE_TTL_LONG,
        CacheType::Static => CACHE_TTL_VERY_LONG,
        CacheType::Image => CACHE_TTL_IMAGE,
        CacheType::Cdn => CACHE_TTL_CDN,
    }
}

/// Cache type classification
#[derive(Debug, Clone, Copy)]
pub enum CacheType {
    /// Real-time data (5 min)
    RealTime,
    /// Volatile data (15 min)
    Volatile,
    /// Regular data (1 hour)
    Regular,
    /// Stable data (6 hours)
    Stable,
    /// Static data (1 day)
    Static,
    /// Images (7 days)
    Image,
    /// CDN assets (30 days)
    Cdn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_ttl_values() {
        assert_eq!(CACHE_TTL_VERY_SHORT, 300);
        assert_eq!(CACHE_TTL_SHORT, 900);
        assert_eq!(CACHE_TTL_MEDIUM, 3600);
        assert_eq!(CACHE_TTL_LONG, 21600);
        assert_eq!(CACHE_TTL_VERY_LONG, 86400);
        assert_eq!(CACHE_TTL_IMAGE, 604800);
        assert_eq!(CACHE_TTL_CDN, 2592000);
    }

    #[test]
    fn test_get_ttl_for() {
        assert_eq!(get_ttl_for(CacheType::RealTime), 300);
        assert_eq!(get_ttl_for(CacheType::Regular), 3600);
        assert_eq!(get_ttl_for(CacheType::Image), 604800);
    }
}
