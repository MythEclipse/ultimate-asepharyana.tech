//! JWT utilities for signing and verifying tokens.
//!
//! Uses the type-safe CONFIG for JWT secret.

use crate::core::config::CONFIG;
use crate::core::error::AppError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

pub fn encode_jwt(claims: Claims) -> Result<String, AppError> {
    let secret = &CONFIG.jwt_secret;
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::from)
}

pub fn decode_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = &CONFIG.jwt_secret;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(AppError::from)
}
