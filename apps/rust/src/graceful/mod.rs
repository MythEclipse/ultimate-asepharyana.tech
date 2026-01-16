//! Graceful shutdown utilities.

pub mod shutdown;
pub mod cleanup;

pub use shutdown::{shutdown_signal, GracefulShutdown};
pub use cleanup::{ShutdownCoordinator, ShutdownHandle, wait_for_shutdown_and_cleanup};
