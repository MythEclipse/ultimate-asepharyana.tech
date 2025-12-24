//! JWT utilities for signing and verifying tokens.
//!
//! Uses the type-safe CONFIG for JWT secret.

use crate::core::config::CONFIG;
use crate::utils::error::AppError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // Add other fields from your JwtPayload here
}

pub fn sign_jwt(payload: Claims) -> Result<String, AppError> {
    let secret = &CONFIG.jwt_secret;
    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(AppError::from)
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = &CONFIG.jwt_secret;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(AppError::from)
}
