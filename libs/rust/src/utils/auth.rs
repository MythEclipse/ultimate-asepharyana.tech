use jsonwebtoken::{decode, DecodingKey, Validation};
use std::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

pub async fn verify_jwt(token: &str, jwt_secret: &str) -> Result<Claims, Box<dyn Error>> {
    let validation = Validation::default();
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret.as_bytes()), &validation)?;
    Ok(decoded.claims)
}
