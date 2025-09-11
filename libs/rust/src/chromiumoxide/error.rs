//! Error module for Chromiumoxide operations

use thiserror::Error;
use chromiumoxide::error::CdpError;

/// Custom error type for Chromiumoxide operations
#[derive(Error, Debug)]
pub enum BrowserError {
  #[error("Browser startup error: {0}")] BrowserStartupError(String),
  #[error("Tab creation error: {0}")] TabCreationError(String),
  #[error("Navigation error: {0}")] NavigationError(String),
  #[error("Element not found: {0}")] ElementNotFound(String),
  #[error("Timeout error: {0}")] TimeoutError(String),
  #[error("Cloudflare challenge detected: {0}")] CloudflareChallenge(String),
  #[error("Retry limit exceeded: {0}")] RetryLimitExceeded(String),
  #[error("Generic browser error: {0}")] GenericError(String),
  #[error(transparent)] Chromiumoxide(#[from] CdpError),
}

/// Result type for Chromiumoxide operations
pub type BrowserResult<T> = Result<T, BrowserError>;
