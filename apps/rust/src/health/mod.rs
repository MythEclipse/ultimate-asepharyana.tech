//! Health check endpoints.
//!
//! Provides standardized health and readiness endpoints for
//! load balancers and orchestration systems.

pub mod endpoints;

pub use endpoints::{health_check, readiness_check, HealthStatus};
