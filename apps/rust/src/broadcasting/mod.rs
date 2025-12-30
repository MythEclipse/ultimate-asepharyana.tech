//! Broadcasting / Server-Sent Events (SSE).
//!
//! Real-time event broadcasting to clients.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::broadcasting::{Broadcaster, Event};
//!
//! let broadcaster = Broadcaster::new();
//!
//! // Subscribe to channel
//! let rx = broadcaster.subscribe("notifications");
//!
//! // Broadcast event
//! broadcaster.broadcast("notifications", Event::new("message", json!({"text": "Hello!"}))).await;
//!
//! // SSE endpoint
//! async fn sse_handler(broadcaster: Extension<Broadcaster>) -> Sse<impl Stream> {
//!     broadcaster.sse_stream("notifications")
//! }
//! ```

pub mod broadcaster;

pub use broadcaster::{Broadcaster, Channel, Event};
