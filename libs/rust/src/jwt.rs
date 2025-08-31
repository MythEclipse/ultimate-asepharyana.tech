// JWT utilities for signing and verifying tokens using dynamic config from CONFIG_MAP

use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::config::CONFIG_MAP;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // Add other fields from your JwtPayload here
}

pub fn sign_jwt(payload: Claims) -> Result<String, AppError> {
    let secret = CONFIG_MAP.get("JWT_SECRET")
        .cloned()
        .unwrap_or_else(|| "default_secret".to_string());
    encode(&Header::default(), &payload, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(AppError::from)
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = CONFIG_MAP.get("JWT_SECRET")
        .cloned()
        .unwrap_or_else(|| "default_secret".to_string());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
        .map_err(AppError::from)
}
