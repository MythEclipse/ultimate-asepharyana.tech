//! WebSocket support with room management and message handling.
//!
//! Provides easy-to-use WebSocket utilities for real-time features.

pub mod handler;
pub mod message;
pub mod room;

pub use handler::{ws_handler, WsState};
pub use message::{WsEvent, WsMessage};
pub use room::{Room, RoomManager};
