// JWT Claims and utilities using dynamic config from CONFIG_MAP

use jsonwebtoken::{encode, decode, Header, DecodingKey, EncodingKey, Validation, Algorithm};
use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

pub fn encode_jwt(claims: Claims) -> Result<String, AppError> {
    let secret = CONFIG_MAP.get("JWT_SECRET")
        .cloned()
        .unwrap_or_else(|| "default_secret".to_string());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(AppError::from)
}

pub fn decode_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = CONFIG_MAP.get("JWT_SECRET")
        .cloned()
        .unwrap_or_else(|| "default_secret".to_string());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)
        .map(|data| data.claims)
        .map_err(AppError::from)
}
