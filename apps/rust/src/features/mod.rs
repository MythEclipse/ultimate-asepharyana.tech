//! Feature flags for runtime feature toggling.
//!
//! Provides Redis-backed feature flags that can be enabled/disabled per user or globally.
//!
//! # Example
//!
//! ```ignore
//! use rust::features::{FeatureFlags, Feature};
//!
//! let flags = FeatureFlags::new(redis_pool);
//!
//! // Check if feature is enabled
//! if flags.is_enabled("new_dashboard").await {
//!     // Show new dashboard
//! }
//!
//! // Check for specific user
//! if flags.is_enabled_for("beta_feature", &user_id).await {
//!     // Show beta feature
//! }
//! ```

pub mod flags;

pub use flags::{Feature, FeatureFlags, FeatureStatus};
