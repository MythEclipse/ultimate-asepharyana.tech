//! Circuit breaker for external service calls.
//!
//! Prevents cascading failures by temporarily blocking requests
//! to failing services.

pub mod breaker;

pub use breaker::{CircuitBreaker, CircuitState};
