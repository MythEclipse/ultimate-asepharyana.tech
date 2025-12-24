//! Background job system for async task processing.
//!
//! This module provides a Redis-backed job queue for executing
//! long-running or deferred tasks outside of the request lifecycle.

pub mod queue;
pub mod worker;

pub use queue::{Job, JobDispatcher, JobStatus};
