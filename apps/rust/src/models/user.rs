use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// Import SeaORM entity
use crate::entities::user;

/// Legacy User struct for SQLx compatibility (will be phased out)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub email_verified: bool,
    pub is_active: bool,
    pub role: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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

impl From<User> for UserResponse {
    /// Legacy conversion from SQLx User to UserResponse
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: Some(user.email),
            name: Some(user.username),
            image: user.avatar_url,
            email_verified: user.email_verified,
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
