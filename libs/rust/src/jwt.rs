use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // Add other fields from your JwtPayload here
}

pub fn sign_jwt(payload: Claims) -> Result<String, AppError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    encode(&Header::default(), &payload, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)).map(|data| data.claims)
}
