//! Comprehensive type conversion utilities for Rust.
//!
//! This module provides safe and ergonomic conversions between Rust's many types.

use std::fmt::Display;
use std::str::FromStr;

// ============================================================================
// Bytes & Hex Conversions
// ============================================================================

/// Convert bytes to hex string.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Convert bytes to uppercase hex string.
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

// ============================================================================
// Boolean Conversions
// ============================================================================

/// Convert string to bool (flexible parsing).
pub fn to_bool(s: &str) -> bool {
    matches!(
        s.to_lowercase().trim(),
        "true" | "1" | "yes" | "on" | "enabled" | "t" | "y"
    )
}

/// Convert to bool with Option for invalid input.
pub fn try_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().trim() {
        "true" | "1" | "yes" | "on" | "enabled" | "t" | "y" => Some(true),
        "false" | "0" | "no" | "off" | "disabled" | "f" | "n" => Some(false),
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

/// Convert bool to 0/1.
pub fn bool_to_int(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}

// ============================================================================
// Integer Conversions (Safe with saturation)
// ============================================================================

/// Safe i64 to i32 (saturates at bounds).
pub fn i64_to_i32(n: i64) -> i32 {
    n.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// Safe i64 to i16 (saturates at bounds).
pub fn i64_to_i16(n: i64) -> i16 {
    n.clamp(i16::MIN as i64, i16::MAX as i64) as i16
}

/// Safe i64 to i8 (saturates at bounds).
pub fn i64_to_i8(n: i64) -> i8 {
    n.clamp(i8::MIN as i64, i8::MAX as i64) as i8
}

/// Safe i64 to usize (clamps to 0).
pub fn i64_to_usize(n: i64) -> usize {
    n.max(0) as usize
}

/// Safe i64 to u64 (clamps to 0).
pub fn i64_to_u64(n: i64) -> u64 {
    n.max(0) as u64
}

/// Safe i64 to u32 (saturates).
pub fn i64_to_u32(n: i64) -> u32 {
    n.clamp(0, u32::MAX as i64) as u32
}

/// Safe i64 to u16 (saturates).
pub fn i64_to_u16(n: i64) -> u16 {
    n.clamp(0, u16::MAX as i64) as u16
}

/// Safe i64 to u8 (saturates).
pub fn i64_to_u8(n: i64) -> u8 {
    n.clamp(0, u8::MAX as i64) as u8
}

/// Safe usize to i64.
pub fn usize_to_i64(n: usize) -> i64 {
    n.min(i64::MAX as usize) as i64
}

/// Safe usize to i32.
pub fn usize_to_i32(n: usize) -> i32 {
    n.min(i32::MAX as usize) as i32
}

/// Safe u64 to i64 (saturates).
pub fn u64_to_i64(n: u64) -> i64 {
    n.min(i64::MAX as u64) as i64
}

/// Safe f64 to i64.
pub fn f64_to_i64(n: f64) -> i64 {
    n.clamp(i64::MIN as f64, i64::MAX as f64) as i64
}

/// Safe f64 to i32.
pub fn f64_to_i32(n: f64) -> i32 {
    n.clamp(i32::MIN as f64, i32::MAX as f64) as i32
}

// ============================================================================
// String Parsing with Defaults
// ============================================================================

/// Parse or return default.
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

/// Parse i64 with default.
pub fn parse_i64(s: &str, default: i64) -> i64 {
    s.parse().unwrap_or(default)
}

/// Parse i32 with default.
pub fn parse_i32(s: &str, default: i32) -> i32 {
    s.parse().unwrap_or(default)
}

/// Parse u64 with default.
pub fn parse_u64(s: &str, default: u64) -> u64 {
    s.parse().unwrap_or(default)
}

/// Parse u32 with default.
pub fn parse_u32(s: &str, default: u32) -> u32 {
    s.parse().unwrap_or(default)
}

/// Parse f64 with default.
pub fn parse_f64(s: &str, default: f64) -> f64 {
    s.parse().unwrap_or(default)
}

/// Parse f32 with default.
pub fn parse_f32(s: &str, default: f32) -> f32 {
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
// Time/Duration Conversions
// ============================================================================

/// Convert seconds to human readable duration.
pub fn seconds_to_human(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs < 86400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d {}h", secs / 86400, (secs % 86400) / 3600)
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
    } else {
        format!("{}d", secs / 86400)
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
pub fn secs_to_duration(secs: u64) -> std::time::Duration {
    std::time::Duration::from_secs(secs)
}

/// Convert milliseconds to Duration.
pub fn ms_to_duration(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}

/// Convert Duration to seconds.
pub fn duration_to_secs(d: std::time::Duration) -> u64 {
    d.as_secs()
}

/// Convert Duration to milliseconds.
pub fn duration_to_ms(d: std::time::Duration) -> u128 {
    d.as_millis()
}

// ============================================================================
// String/Option Conversions
// ============================================================================

/// Convert string to Option<T>, returning None for empty string.
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

/// Convert String to Option<String> (None if empty).
pub fn string_to_option(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Trim and convert to option (None if only whitespace).
pub fn trim_to_option(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// ============================================================================
// Float Conversions
// ============================================================================

/// Convert f64 to f32 (may lose precision).
pub fn f64_to_f32(n: f64) -> f32 {
    n as f32
}

/// Round f64 to n decimal places.
pub fn round_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).round() / factor
}

/// Truncate f64 to n decimal places.
pub fn trunc_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).trunc() / factor
}

/// Check if f64 is essentially zero.
pub fn is_zero(n: f64, epsilon: f64) -> bool {
    n.abs() < epsilon
}

/// Compare two f64 for approximate equality.
pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

// ============================================================================
// Vec/Slice Conversions
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

/// Convert slice to fixed array (returns None if wrong size).
pub fn slice_to_array<T: Copy, const N: usize>(slice: &[T]) -> Option<[T; N]> {
    slice.try_into().ok()
}

// ============================================================================
// Char Conversions
// ============================================================================

/// Convert char to u32 (unicode code point).
pub fn char_to_u32(c: char) -> u32 {
    c as u32
}

/// Convert u32 to char (unicode code point).
pub fn u32_to_char(n: u32) -> Option<char> {
    char::from_u32(n)
}

/// Convert char to ascii u8 (None if not ascii).
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

/// Convert digit char to u8 (0-9).
pub fn digit_to_u8(c: char) -> Option<u8> {
    c.to_digit(10).map(|d| d as u8)
}

/// Convert u8 to digit char (0-9).
pub fn u8_to_digit(n: u8) -> Option<char> {
    if n <= 9 {
        Some((b'0' + n) as char)
    } else {
        None
    }
}

// ============================================================================
// Result/Option Conversions
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

// ============================================================================
// Misc Conversions
// ============================================================================

/// Convert IP string to u32 (IPv4 only).
pub fn ip_to_u32(ip: &str) -> Option<u32> {
    let parts: Vec<u8> = ip.split('.').filter_map(|s| s.parse().ok()).collect();
    if parts.len() == 4 {
        Some(
            (parts[0] as u32) << 24
                | (parts[1] as u32) << 16
                | (parts[2] as u32) << 8
                | (parts[3] as u32),
        )
    } else {
        None
    }
}

/// Convert u32 to IP string (IPv4).
pub fn u32_to_ip(n: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (n >> 24) & 0xFF,
        (n >> 16) & 0xFF,
        (n >> 8) & 0xFF,
        n & 0xFF
    )
}

/// Convert color hex to RGB tuple.
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

/// Convert RGB to hex color.
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Swap endianness of u32.
pub fn swap_endian_u32(n: u32) -> u32 {
    n.swap_bytes()
}

/// Swap endianness of u64.
pub fn swap_endian_u64(n: u64) -> u64 {
    n.swap_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_hex() {
        assert_eq!(bytes_to_hex(&[255, 128, 0]), "ff8000");
        assert_eq!(bytes_to_hex_upper(&[255, 128, 0]), "FF8000");
    }

    #[test]
    fn test_i64_conversions() {
        assert_eq!(i64_to_u8(300), 255); // saturates
        assert_eq!(i64_to_u8(-5), 0); // clamps to 0
    }

    #[test]
    fn test_seconds_to_human() {
        assert_eq!(seconds_to_human(90), "1m 30s");
        assert_eq!(seconds_to_human(3665), "1h 1m");
    }

    #[test]
    fn test_ip_conversions() {
        assert_eq!(ip_to_u32("192.168.1.1"), Some(0xC0A80101));
        assert_eq!(u32_to_ip(0xC0A80101), "192.168.1.1");
    }

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#ff8000"), Some((255, 128, 0)));
        assert_eq!(rgb_to_hex(255, 128, 0), "#ff8000");
    }
}
