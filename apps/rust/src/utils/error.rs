use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Scraper error: {0}")]
    ScraperError(String),
    #[error("Fantoccini error: {0}")]
    FantocciniError(String),
    #[error("Chromiumoxide error: {0}")]
    ChromiumoxideError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Other error: {0}")]
    Other(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),

    // Authentication Errors
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Username already exists")]
    UsernameAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Account is inactive")]
    AccountInactive,
    #[error("Password too weak: {0}")]
    WeakPassword(String),
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Bcrypt error: {0}")]
    BcryptError(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
}

impl From<failure::Error> for AppError {
    fn from(err: failure::Error) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Other(s.to_string())
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Other(s)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::BcryptError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::InvalidCredentials => (http::StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::EmailAlreadyExists => (http::StatusCode::CONFLICT, self.to_string()),
            AppError::UsernameAlreadyExists => (http::StatusCode::CONFLICT, self.to_string()),
            AppError::UserNotFound => (http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidToken => (http::StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::TokenExpired => (http::StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::EmailNotVerified => (http::StatusCode::FORBIDDEN, self.to_string()),
            AppError::AccountInactive => (http::StatusCode::FORBIDDEN, self.to_string()),
            AppError::WeakPassword(_) => (http::StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InvalidEmail => (http::StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Unauthorized => (http::StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden => (http::StatusCode::FORBIDDEN, self.to_string()),
            AppError::DatabaseError(_) => {
                (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            _ => (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, error_message).into_response()
    }
}
