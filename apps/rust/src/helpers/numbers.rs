//! Number utilities.

/// Format number with thousand separators.
pub fn format_number(n: i64) -> String {
    let s = n.abs().to_string();
    let chars: Vec<char> = s.chars().rev().collect();
    let mut result = String::new();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    if n < 0 {
        result.push('-');
    }

    result.chars().rev().collect()
}

/// Format as percentage.
pub fn format_percent(value: f64, decimals: usize) -> String {
    format!("{:.1$}%", value * 100.0, decimals)
}

/// Format as currency.
pub fn format_currency(value: f64, symbol: &str) -> String {
    format!("{}{:.2}", symbol, value)
}

/// Format bytes to human readable.
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Clamp value to range.
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Map value from one range to another.
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let from_range = from_max - from_min;
    let to_range = to_max - to_min;
    ((value - from_min) / from_range) * to_range + to_min
}

/// Round to n decimal places.
pub fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// Check if number is even.
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// Check if number is odd.
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// Check if number is positive.
pub fn is_positive<T: PartialOrd + Default>(n: T) -> bool {
    n > T::default()
}

/// Check if number is negative.
pub fn is_negative<T: PartialOrd + Default>(n: T) -> bool {
    n < T::default()
}

/// Safe division (returns 0 if divisor is 0).
pub fn safe_div(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        0.0
    } else {
        a / b
    }
}

/// Calculate percentage.
pub fn percentage(part: f64, total: f64) -> f64 {
    safe_div(part, total) * 100.0
}

/// Parse string to i64 with default.
pub fn parse_i64(s: &str, default: i64) -> i64 {
    s.parse().unwrap_or(default)
}

/// Parse string to f64 with default.
pub fn parse_f64(s: &str, default: f64) -> f64 {
    s.parse().unwrap_or(default)
}

/// Generate range of numbers.
pub fn range(start: i64, end: i64) -> Vec<i64> {
    (start..end).collect()
}

/// Generate range with step.
pub fn range_step(start: i64, end: i64, step: i64) -> Vec<i64> {
    (start..end).step_by(step as usize).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(-1234), "-1,234");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }
}
