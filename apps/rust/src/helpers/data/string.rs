//! String utilities.

use once_cell::sync::Lazy;
use regex::Regex;

/// Slugify a string (lowercase, hyphens, no special chars).
pub fn slugify(s: &str) -> String {
    static RE_SPECIAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-z0-9\s-]").unwrap());
    static RE_SPACES: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\s_]+").unwrap());
    static RE_HYPHENS: Lazy<Regex> = Lazy::new(|| Regex::new(r"-+").unwrap());

    let s = s.to_lowercase();
    let s = RE_SPECIAL.replace_all(&s, "");
    let s = RE_SPACES.replace_all(&s, "-");
    let s = RE_HYPHENS.replace_all(&s, "-");
    s.trim_matches('-').to_string()
}

/// Truncate string to max length with ellipsis.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s.chars().take(max_len).collect()
    } else {
        format!("{}...", s.chars().take(max_len - 3).collect::<String>())
    }
}

/// Extract initials from a name.
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// Mask sensitive data (e.g., email).
pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let (local, domain) = email.split_at(at_pos);
        if local.len() <= 2 {
            format!("{}***{}", local, domain)
        } else {
            let visible = &local[..2];
            format!("{}***{}", visible, domain)
        }
    } else {
        "***".to_string()
    }
}

/// Generate a random string.
pub fn random_string(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generate a random alphanumeric code (for verification, etc.).
pub fn random_code(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Check if string is a valid email format.
pub fn is_valid_email(email: &str) -> bool {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
    RE.is_match(email)
}

/// Convert string to title case.
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + chars.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World!"), "hello-world");
        assert_eq!(slugify("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello", 10), "Hello");
        assert_eq!(truncate("Hello World", 8), "Hello...");
    }

    #[test]
    fn test_initials() {
        assert_eq!(initials("John Doe"), "JD");
        assert_eq!(initials("Alice"), "A");
    }
}
