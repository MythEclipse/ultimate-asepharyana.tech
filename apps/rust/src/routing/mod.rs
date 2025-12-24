//! Routing utilities - versioning, route helpers.

pub mod versioning;

pub use versioning::{VersionedApi, ApiVersion, versioned_routes, extract_version};
