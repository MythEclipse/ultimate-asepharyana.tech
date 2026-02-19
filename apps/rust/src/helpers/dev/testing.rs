//! Testing utilities and helpers.

/// Assert that two JSON values are equal.
#[macro_export]
macro_rules! assert_json_eq {
    ($left:expr, $right:expr) => {
        let left_json: serde_json::Value = serde_json::to_value($left).unwrap();
        let right_json: serde_json::Value = serde_json::to_value($right).unwrap();
        assert_eq!(left_json, right_json);
    };
}

/// Assert that a result is Ok.
#[macro_export]
macro_rules! assert_ok {
    ($expr:expr) => {
        assert!($expr.is_ok(), "Expected Ok, got Err: {:?}", $expr.err());
    };
    ($expr:expr, $msg:expr) => {
        assert!(
            $expr.is_ok(),
            "{}: Expected Ok, got Err: {:?}",
            $msg,
            $expr.err()
        );
    };
}

/// Assert that a result is Err.
#[macro_export]
macro_rules! assert_err {
    ($expr:expr) => {
        assert!($expr.is_err(), "Expected Err, got Ok: {:?}", $expr.ok());
    };
    ($expr:expr, $msg:expr) => {
        assert!(
            $expr.is_err(),
            "{}: Expected Err, got Ok: {:?}",
            $msg,
            $expr.ok()
        );
    };
}

/// Assert that an option is Some.
#[macro_export]
macro_rules! assert_some {
    ($expr:expr) => {
        assert!($expr.is_some(), "Expected Some, got None");
    };
}

/// Assert that an option is None.
#[macro_export]
macro_rules! assert_none {
    ($expr:expr) => {
        assert!($expr.is_none(), "Expected None, got Some: {:?}", $expr);
    };
}

/// Assert that a string contains a substring.
#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected {:?} to contain {:?}",
            $haystack,
            $needle
        );
    };
}

/// Assert that a string starts with prefix.
#[macro_export]
macro_rules! assert_starts_with {
    ($string:expr, $prefix:expr) => {
        assert!(
            $string.starts_with($prefix),
            "Expected {:?} to start with {:?}",
            $string,
            $prefix
        );
    };
}

/// Assert that a string ends with suffix.
#[macro_export]
macro_rules! assert_ends_with {
    ($string:expr, $suffix:expr) => {
        assert!(
            $string.ends_with($suffix),
            "Expected {:?} to end with {:?}",
            $string,
            $suffix
        );
    };
}

/// Mock HTTP response builder.
#[derive(Debug, Clone)]
pub struct MockResponse {
    pub status: u16,
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
}

impl MockResponse {
    pub fn new(status: u16) -> Self {
        Self {
            status,
            body: String::new(),
            headers: std::collections::HashMap::new(),
        }
    }

    pub fn ok() -> Self {
        Self::new(200)
    }

    pub fn not_found() -> Self {
        Self::new(404)
    }

    pub fn error() -> Self {
        Self::new(500)
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    pub fn json<T: serde::Serialize>(mut self, value: &T) -> Self {
        self.body = serde_json::to_string(value).unwrap();
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

/// Simple test fixture.
pub struct TestFixture<T> {
    pub data: T,
    setup_done: bool,
}

impl<T> TestFixture<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            setup_done: false,
        }
    }

    pub fn setup<F: FnOnce(&mut T)>(mut self, f: F) -> Self {
        f(&mut self.data);
        self.setup_done = true;
        self
    }

    pub fn get(&self) -> &T {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

/// Generate random test data.
pub mod random {
    use rand::Rng;

    pub fn string(len: usize) -> String {
        use rand::distributions::Alphanumeric;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn email() -> String {
        format!("test_{}@example.com", string(8).to_lowercase())
    }

    pub fn int(min: i64, max: i64) -> i64 {
        rand::thread_rng().gen_range(min..=max)
    }

    pub fn bool() -> bool {
        rand::thread_rng().gen()
    }

    pub fn choice<T: Clone>(items: &[T]) -> T {
        let idx = rand::thread_rng().gen_range(0..items.len());
        items[idx].clone()
    }
}
