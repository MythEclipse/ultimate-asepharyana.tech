//! Helper utilities for easier development.
//!
//! Provides common patterns, response helpers, and utilities.

pub mod prelude;
pub mod response;
pub mod pagination;
pub mod handler;
pub mod string;
pub mod datetime;
pub mod crypto;
pub mod file;
pub mod retry;
pub mod cache;
pub mod scraping;
pub mod errors;

// Re-export all helpers for convenience
pub use prelude::*;
pub use response::*;
pub use pagination::*;
pub use errors::{HandlerError, internal_error, internal_err, bad_request, not_found, 
    unauthorized, forbidden, redis_error, db_error, ResultExt};
pub use retry::{default_backoff, quick_backoff, slow_backoff, custom_backoff, transient, permanent, retry};
pub use cache::{Cache, cache_key, cache_key_multi, DEFAULT_CACHE_TTL};
pub use scraping::{parse_html, selector, text, select_text, select_attr, 
    extract_slug, strip_tags, extract_number, Scraper};
pub use string::{slugify, truncate, initials, mask_email, random_string, random_code, is_valid_email, title_case};
pub use datetime::{now, timestamp, to_iso, to_human, parse_iso, add_days, add_hours, relative, is_past, is_future};
pub use crypto::{hash_password, verify_password, sha256, base64_encode, base64_decode, generate_token, generate_verification_code};
pub use file::{read_file, write_file, file_exists, create_dir, get_extension, format_file_size, mime_from_extension};
