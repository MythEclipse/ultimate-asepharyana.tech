//! Graceful shutdown utilities.

pub mod shutdown;

pub use shutdown::{shutdown_signal, GracefulShutdown};
