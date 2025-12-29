//! Multi-channel notifications system.
//!
//! Provides notifications beyond email - database, Slack, Discord, etc.
//!
//! # Example
//!
//! ```ignore
//! use rust::notifications::{Notification, NotificationChannel, Notifier};
//!
//! let notifier = Notifier::new(redis_pool);
//!
//! // Send via multiple channels
//! notifier.send(
//!     &user_id,
//!     Notification::new("New message")
//!         .via(NotificationChannel::Database)
//!         .via(NotificationChannel::Slack),
//! ).await?;
//! ```

pub mod channels;
pub mod notifier;

pub use channels::{DiscordConfig, NotificationChannel, SlackConfig};
pub use notifier::{Notification, NotificationError, Notifier};
