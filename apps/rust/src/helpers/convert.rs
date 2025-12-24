//! Ultra-comprehensive type conversion utilities for Rust.
//!
//! This module provides safe and ergonomic conversions between ALL Rust types.
//! Designed to handle every possible conversion scenario.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ============================================================================
// BYTES & ENCODING CONVERSIONS
// ============================================================================

/// Convert bytes to hex string (lowercase).
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Convert bytes to hex string (uppercase).
pub fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

/// Convert hex string to bytes.
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

/// Convert bytes to base64.
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
}

/// Convert base64 to bytes.
pub fn base64_to_bytes(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s)
}

/// Convert bytes to binary string.
pub fn bytes_to_binary(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert bytes to octal string.
pub fn bytes_to_octal(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:03o}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert u8 to binary string.
pub fn u8_to_binary(n: u8) -> String {
    format!("{:08b}", n)
}

/// Convert u16 to binary string.
pub fn u16_to_binary(n: u16) -> String {
    format!("{:016b}", n)
}

/// Convert u32 to binary string.
pub fn u32_to_binary(n: u32) -> String {
    format!("{:032b}", n)
}

/// Convert u64 to binary string.
pub fn u64_to_binary(n: u64) -> String {
    format!("{:064b}", n)
}

/// Convert binary string to u64.
pub fn binary_to_u64(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(&s.replace(" ", ""), 2)
}

// ============================================================================
// BOOLEAN CONVERSIONS
// ============================================================================

/// Convert string to bool (flexible parsing).
pub fn to_bool(s: &str) -> bool {
    matches!(
        s.to_lowercase().trim(),
        "true" | "1" | "yes" | "on" | "enabled" | "t" | "y" | "ok" | "active"
    )
}

/// Convert to bool with Option for invalid input.
pub fn try_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().trim() {
        "true" | "1" | "yes" | "on" | "enabled" | "t" | "y" | "ok" | "active" => Some(true),
        "false" | "0" | "no" | "off" | "disabled" | "f" | "n" | "inactive" => Some(false),
        _ => None,
    }
}

/// Convert bool to string.
pub fn bool_to_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

/// Convert bool to yes/no.
pub fn bool_to_yes_no(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}

/// Convert bool to on/off.
pub fn bool_to_on_off(b: bool) -> &'static str {
    if b {
        "on"
    } else {
        "off"
    }
}

/// Convert bool to enabled/disabled.
pub fn bool_to_enabled(b: bool) -> &'static str {
    if b {
        "enabled"
    } else {
        "disabled"
    }
}

/// Convert bool to active/inactive.
pub fn bool_to_active(b: bool) -> &'static str {
    if b {
        "active"
    } else {
        "inactive"
    }
}

/// Convert bool to 0/1 i32.
pub fn bool_to_int(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}

/// Convert bool to 0/1 i64.
pub fn bool_to_i64(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

/// Convert i32 to bool (0 = false, other = true).
pub fn int_to_bool(n: i32) -> bool {
    n != 0
}

// ============================================================================
// INTEGER CONVERSIONS - i8
// ============================================================================

/// Safe i8 to u8.
pub fn i8_to_u8(n: i8) -> u8 {
    n.max(0) as u8
}

/// Safe i8 to i16.
pub fn i8_to_i16(n: i8) -> i16 {
    n as i16
}

/// Safe i8 to i32.
pub fn i8_to_i32(n: i8) -> i32 {
    n as i32
}

/// Safe i8 to i64.
pub fn i8_to_i64(n: i8) -> i64 {
    n as i64
}

/// Safe i8 to i128.
pub fn i8_to_i128(n: i8) -> i128 {
    n as i128
}

/// Safe i8 to f32.
pub fn i8_to_f32(n: i8) -> f32 {
    n as f32
}

/// Safe i8 to f64.
pub fn i8_to_f64(n: i8) -> f64 {
    n as f64
}

// ============================================================================
// INTEGER CONVERSIONS - i16
// ============================================================================

/// Safe i16 to i8 (saturates).
pub fn i16_to_i8(n: i16) -> i8 {
    n.clamp(i8::MIN as i16, i8::MAX as i16) as i8
}

/// Safe i16 to u8 (saturates).
pub fn i16_to_u8(n: i16) -> u8 {
    n.clamp(0, u8::MAX as i16) as u8
}

/// Safe i16 to u16.
pub fn i16_to_u16(n: i16) -> u16 {
    n.max(0) as u16
}

/// Safe i16 to i32.
pub fn i16_to_i32(n: i16) -> i32 {
    n as i32
}

/// Safe i16 to i64.
pub fn i16_to_i64(n: i16) -> i64 {
    n as i64
}

/// Safe i16 to i128.
pub fn i16_to_i128(n: i16) -> i128 {
    n as i128
}

// ============================================================================
// INTEGER CONVERSIONS - i32
// ============================================================================

/// Safe i32 to i8 (saturates).
pub fn i32_to_i8(n: i32) -> i8 {
    n.clamp(i8::MIN as i32, i8::MAX as i32) as i8
}

/// Safe i32 to i16 (saturates).
pub fn i32_to_i16(n: i32) -> i16 {
    n.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Safe i32 to u8 (saturates).
pub fn i32_to_u8(n: i32) -> u8 {
    n.clamp(0, u8::MAX as i32) as u8
}

/// Safe i32 to u16 (saturates).
pub fn i32_to_u16(n: i32) -> u16 {
    n.clamp(0, u16::MAX as i32) as u16
}

/// Safe i32 to u32.
pub fn i32_to_u32(n: i32) -> u32 {
    n.max(0) as u32
}

/// Safe i32 to i64.
pub fn i32_to_i64(n: i32) -> i64 {
    n as i64
}

/// Safe i32 to i128.
pub fn i32_to_i128(n: i32) -> i128 {
    n as i128
}

/// Safe i32 to usize.
pub fn i32_to_usize(n: i32) -> usize {
    n.max(0) as usize
}

/// Safe i32 to f32.
pub fn i32_to_f32(n: i32) -> f32 {
    n as f32
}

/// Safe i32 to f64.
pub fn i32_to_f64(n: i32) -> f64 {
    n as f64
}

// ============================================================================
// INTEGER CONVERSIONS - i64
// ============================================================================

/// Safe i64 to i8 (saturates).
pub fn i64_to_i8(n: i64) -> i8 {
    n.clamp(i8::MIN as i64, i8::MAX as i64) as i8
}

/// Safe i64 to i16 (saturates).
pub fn i64_to_i16(n: i64) -> i16 {
    n.clamp(i16::MIN as i64, i16::MAX as i64) as i16
}

/// Safe i64 to i32 (saturates).
pub fn i64_to_i32(n: i64) -> i32 {
    n.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// Safe i64 to u8 (saturates).
pub fn i64_to_u8(n: i64) -> u8 {
    n.clamp(0, u8::MAX as i64) as u8
}

/// Safe i64 to u16 (saturates).
pub fn i64_to_u16(n: i64) -> u16 {
    n.clamp(0, u16::MAX as i64) as u16
}

/// Safe i64 to u32 (saturates).
pub fn i64_to_u32(n: i64) -> u32 {
    n.clamp(0, u32::MAX as i64) as u32
}

/// Safe i64 to u64.
pub fn i64_to_u64(n: i64) -> u64 {
    n.max(0) as u64
}

/// Safe i64 to usize.
pub fn i64_to_usize(n: i64) -> usize {
    n.max(0) as usize
}

/// Safe i64 to i128.
pub fn i64_to_i128(n: i64) -> i128 {
    n as i128
}

/// Safe i64 to u128.
pub fn i64_to_u128(n: i64) -> u128 {
    n.max(0) as u128
}

/// Safe i64 to f32.
pub fn i64_to_f32(n: i64) -> f32 {
    n as f32
}

/// Safe i64 to f64.
pub fn i64_to_f64(n: i64) -> f64 {
    n as f64
}

// ============================================================================
// INTEGER CONVERSIONS - i128
// ============================================================================

/// Safe i128 to i8 (saturates).
pub fn i128_to_i8(n: i128) -> i8 {
    n.clamp(i8::MIN as i128, i8::MAX as i128) as i8
}

/// Safe i128 to i16 (saturates).
pub fn i128_to_i16(n: i128) -> i16 {
    n.clamp(i16::MIN as i128, i16::MAX as i128) as i16
}

/// Safe i128 to i32 (saturates).
pub fn i128_to_i32(n: i128) -> i32 {
    n.clamp(i32::MIN as i128, i32::MAX as i128) as i32
}

/// Safe i128 to i64 (saturates).
pub fn i128_to_i64(n: i128) -> i64 {
    n.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

/// Safe i128 to u128.
pub fn i128_to_u128(n: i128) -> u128 {
    n.max(0) as u128
}

/// Safe i128 to usize.
pub fn i128_to_usize(n: i128) -> usize {
    n.clamp(0, usize::MAX as i128) as usize
}

// ============================================================================
// INTEGER CONVERSIONS - u8
// ============================================================================

/// u8 to i8 (may overflow to negative).
pub fn u8_to_i8_wrap(n: u8) -> i8 {
    n as i8
}

/// u8 to i8 (saturates at i8::MAX).
pub fn u8_to_i8_sat(n: u8) -> i8 {
    n.min(i8::MAX as u8) as i8
}

/// u8 to i16.
pub fn u8_to_i16(n: u8) -> i16 {
    n as i16
}

/// u8 to i32.
pub fn u8_to_i32(n: u8) -> i32 {
    n as i32
}

/// u8 to i64.
pub fn u8_to_i64(n: u8) -> i64 {
    n as i64
}

/// u8 to u16.
pub fn u8_to_u16(n: u8) -> u16 {
    n as u16
}

/// u8 to u32.
pub fn u8_to_u32(n: u8) -> u32 {
    n as u32
}

/// u8 to u64.
pub fn u8_to_u64(n: u8) -> u64 {
    n as u64
}

/// u8 to usize.
pub fn u8_to_usize(n: u8) -> usize {
    n as usize
}

/// u8 to f32.
pub fn u8_to_f32(n: u8) -> f32 {
    n as f32
}

/// u8 to f64.
pub fn u8_to_f64(n: u8) -> f64 {
    n as f64
}

// ============================================================================
// INTEGER CONVERSIONS - u16
// ============================================================================

/// u16 to u8 (saturates).
pub fn u16_to_u8(n: u16) -> u8 {
    n.min(u8::MAX as u16) as u8
}

/// u16 to i16 (saturates).
pub fn u16_to_i16(n: u16) -> i16 {
    n.min(i16::MAX as u16) as i16
}

/// u16 to i32.
pub fn u16_to_i32(n: u16) -> i32 {
    n as i32
}

/// u16 to i64.
pub fn u16_to_i64(n: u16) -> i64 {
    n as i64
}

/// u16 to u32.
pub fn u16_to_u32(n: u16) -> u32 {
    n as u32
}

/// u16 to u64.
pub fn u16_to_u64(n: u16) -> u64 {
    n as u64
}

/// u16 to usize.
pub fn u16_to_usize(n: u16) -> usize {
    n as usize
}

// ============================================================================
// INTEGER CONVERSIONS - u32
// ============================================================================

/// u32 to u8 (saturates).
pub fn u32_to_u8(n: u32) -> u8 {
    n.min(u8::MAX as u32) as u8
}

/// u32 to u16 (saturates).
pub fn u32_to_u16(n: u32) -> u16 {
    n.min(u16::MAX as u32) as u16
}

/// u32 to i32 (saturates).
pub fn u32_to_i32(n: u32) -> i32 {
    n.min(i32::MAX as u32) as i32
}

/// u32 to i64.
pub fn u32_to_i64(n: u32) -> i64 {
    n as i64
}

/// u32 to u64.
pub fn u32_to_u64(n: u32) -> u64 {
    n as u64
}

/// u32 to usize.
pub fn u32_to_usize(n: u32) -> usize {
    n as usize
}

/// u32 to f32.
pub fn u32_to_f32(n: u32) -> f32 {
    n as f32
}

/// u32 to f64.
pub fn u32_to_f64(n: u32) -> f64 {
    n as f64
}

// ============================================================================
// INTEGER CONVERSIONS - u64
// ============================================================================

/// u64 to u8 (saturates).
pub fn u64_to_u8(n: u64) -> u8 {
    n.min(u8::MAX as u64) as u8
}

/// u64 to u16 (saturates).
pub fn u64_to_u16(n: u64) -> u16 {
    n.min(u16::MAX as u64) as u16
}

/// u64 to u32 (saturates).
pub fn u64_to_u32(n: u64) -> u32 {
    n.min(u32::MAX as u64) as u32
}

/// u64 to i64 (saturates).
pub fn u64_to_i64(n: u64) -> i64 {
    n.min(i64::MAX as u64) as i64
}

/// u64 to i128.
pub fn u64_to_i128(n: u64) -> i128 {
    n as i128
}

/// u64 to u128.
pub fn u64_to_u128(n: u64) -> u128 {
    n as u128
}

/// u64 to usize (may truncate on 32-bit).
pub fn u64_to_usize(n: u64) -> usize {
    n as usize
}

/// u64 to f64.
pub fn u64_to_f64(n: u64) -> f64 {
    n as f64
}

// ============================================================================
// INTEGER CONVERSIONS - u128
// ============================================================================

/// u128 to u64 (saturates).
pub fn u128_to_u64(n: u128) -> u64 {
    n.min(u64::MAX as u128) as u64
}

/// u128 to i128 (saturates).
pub fn u128_to_i128(n: u128) -> i128 {
    n.min(i128::MAX as u128) as i128
}

/// u128 to usize (saturates).
pub fn u128_to_usize(n: u128) -> usize {
    n.min(usize::MAX as u128) as usize
}

// ============================================================================
// INTEGER CONVERSIONS - usize/isize
// ============================================================================

/// usize to i32 (saturates).
pub fn usize_to_i32(n: usize) -> i32 {
    n.min(i32::MAX as usize) as i32
}

/// usize to i64.
pub fn usize_to_i64(n: usize) -> i64 {
    n as i64
}

/// usize to u32 (saturates on 64-bit).
pub fn usize_to_u32(n: usize) -> u32 {
    n.min(u32::MAX as usize) as u32
}

/// usize to u64.
pub fn usize_to_u64(n: usize) -> u64 {
    n as u64
}

/// isize to i32 (saturates).
pub fn isize_to_i32(n: isize) -> i32 {
    n.clamp(i32::MIN as isize, i32::MAX as isize) as i32
}

/// isize to i64.
pub fn isize_to_i64(n: isize) -> i64 {
    n as i64
}

/// isize to usize.
pub fn isize_to_usize(n: isize) -> usize {
    n.max(0) as usize
}

// ============================================================================
// FLOAT CONVERSIONS
// ============================================================================

/// f64 to f32 (may lose precision).
pub fn f64_to_f32(n: f64) -> f32 {
    n as f32
}

/// f32 to f64.
pub fn f32_to_f64(n: f32) -> f64 {
    n as f64
}

/// f64 to i64 (truncates).
pub fn f64_to_i64(n: f64) -> i64 {
    n.clamp(i64::MIN as f64, i64::MAX as f64) as i64
}

/// f64 to i32 (truncates).
pub fn f64_to_i32(n: f64) -> i32 {
    n.clamp(i32::MIN as f64, i32::MAX as f64) as i32
}

/// f64 to u64 (truncates).
pub fn f64_to_u64(n: f64) -> u64 {
    n.clamp(0.0, u64::MAX as f64) as u64
}

/// f64 to u32 (truncates).
pub fn f64_to_u32(n: f64) -> u32 {
    n.clamp(0.0, u32::MAX as f64) as u32
}

/// f32 to i32 (truncates).
pub fn f32_to_i32(n: f32) -> i32 {
    n.clamp(i32::MIN as f32, i32::MAX as f32) as i32
}

/// Round f64 to n decimal places.
pub fn round_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).round() / factor
}

/// Round f32 to n decimal places.
pub fn round_f32(n: f32, decimals: u32) -> f32 {
    let factor = 10_f32.powi(decimals as i32);
    (n * factor).round() / factor
}

/// Truncate f64 to n decimal places.
pub fn trunc_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).trunc() / factor
}

/// Ceil f64 to n decimal places.
pub fn ceil_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).ceil() / factor
}

/// Floor f64 to n decimal places.
pub fn floor_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).floor() / factor
}

/// Check if f64 is essentially zero.
pub fn is_zero(n: f64, epsilon: f64) -> bool {
    n.abs() < epsilon
}

/// Compare two f64 for approximate equality.
pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Check if f64 is NaN.
pub fn is_nan(n: f64) -> bool {
    n.is_nan()
}

/// Check if f64 is infinite.
pub fn is_infinite(n: f64) -> bool {
    n.is_infinite()
}

/// Check if f64 is finite.
pub fn is_finite(n: f64) -> bool {
    n.is_finite()
}

/// Convert NaN to 0.
pub fn nan_to_zero(n: f64) -> f64 {
    if n.is_nan() {
        0.0
    } else {
        n
    }
}

/// Convert NaN to default.
pub fn nan_to_default(n: f64, default: f64) -> f64 {
    if n.is_nan() {
        default
    } else {
        n
    }
}

// ============================================================================
// STRING PARSING
// ============================================================================

/// Parse with default value.
pub fn parse_or<T: FromStr>(s: &str, default: T) -> T {
    s.parse().unwrap_or(default)
}

/// Try parse with error context.
pub fn try_parse<T: FromStr>(s: &str, name: &str) -> Result<T, String>
where
    T::Err: Display,
{
    s.parse()
        .map_err(|e| format!("Failed to parse {}: {}", name, e))
}

/// Parse i8 with default.
pub fn parse_i8(s: &str, default: i8) -> i8 {
    s.parse().unwrap_or(default)
}

/// Parse i16 with default.
pub fn parse_i16(s: &str, default: i16) -> i16 {
    s.parse().unwrap_or(default)
}

/// Parse i32 with default.
pub fn parse_i32(s: &str, default: i32) -> i32 {
    s.parse().unwrap_or(default)
}

/// Parse i64 with default.
pub fn parse_i64(s: &str, default: i64) -> i64 {
    s.parse().unwrap_or(default)
}

/// Parse i128 with default.
pub fn parse_i128(s: &str, default: i128) -> i128 {
    s.parse().unwrap_or(default)
}

/// Parse u8 with default.
pub fn parse_u8(s: &str, default: u8) -> u8 {
    s.parse().unwrap_or(default)
}

/// Parse u16 with default.
pub fn parse_u16(s: &str, default: u16) -> u16 {
    s.parse().unwrap_or(default)
}

/// Parse u32 with default.
pub fn parse_u32(s: &str, default: u32) -> u32 {
    s.parse().unwrap_or(default)
}

/// Parse u64 with default.
pub fn parse_u64(s: &str, default: u64) -> u64 {
    s.parse().unwrap_or(default)
}

/// Parse u128 with default.
pub fn parse_u128(s: &str, default: u128) -> u128 {
    s.parse().unwrap_or(default)
}

/// Parse f32 with default.
pub fn parse_f32(s: &str, default: f32) -> f32 {
    s.parse().unwrap_or(default)
}

/// Parse f64 with default.
pub fn parse_f64(s: &str, default: f64) -> f64 {
    s.parse().unwrap_or(default)
}

/// Parse usize with default.
pub fn parse_usize(s: &str, default: usize) -> usize {
    s.parse().unwrap_or(default)
}

/// Parse isize with default.
pub fn parse_isize(s: &str, default: isize) -> isize {
    s.parse().unwrap_or(default)
}

// ============================================================================
// DURATION / TIME CONVERSIONS
// ============================================================================

/// Convert seconds to human readable duration.
pub fn seconds_to_human(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs < 86400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else if secs < 604800 {
        format!("{}d {}h", secs / 86400, (secs % 86400) / 3600)
    } else if secs < 2592000 {
        format!("{}w {}d", secs / 604800, (secs % 604800) / 86400)
    } else if secs < 31536000 {
        format!("{}mo {}d", secs / 2592000, (secs % 2592000) / 86400)
    } else {
        format!("{}y {}mo", secs / 31536000, (secs % 31536000) / 2592000)
    }
}

/// Convert seconds to compact human readable.
pub fn seconds_to_compact(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h", secs / 3600)
    } else if secs < 604800 {
        format!("{}d", secs / 86400)
    } else if secs < 2592000 {
        format!("{}w", secs / 604800)
    } else if secs < 31536000 {
        format!("{}mo", secs / 2592000)
    } else {
        format!("{}y", secs / 31536000)
    }
}

/// Convert milliseconds to human readable.
pub fn ms_to_human(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        seconds_to_human(ms / 1000)
    }
}

/// Convert microseconds to human readable.
pub fn us_to_human(us: u64) -> String {
    if us < 1000 {
        format!("{}μs", us)
    } else if us < 1_000_000 {
        format!("{:.2}ms", us as f64 / 1000.0)
    } else {
        seconds_to_human(us / 1_000_000)
    }
}

/// Convert nanoseconds to human readable.
pub fn ns_to_human(ns: u64) -> String {
    if ns < 1_000 {
        format!("{}ns", ns)
    } else if ns < 1_000_000 {
        format!("{:.2}μs", ns as f64 / 1_000.0)
    } else if ns < 1_000_000_000 {
        format!("{:.2}ms", ns as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", ns as f64 / 1_000_000_000.0)
    }
}

/// Convert seconds to Duration.
pub fn secs_to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

/// Convert milliseconds to Duration.
pub fn ms_to_duration(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

/// Convert microseconds to Duration.
pub fn us_to_duration(us: u64) -> Duration {
    Duration::from_micros(us)
}

/// Convert nanoseconds to Duration.
pub fn ns_to_duration(ns: u64) -> Duration {
    Duration::from_nanos(ns)
}

/// Convert Duration to seconds.
pub fn duration_to_secs(d: Duration) -> u64 {
    d.as_secs()
}

/// Convert Duration to milliseconds.
pub fn duration_to_ms(d: Duration) -> u128 {
    d.as_millis()
}

/// Convert Duration to microseconds.
pub fn duration_to_us(d: Duration) -> u128 {
    d.as_micros()
}

/// Convert Duration to nanoseconds.
pub fn duration_to_ns(d: Duration) -> u128 {
    d.as_nanos()
}

/// Convert Duration to f64 seconds.
pub fn duration_to_secs_f64(d: Duration) -> f64 {
    d.as_secs_f64()
}

/// Convert f64 seconds to Duration.
pub fn secs_f64_to_duration(secs: f64) -> Duration {
    Duration::from_secs_f64(secs.max(0.0))
}

/// Get SystemTime as Unix timestamp (seconds).
pub fn system_time_to_unix(t: SystemTime) -> u64 {
    t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

/// Get SystemTime as Unix timestamp (milliseconds).
pub fn system_time_to_unix_ms(t: SystemTime) -> u128 {
    t.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()
}

/// Convert Unix timestamp to SystemTime.
pub fn unix_to_system_time(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}

/// Convert Unix milliseconds to SystemTime.
pub fn unix_ms_to_system_time(ms: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(ms)
}

// ============================================================================
// STRING / OPTION CONVERSIONS
// ============================================================================

/// Convert string to Option (None for empty).
pub fn empty_to_none(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

/// Convert None to empty string.
pub fn none_to_empty(opt: Option<String>) -> String {
    opt.unwrap_or_default()
}

/// Convert Option<&str> to Option<String>.
pub fn str_to_string(opt: Option<&str>) -> Option<String> {
    opt.map(|s| s.to_string())
}

/// Convert String to Option (None if empty).
pub fn string_to_option(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Trim and convert to Option (None if whitespace only).
pub fn trim_to_option(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Convert &str to String.
pub fn str_to_owned(s: &str) -> String {
    s.to_string()
}

/// Convert String to &str (returns empty if none).
pub fn string_to_str(s: &Option<String>) -> &str {
    s.as_deref().unwrap_or("")
}

// ============================================================================
// PATH CONVERSIONS
// ============================================================================

/// Convert &str to PathBuf.
pub fn str_to_path(s: &str) -> PathBuf {
    PathBuf::from(s)
}

/// Convert String to PathBuf.
pub fn string_to_path(s: String) -> PathBuf {
    PathBuf::from(s)
}

/// Convert PathBuf to String (lossy).
pub fn path_to_string(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// Convert PathBuf to Option<String> (None if not valid UTF-8).
pub fn path_to_string_strict(p: &Path) -> Option<String> {
    p.to_str().map(String::from)
}

/// Convert &str to &Path.
pub fn str_to_path_ref(s: &str) -> &Path {
    Path::new(s)
}

/// Convert OsStr to String (lossy).
pub fn os_str_to_string(s: &OsStr) -> String {
    s.to_string_lossy().to_string()
}

/// Convert OsString to String (lossy).
pub fn os_string_to_string(s: OsString) -> String {
    s.to_string_lossy().to_string()
}

/// Convert String to OsString.
pub fn string_to_os_string(s: String) -> OsString {
    OsString::from(s)
}

/// Convert &str to &OsStr.
pub fn str_to_os_str(s: &str) -> &OsStr {
    OsStr::new(s)
}

/// Get file extension as String.
pub fn path_extension(p: &Path) -> Option<String> {
    p.extension().and_then(|e| e.to_str()).map(String::from)
}

/// Get file name as String.
pub fn path_filename(p: &Path) -> Option<String> {
    p.file_name().and_then(|n| n.to_str()).map(String::from)
}

/// Get parent directory as PathBuf.
pub fn path_parent(p: &Path) -> Option<PathBuf> {
    p.parent().map(PathBuf::from)
}

// ============================================================================
// SMART POINTER CONVERSIONS
// ============================================================================

/// Wrap value in Box.
pub fn to_box<T>(value: T) -> Box<T> {
    Box::new(value)
}

/// Wrap value in Rc.
pub fn to_rc<T>(value: T) -> Rc<T> {
    Rc::new(value)
}

/// Wrap value in Arc.
pub fn to_arc<T>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Convert Box<T> to T (unbox).
pub fn unbox<T>(boxed: Box<T>) -> T {
    *boxed
}

/// Clone from Rc<T>.
pub fn rc_to_owned<T: Clone>(rc: &Rc<T>) -> T {
    (**rc).clone()
}

/// Clone from Arc<T>.
pub fn arc_to_owned<T: Clone>(arc: &Arc<T>) -> T {
    (**arc).clone()
}

/// Convert &str to Cow<str>.
pub fn str_to_cow(s: &str) -> Cow<'_, str> {
    Cow::Borrowed(s)
}

/// Convert String to Cow<str>.
pub fn string_to_cow(s: String) -> Cow<'static, str> {
    Cow::Owned(s)
}

/// Convert Cow<str> to String.
pub fn cow_to_string(cow: Cow<'_, str>) -> String {
    cow.into_owned()
}

/// Convert &[T] to Cow<[T]>.
pub fn slice_to_cow<T: Clone>(s: &[T]) -> Cow<'_, [T]> {
    Cow::Borrowed(s)
}

/// Convert Vec<T> to Cow<[T]>.
pub fn vec_to_cow<T: Clone>(v: Vec<T>) -> Cow<'static, [T]> {
    Cow::Owned(v)
}

// ============================================================================
// COLLECTION CONVERSIONS
// ============================================================================

/// Convert Vec<T> to Vec<U>.
pub fn map_vec<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    vec.into_iter().map(f).collect()
}

/// Convert &[T] to Vec<U>.
pub fn map_slice<T, U, F>(slice: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    slice.iter().map(f).collect()
}

/// Convert Vec<String> to Vec<&str>.
pub fn strings_to_strs(strings: &[String]) -> Vec<&str> {
    strings.iter().map(|s| s.as_str()).collect()
}

/// Convert Vec<&str> to Vec<String>.
pub fn strs_to_strings(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

/// Convert slice to fixed array.
pub fn slice_to_array<T: Copy, const N: usize>(slice: &[T]) -> Option<[T; N]> {
    slice.try_into().ok()
}

/// Convert Vec to VecDeque.
pub fn vec_to_deque<T>(v: Vec<T>) -> VecDeque<T> {
    VecDeque::from(v)
}

/// Convert VecDeque to Vec.
pub fn deque_to_vec<T>(d: VecDeque<T>) -> Vec<T> {
    d.into_iter().collect()
}

/// Convert Vec to LinkedList.
pub fn vec_to_linked_list<T>(v: Vec<T>) -> LinkedList<T> {
    v.into_iter().collect()
}

/// Convert LinkedList to Vec.
pub fn linked_list_to_vec<T>(l: LinkedList<T>) -> Vec<T> {
    l.into_iter().collect()
}

/// Convert Vec to HashSet.
pub fn vec_to_hashset<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.into_iter().collect()
}

/// Convert HashSet to Vec.
pub fn hashset_to_vec<T>(s: HashSet<T>) -> Vec<T> {
    s.into_iter().collect()
}

/// Convert Vec to BTreeSet.
pub fn vec_to_btreeset<T: Ord>(v: Vec<T>) -> BTreeSet<T> {
    v.into_iter().collect()
}

/// Convert BTreeSet to Vec.
pub fn btreeset_to_vec<T>(s: BTreeSet<T>) -> Vec<T> {
    s.into_iter().collect()
}

/// Convert Vec of tuples to HashMap.
pub fn vec_to_hashmap<K: Eq + Hash, V>(v: Vec<(K, V)>) -> HashMap<K, V> {
    v.into_iter().collect()
}

/// Convert HashMap to Vec of tuples.
pub fn hashmap_to_vec<K, V>(m: HashMap<K, V>) -> Vec<(K, V)> {
    m.into_iter().collect()
}

/// Convert Vec of tuples to BTreeMap.
pub fn vec_to_btreemap<K: Ord, V>(v: Vec<(K, V)>) -> BTreeMap<K, V> {
    v.into_iter().collect()
}

/// Convert BTreeMap to Vec of tuples.
pub fn btreemap_to_vec<K, V>(m: BTreeMap<K, V>) -> Vec<(K, V)> {
    m.into_iter().collect()
}

/// Convert HashMap to BTreeMap.
pub fn hashmap_to_btreemap<K: Ord + Hash, V>(m: HashMap<K, V>) -> BTreeMap<K, V> {
    m.into_iter().collect()
}

/// Convert BTreeMap to HashMap.
pub fn btreemap_to_hashmap<K: Eq + Hash, V>(m: BTreeMap<K, V>) -> HashMap<K, V> {
    m.into_iter().collect()
}

// ============================================================================
// CHAR CONVERSIONS
// ============================================================================

/// Convert char to u32 (unicode code point).
pub fn char_to_u32(c: char) -> u32 {
    c as u32
}

/// Convert u32 to char (unicode code point).
pub fn u32_to_char(n: u32) -> Option<char> {
    char::from_u32(n)
}

/// Convert char to ascii u8.
pub fn char_to_ascii(c: char) -> Option<u8> {
    if c.is_ascii() {
        Some(c as u8)
    } else {
        None
    }
}

/// Convert u8 to char.
pub fn u8_to_char(n: u8) -> char {
    n as char
}

/// Convert digit char to u8.
pub fn digit_to_u8(c: char) -> Option<u8> {
    c.to_digit(10).map(|d| d as u8)
}

/// Convert u8 to digit char.
pub fn u8_to_digit(n: u8) -> Option<char> {
    if n <= 9 {
        Some((b'0' + n) as char)
    } else {
        None
    }
}

/// Convert hex char to u8.
pub fn hex_char_to_u8(c: char) -> Option<u8> {
    c.to_digit(16).map(|d| d as u8)
}

/// Convert u8 to hex char (lowercase).
pub fn u8_to_hex_char(n: u8) -> Option<char> {
    if n < 16 {
        Some(if n < 10 {
            (b'0' + n) as char
        } else {
            (b'a' + n - 10) as char
        })
    } else {
        None
    }
}

/// Convert char to uppercase.
pub fn char_to_upper(c: char) -> char {
    c.to_ascii_uppercase()
}

/// Convert char to lowercase.
pub fn char_to_lower(c: char) -> char {
    c.to_ascii_lowercase()
}

// ============================================================================
// RESULT / OPTION CONVERSIONS
// ============================================================================

/// Convert Option<T> to Result<T, E>.
pub fn option_to_result<T, E>(opt: Option<T>, err: E) -> Result<T, E> {
    opt.ok_or(err)
}

/// Convert Option<T> to Result<T, String>.
pub fn option_to_result_str<T>(opt: Option<T>, msg: &str) -> Result<T, String> {
    opt.ok_or_else(|| msg.to_string())
}

/// Convert Result<T, E> to Option<T>.
pub fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
    res.ok()
}

/// Convert Result<T, E> to Option<E>.
pub fn result_to_err<T, E>(res: Result<T, E>) -> Option<E> {
    res.err()
}

/// Flatten nested Option.
pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.flatten()
}

/// Flatten nested Result.
pub fn flatten_result<T, E>(res: Result<Result<T, E>, E>) -> Result<T, E> {
    res.and_then(|r| r)
}

/// Transpose Option<Result<T, E>> to Result<Option<T>, E>.
pub fn transpose_option_result<T, E>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    opt.transpose()
}

/// Transpose Result<Option<T>, E> to Option<Result<T, E>>.
pub fn transpose_result_option<T, E>(res: Result<Option<T>, E>) -> Option<Result<T, E>> {
    res.transpose()
}

// ============================================================================
// NETWORK CONVERSIONS
// ============================================================================

/// Convert IPv4 string to u32.
pub fn ipv4_to_u32(ip: &str) -> Option<u32> {
    ip.parse::<Ipv4Addr>().ok().map(|addr| u32::from(addr))
}

/// Convert u32 to IPv4 string.
pub fn u32_to_ipv4(n: u32) -> String {
    Ipv4Addr::from(n).to_string()
}

/// Convert u32 to Ipv4Addr.
pub fn u32_to_ipv4_addr(n: u32) -> Ipv4Addr {
    Ipv4Addr::from(n)
}

/// Convert string to IpAddr.
pub fn str_to_ip_addr(s: &str) -> Option<IpAddr> {
    s.parse().ok()
}

/// Convert IpAddr to string.
pub fn ip_addr_to_string(ip: IpAddr) -> String {
    ip.to_string()
}

/// Convert string to SocketAddr.
pub fn str_to_socket_addr(s: &str) -> Option<SocketAddr> {
    s.parse().ok()
}

/// Convert SocketAddr to string.
pub fn socket_addr_to_string(addr: SocketAddr) -> String {
    addr.to_string()
}

/// Create SocketAddrV4 from IP and port.
pub fn ipv4_port_to_socket(ip: Ipv4Addr, port: u16) -> SocketAddrV4 {
    SocketAddrV4::new(ip, port)
}

/// Create SocketAddrV6 from IP and port.
pub fn ipv6_port_to_socket(ip: Ipv6Addr, port: u16) -> SocketAddrV6 {
    SocketAddrV6::new(ip, port, 0, 0)
}

/// Check if IP is loopback.
pub fn is_loopback(ip: &str) -> bool {
    ip.parse::<IpAddr>()
        .map(|a| a.is_loopback())
        .unwrap_or(false)
}

/// Check if IP is private (IPv4).
pub fn is_private_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>()
        .map(|a| a.is_private())
        .unwrap_or(false)
}

// ============================================================================
// COLOR CONVERSIONS
// ============================================================================

/// Convert hex color to RGB tuple.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

/// Convert hex color to RGBA tuple.
pub fn hex_to_rgba(hex: &str) -> Option<(u8, u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let (r, g, b) = hex_to_rgb(hex)?;
        return Some((r, g, b, 255));
    }
    if hex.len() != 8 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
    Some((r, g, b, a))
}

/// Convert RGB to hex color.
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Convert RGBA to hex color.
pub fn rgba_to_hex(r: u8, g: u8, b: u8, a: u8) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
}

/// Convert RGB to HSL.
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f64::EPSILON {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f64::EPSILON {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < f64::EPSILON {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    (h * 360.0, s * 100.0, l * 100.0)
}

// ============================================================================
// ENDIANNESS CONVERSIONS
// ============================================================================

/// Swap endianness of u16.
pub fn swap_endian_u16(n: u16) -> u16 {
    n.swap_bytes()
}

/// Swap endianness of u32.
pub fn swap_endian_u32(n: u32) -> u32 {
    n.swap_bytes()
}

/// Swap endianness of u64.
pub fn swap_endian_u64(n: u64) -> u64 {
    n.swap_bytes()
}

/// Swap endianness of u128.
pub fn swap_endian_u128(n: u128) -> u128 {
    n.swap_bytes()
}

/// Convert to big endian bytes.
pub fn u32_to_be_bytes(n: u32) -> [u8; 4] {
    n.to_be_bytes()
}

/// Convert to little endian bytes.
pub fn u32_to_le_bytes(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

/// Convert from big endian bytes.
pub fn be_bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

/// Convert from little endian bytes.
pub fn le_bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
}

/// Convert to big endian bytes.
pub fn u64_to_be_bytes(n: u64) -> [u8; 8] {
    n.to_be_bytes()
}

/// Convert to little endian bytes.
pub fn u64_to_le_bytes(n: u64) -> [u8; 8] {
    n.to_le_bytes()
}

/// Convert from big endian bytes.
pub fn be_bytes_to_u64(bytes: [u8; 8]) -> u64 {
    u64::from_be_bytes(bytes)
}

/// Convert from little endian bytes.
pub fn le_bytes_to_u64(bytes: [u8; 8]) -> u64 {
    u64::from_le_bytes(bytes)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_conversions() {
        assert_eq!(i64_to_u8(300), 255);
        assert_eq!(i64_to_u8(-5), 0);
        assert_eq!(i128_to_i64(i128::MAX), i64::MAX);
    }

    #[test]
    fn test_time_conversions() {
        assert_eq!(seconds_to_human(90), "1m 30s");
        assert_eq!(seconds_to_compact(86400), "1d");
    }

    #[test]
    fn test_color_conversions() {
        assert_eq!(hex_to_rgb("#ff8000"), Some((255, 128, 0)));
        assert_eq!(rgb_to_hex(255, 128, 0), "#ff8000");
    }

    #[test]
    fn test_network_conversions() {
        assert_eq!(ipv4_to_u32("192.168.1.1"), Some(0xC0A80101));
        assert_eq!(u32_to_ipv4(0xC0A80101), "192.168.1.1");
    }

    #[test]
    fn test_path_conversions() {
        let path = str_to_path("/test/path.txt");
        assert_eq!(path_extension(&path), Some("txt".to_string()));
        assert_eq!(path_filename(&path), Some("path.txt".to_string()));
    }
}
