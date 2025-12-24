//! WebSocket support with room management and message handling.
//!
//! Provides easy-to-use WebSocket utilities for real-time features.

pub mod handler;
pub mod room;
pub mod message;

pub use handler::{ws_handler, WsState};
pub use room::{Room, RoomManager};
pub use message::{WsMessage, WsEvent};
