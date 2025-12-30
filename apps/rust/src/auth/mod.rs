//! Authorization and Role-Based Access Control (RBAC).
//!
//! Provides policy-based authorization with roles and permissions.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::auth::{Gate, Permission, Role};
//!
//! // Define permissions
//! let mut gate = Gate::new();
//! gate.define("posts.create", |user| user.has_role("admin") || user.has_role("editor"));
//! gate.define("posts.delete", |user| user.has_role("admin"));
//!
//! // Check authorization
//! if gate.allows(&user, "posts.create") {
//!     // User can create posts
//! }
//! ```

pub mod api_key;
pub mod gate;
pub mod oauth;
pub mod password_reset;
pub mod permission;
pub mod remember_me;
pub mod role;
pub mod totp;

pub use api_key::{ApiKey, ApiKeyManager};
pub use gate::{Authorizable, Gate, Policy, SimpleUser};
pub use oauth::{DiscordProvider, GitHubProvider, GoogleProvider, OAuthProvider, OAuthUser};
pub use password_reset::PasswordReset;
pub use permission::Permission;
pub use remember_me::RememberMe;
pub use role::Role;
pub use totp::{generate_backup_codes, Totp, TotpConfig};
