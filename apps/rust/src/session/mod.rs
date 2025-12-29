//! Session management for the framework.
//!
//! Provides a Redis-backed session system with middleware integration.
//!
//! # Example
//!
//! ```ignore
//! use rust::session::{SessionStore, SessionMiddleware, Session};
//!
//! // In your route handler
//! async fn handler(session: Session) -> impl IntoResponse {
//!     // Set session data
//!     session.set("user_id", "123").await;
//!     
//!     // Get session data
//!     let user_id: Option<String> = session.get("user_id").await;
//!     
//!     // Flash message (available for next request only)
//!     session.flash("success", "Login successful!").await;
//! }
//! ```

pub mod middleware;
pub mod store;

pub use middleware::{Session, SessionLayer};
pub use store::SessionStore;
