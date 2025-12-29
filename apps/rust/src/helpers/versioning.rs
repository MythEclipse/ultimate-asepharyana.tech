//! Versioned API helpers.
//!
//! API version extraction and routing.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::versioning::{ApiVersion, extract_version};
//!
//! // From header: Accept: application/vnd.api+json; version=2
//! let version = extract_version(&headers);
//!
//! // From path: /api/v2/users
//! let version = ApiVersion::from_path("/api/v2/users");
//! ```

use axum::http::{header::ACCEPT, HeaderMap};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// API version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
}

impl ApiVersion {
    /// Create a new version.
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Create from major version only.
    pub const fn major_only(major: u32) -> Self {
        Self { major, minor: 0 }
    }

    /// Parse from string like "v2" or "2.1".
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().trim_start_matches('v').trim_start_matches('V');

        if let Some((major, minor)) = s.split_once('.') {
            Some(Self {
                major: major.parse().ok()?,
                minor: minor.parse().ok()?,
            })
        } else {
            Some(Self {
                major: s.parse().ok()?,
                minor: 0,
            })
        }
    }

    /// Extract from URL path like /api/v2/users.
    pub fn from_path(path: &str) -> Option<Self> {
        for segment in path.split('/') {
            if segment.starts_with('v') || segment.starts_with('V') {
                if let Some(v) = Self::parse(segment) {
                    return Some(v);
                }
            }
        }
        None
    }

    /// Extract from Accept header.
    pub fn from_accept_header(accept: &str) -> Option<Self> {
        // Format: application/vnd.api+json; version=2
        for part in accept.split(';') {
            let part = part.trim();
            if part.starts_with("version=") {
                let version = part.trim_start_matches("version=");
                return Self::parse(version);
            }
        }
        None
    }

    /// Check if version is at least the given version.
    pub fn at_least(&self, major: u32, minor: u32) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }

    /// Check if version is below the given version.
    pub fn below(&self, major: u32, minor: u32) -> bool {
        !self.at_least(major, minor)
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}

impl PartialOrd for ApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ApiVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => self.minor.cmp(&other.minor),
            other => other,
        }
    }
}

/// Extract API version from request.
pub fn extract_version(headers: &HeaderMap) -> ApiVersion {
    // Try Accept header first
    if let Some(accept) = headers.get(ACCEPT).and_then(|h| h.to_str().ok()) {
        if let Some(v) = ApiVersion::from_accept_header(accept) {
            return v;
        }
    }

    // Try custom header
    if let Some(version) = headers.get("X-API-Version").and_then(|h| h.to_str().ok()) {
        if let Some(v) = ApiVersion::parse(version) {
            return v;
        }
    }

    ApiVersion::default()
}

/// Common API versions.
pub mod versions {
    use super::ApiVersion;

    pub const V1: ApiVersion = ApiVersion::new(1, 0);
    pub const V2: ApiVersion = ApiVersion::new(2, 0);
    pub const V3: ApiVersion = ApiVersion::new(3, 0);
}

/// Version constraint for routing.
#[derive(Debug, Clone)]
pub struct VersionConstraint {
    pub min: Option<ApiVersion>,
    pub max: Option<ApiVersion>,
}

impl VersionConstraint {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
        }
    }

    pub fn min(mut self, version: ApiVersion) -> Self {
        self.min = Some(version);
        self
    }

    pub fn max(mut self, version: ApiVersion) -> Self {
        self.max = Some(version);
        self
    }

    pub fn matches(&self, version: ApiVersion) -> bool {
        if let Some(min) = self.min {
            if version < min {
                return false;
            }
        }
        if let Some(max) = self.max {
            if version > max {
                return false;
            }
        }
        true
    }
}

impl Default for VersionConstraint {
    fn default() -> Self {
        Self::new()
    }
}
