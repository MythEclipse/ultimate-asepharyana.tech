// JWT Claims and verification using dynamic config from CONFIG_MAP

use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

pub async fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = CONFIG_MAP.get("JWT_SECRET")
        .cloned()
        .unwrap_or_else(|| "default_secret".to_string());
    let validation = Validation::default();
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)?;
    Ok(decoded.claims)
}
