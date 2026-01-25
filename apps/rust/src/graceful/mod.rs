//! Graceful shutdown utilities.

pub mod cleanup;
pub mod shutdown;

pub use cleanup::{wait_for_shutdown_and_cleanup, ShutdownCoordinator, ShutdownHandle};
pub use shutdown::{shutdown_signal, GracefulShutdown};
