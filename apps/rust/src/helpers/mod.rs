//! Helper utilities for easier development.
//!
//! Comprehensive collection of updated utility modules for cleaner, more maintainable code.

// Submodules
pub mod core;
pub mod data;
pub mod dev;
pub mod infra;
pub mod io;
pub mod web;
pub mod ryzen_cdn;

// Re-exports for convenience (backward compatibility)

// Core
pub use core::api_response;
pub use core::errors;
pub use core::handler;
pub use core::pagination;
pub use core::prelude;
pub use core::response;

// Data
pub use data::collections;
pub use data::convert;
pub use data::crypto;
pub use data::datetime;
pub use data::json;
pub use data::numbers;
pub use data::string;
pub use data::text;

// IO
pub use io::cache;
pub use io::cache_tags;
pub use io::cache_ttl;
pub use io::file;
pub use io::retry;
pub use io::soft_delete;

// Web
pub use web::query;
pub use web::request;
pub use web::scraping;
pub use web::url;
pub use web::validation;
pub use web::http;

// Dev
pub use dev::async_utils;
pub use dev::logging;
pub use dev::memory;
pub use dev::performance;
pub use dev::result_ext;
pub use dev::serde_helpers;
pub use dev::testing;

// Infra
pub use infra::bulk;
pub use infra::console;
pub use infra::email_template;
pub use infra::encryption;
pub use infra::env;
pub use infra::form_request;
pub use infra::health_check;
pub use infra::import_export;
pub use infra::query_profiler;
pub use infra::resource;
pub use infra::searchable;
pub use infra::security;
pub use infra::signed_url;
pub use infra::sluggable;
pub use infra::state_machine;
pub use infra::tenant;
pub use infra::transaction;
pub use infra::uuid_utils;
pub use infra::versioning;
pub use infra::mailer;

// Ryzen CDN
pub use ryzen_cdn::*;


// ============================================================================
// Original Re-exports (preserved)
// ============================================================================

// Prelude (common imports)
pub use prelude::*;

// Response helpers
pub use pagination::*;
pub use response::*;

// Error helpers
pub use errors::{
    bad_request, db_error, forbidden, internal_err, internal_error, not_found, redis_error,
    unauthorized, HandlerError, ResultExt,
};

// Retry/Backoff
pub use retry::{
    custom_backoff, default_backoff, permanent, quick_backoff, retry, slow_backoff, transient,
};

// Caching
pub use cache::{cache_key, cache_key_multi, Cache, DEFAULT_CACHE_TTL};

// Scraping
pub use scraping::{
    attr_from, attr_from_or, extract_number, extract_slug, fetch_html_with_retry, parse_html,
    select_attr, select_text, selector, strip_tags, text, text_from, text_from_or, Scraper,
};

// Strings
pub use string::{
    initials, is_valid_email, mask_email, random_code, random_string, slugify, title_case, truncate,
};

// DateTime
pub use datetime::{
    add_days, add_hours, is_future, is_past, now, parse_iso, relative, timestamp, to_human, to_iso,
};

// Crypto
pub use crypto::{
    base64_decode, base64_encode, generate_token, generate_verification_code, hash_password,
    sha256, verify_password,
};

// Files
pub use file::{
    create_dir, file_exists, format_file_size, get_extension, mime_from_extension, read_file,
    write_file,
};

// Validation
pub use validation::{
    is_alphanumeric, is_email, is_phone, is_slug, is_strong_password, is_url, is_uuid, max_length,
    min_length, Validator,
};

// JSON
pub use json::{
    deep_merge, get_i64, get_path, get_str, is_empty, merge, parse, remove_nulls, stringify,
    stringify_pretty,
};

// URL
pub use url::{
    decode, encode, extract_domain, is_absolute, join_paths, make_absolute, parse_query, UrlBuilder,
};

// Logging
pub use logging::{log_and_map, log_error, log_request, PerfLogger, TimedOperation};

// Collections
pub use collections::{
    all, any, chunk, count, find, find_index, flatten, frequencies, group_by, partition, reverse,
    skip, sum, take, unique, zip,
};

// Numbers
pub use numbers::{
    clamp, format_bytes, format_currency, format_number, format_percent, is_even, is_odd, lerp,
    parse_f64, parse_i64, percentage, round_to, safe_div,
};

// Async
pub use async_utils::{
    join_all, join_all_limited, simple_retry, sleep, sleep_ms, sleep_secs, spawn, spawn_blocking,
    timeout_ms, timeout_secs, with_timeout, Debouncer,
};

// Serde helpers
pub use serde_helpers::{
    default_empty_string, default_empty_vec, default_false, default_true, default_zero,
};

// Text processing
pub use text::{
    capitalize, highlight, lorem_ipsum, normalize_whitespace, remove_accents, to_camel_case,
    to_constant_case, to_kebab_case, to_pascal_case, to_snake_case, truncate_words, word_count,
};

// Conversions
pub use convert::{
    bool_to_str, bytes_to_hex, empty_to_none, hex_to_bytes, i64_to_usize, ms_to_human,
    none_to_empty, parse_or, seconds_to_human, to_bool, try_parse,
};

// Result/Option extensions
pub use result_ext::{err, flatten_option, flatten_result, ok, some, OptionExt, ResultExt2};

// HTTP Request helpers
pub use request::{
    accepts_gzip, bearer_token, client_ip, content_type, header_value, is_form, is_json, origin,
    referer, request_id, user_agent,
};

// Environment
pub use env::{
    database_url, get_or as env_get_or, host, is_debug, is_development, is_production, load_dotenv,
    port, redis_url, require as env_require,
};

// Security
pub use security::{
    check_rate_limit, clear_rate_limit, generate_api_key, generate_csrf_token, generate_session_id,
    mask_sensitive, sanitize_filename, sanitize_html, sanitize_input, secure_compare,
};

// UUID
pub use uuid_utils::{
    is_valid as is_valid_uuid_format, medium_id, new_v4 as uuid_v4, new_v4_simple as uuid_simple,
    short_id,
};
