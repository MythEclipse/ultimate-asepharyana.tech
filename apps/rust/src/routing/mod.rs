//! Routing utilities - versioning, route helpers.

pub mod versioning;

pub use versioning::{extract_version, versioned_routes, ApiVersion, VersionedApi};
