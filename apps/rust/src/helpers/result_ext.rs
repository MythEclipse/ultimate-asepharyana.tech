//! Result and Option extension utilities.

/// Extension trait for Result.
pub trait ResultExt2<T, E> {
    /// Log error and return default.
    fn unwrap_or_log(self, context: &str, default: T) -> T
    where
        E: std::fmt::Display;

    /// Convert to Option, logging error.
    fn ok_or_log(self, context: &str) -> Option<T>
    where
        E: std::fmt::Display;

    /// Map both Ok and Err.
    fn map_both<U, F, G>(self, ok_fn: F, err_fn: G) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
        G: FnOnce(E) -> E;

    /// Tap into Ok value without consuming.
    fn tap_ok<F>(self, f: F) -> Self
    where
        F: FnOnce(&T);

    /// Tap into Err value without consuming.
    fn tap_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E);
}

impl<T, E> ResultExt2<T, E> for Result<T, E> {
    fn unwrap_or_log(self, context: &str, default: T) -> T
    where
        E: std::fmt::Display,
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("[{}] Error: {}", context, e);
                default
            }
        }
    }

    fn ok_or_log(self, context: &str) -> Option<T>
    where
        E: std::fmt::Display,
    {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                tracing::error!("[{}] Error: {}", context, e);
                None
            }
        }
    }

    fn map_both<U, F, G>(self, ok_fn: F, err_fn: G) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
        G: FnOnce(E) -> E,
    {
        match self {
            Ok(v) => Ok(ok_fn(v)),
            Err(e) => Err(err_fn(e)),
        }
    }

    fn tap_ok<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Ok(ref v) = self {
            f(v);
        }
        self
    }

    fn tap_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
    {
        if let Err(ref e) = self {
            f(e);
        }
        self
    }
}

/// Extension trait for Option.
pub trait OptionExt<T> {
    /// Log if None and return default.
    fn unwrap_or_log(self, context: &str, default: T) -> T;

    /// Convert None to Err with message.
    fn ok_or_msg(self, msg: &str) -> Result<T, String>;

    /// Tap into Some value.
    fn tap_some<F>(self, f: F) -> Self
    where
        F: FnOnce(&T);

    /// Execute on None.
    fn on_none<F>(self, f: F) -> Self
    where
        F: FnOnce();
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_log(self, context: &str, default: T) -> T {
        match self {
            Some(v) => v,
            None => {
                tracing::warn!("[{}] Value was None, using default", context);
                default
            }
        }
    }

    fn ok_or_msg(self, msg: &str) -> Result<T, String> {
        self.ok_or_else(|| msg.to_string())
    }

    fn tap_some<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Some(ref v) = self {
            f(v);
        }
        self
    }

    fn on_none<F>(self, f: F) -> Self
    where
        F: FnOnce(),
    {
        if self.is_none() {
            f();
        }
        self
    }
}

/// Wrap a value in Ok.
pub fn ok<T, E>(value: T) -> Result<T, E> {
    Ok(value)
}

/// Wrap a value in Some.
pub fn some<T>(value: T) -> Option<T> {
    Some(value)
}

/// Create an error result.
pub fn err<T, E>(error: E) -> Result<T, E> {
    Err(error)
}

/// Flatten nested option.
pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.flatten()
}

/// Flatten nested result.
pub fn flatten_result<T, E>(res: Result<Result<T, E>, E>) -> Result<T, E> {
    res.and_then(|r| r)
}
