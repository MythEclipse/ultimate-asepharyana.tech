//! Webhook handling for inbound webhooks.
//!
//! Provides signature verification and event routing for webhooks.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::webhooks::{WebhookHandler, WebhookEvent};
//!
//! let handler = WebhookHandler::new()
//!     .add_secret("stripe", "whsec_xxx")
//!     .add_secret("github", "secret123");
//!
//! // Verify and parse incoming webhook
//! let event = handler.verify_and_parse("stripe", &body, &signature).await?;
//! ```

pub mod handler;

pub use handler::{SignatureVerifier, WebhookError, WebhookEvent, WebhookHandler};
