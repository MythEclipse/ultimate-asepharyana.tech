//! Event system for pub/sub communication between modules.
//!
//! Provides a simple in-process event bus for decoupled communication.

pub mod bus;

pub use bus::{Event, EventBus, EventHandler};
