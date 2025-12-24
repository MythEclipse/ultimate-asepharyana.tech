//! Custom extractors for the framework.
//!
//! This module provides enhanced extractors that add functionality
//! beyond what Axum provides out of the box.

pub mod validated;

pub use validated::ValidatedJson;
pub use validated::ValidatedQuery;
