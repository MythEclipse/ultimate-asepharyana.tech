//! Date and time utilities.

use chrono::{DateTime, Duration, NaiveDateTime, Utc};

/// Get current UTC timestamp.
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Get current Unix timestamp (seconds).
pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Get current Unix timestamp (milliseconds).
pub fn timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// Format datetime as ISO 8601 string.
pub fn to_iso(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Format datetime as human-readable string.
pub fn to_human(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Format datetime as date only.
pub fn to_date(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d").to_string()
}

/// Parse ISO 8601 string to DateTime.
pub fn parse_iso(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Add duration to a datetime.
pub fn add_days(dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    dt + Duration::days(days)
}

/// Add hours to a datetime.
pub fn add_hours(dt: DateTime<Utc>, hours: i64) -> DateTime<Utc> {
    dt + Duration::hours(hours)
}

/// Add minutes to a datetime.
pub fn add_minutes(dt: DateTime<Utc>, minutes: i64) -> DateTime<Utc> {
    dt + Duration::minutes(minutes)
}

/// Check if datetime is in the past.
pub fn is_past(dt: DateTime<Utc>) -> bool {
    dt < Utc::now()
}

/// Check if datetime is in the future.
pub fn is_future(dt: DateTime<Utc>) -> bool {
    dt > Utc::now()
}

/// Get relative time string (e.g., "2 hours ago").
pub fn relative(dt: DateTime<Utc>) -> String {
    let duration = Utc::now().signed_duration_since(dt);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} minutes ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_days() < 30 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_days() < 365 {
        format!("{} months ago", duration.num_days() / 30)
    } else {
        format!("{} years ago", duration.num_days() / 365)
    }
}

/// Calculate age in years from birthdate.
pub fn age_years(birthdate: NaiveDateTime) -> i32 {
    let today = Utc::now().naive_utc();
    let years = today.date().years_since(birthdate.date());
    years.map(|y| y as i32).unwrap_or(0)
}

/// Get start of day (00:00:00).
pub fn start_of_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive()
        .and_hms_opt(0, 0, 0)
        .map(|naive| DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
        .unwrap_or(dt)
}

/// Get end of day (23:59:59).
pub fn end_of_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive()
        .and_hms_opt(23, 59, 59)
        .map(|naive| DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
        .unwrap_or(dt)
}
