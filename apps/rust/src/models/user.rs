use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Import SeaORM entity
use crate::entities::user;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub role: String,
}

impl From<user::Model> for UserResponse {
    /// Convert SeaORM user entity to UserResponse
    fn from(user: user::Model) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            image: user.image,
            email_verified: user.email_verified.is_some(),
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
