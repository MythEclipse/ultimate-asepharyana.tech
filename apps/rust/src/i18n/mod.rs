//! Internationalization (i18n) support.
//!
//! Provides locale detection and translation loading from JSON files.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::i18n::{I18n, Locale};
//!
//! let i18n = I18n::load("./locales").await?;
//!
//! // Translate with default locale
//! let msg = i18n.t("auth.login_success");
//!
//! // Translate with specific locale
//! let msg = i18n.t_locale("auth.login_success", &Locale::from("id"));
//!
//! // With placeholders
//! let msg = i18n.t_with("welcome.greeting", &[("name", "John")]);
//! ```

pub mod translator;

pub use translator::{I18n, Locale, TranslationError};
