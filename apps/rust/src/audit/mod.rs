//! Audit logging for entity changes.
//!
//! Provides automatic audit trail for create, update, and delete operations.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::audit::{AuditLogger, AuditAction};
//!
//! let audit = AuditLogger::new(redis_pool);
//!
//! // Log an action
//! audit.log(AuditAction::Create, "user", "123", Some("admin_user"), None).await?;
//!
//! // Log with changes
//! audit.log_with_changes(
//!     AuditAction::Update,
//!     "user",
//!     "123",
//!     Some("admin_user"),
//!     &old_user,
//!     &new_user,
//! ).await?;
//!
//! // Query audit history
//! let history = audit.history("user", "123", 10).await?;
//! ```

pub mod logger;

pub use logger::{AuditAction, AuditEntry, AuditLogger};
