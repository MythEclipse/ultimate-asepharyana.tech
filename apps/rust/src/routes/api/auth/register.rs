//! Handler for the register endpoint.

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// SeaORM imports
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entities::{user, email_verification_token};

use crate::models::user::UserResponse;
use crate::routes::AppState;
use crate::utils::email::EmailService;
use crate::utils::error::AppError;

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/register";
pub const ENDPOINT_DESCRIPTION: &str = "Register a new user account";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_register";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<RegisterResponse>";

/// Register request payload
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub full_name: Option<String>,
}

/// Register response
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user: UserResponse,
    pub verification_token: Option<String>,
}

/// Validate password strength
fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::WeakPassword(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(AppError::WeakPassword(
            "Password must contain uppercase, lowercase, and numbers".to_string(),
        ));
    }

    if !has_special {
        return Err(AppError::WeakPassword(
            "Password should contain at least one special character".to_string(),
        ));
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    operation_id = "auth_register",
    responses(
        (status = 200, description = "Register a new user account", body = RegisterResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Other(format!("Validation error: {}", e)))?;

    // Validate password strength
    validate_password_strength(&payload.password)?;

    // Check if email already exists using SeaORM
    let email_exists = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if email_exists {
        return Err(AppError::EmailAlreadyExists);
    }

    // Check if name/username already exists using SeaORM
    let username_exists = user::Entity::find()
        .filter(user::Column::Name.eq(&payload.username))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if username_exists {
        return Err(AppError::UsernameAlreadyExists);
    }

    // Hash password
    let password_hash = hash(&payload.password, DEFAULT_COST)?;

    // Generate user ID
    let user_id = Uuid::new_v4().to_string();

    // Insert user into database using SeaORM
    let new_user = user::ActiveModel {
        id: Set(user_id.clone()),
        email: Set(Some(payload.email.clone())),
        name: Set(Some(payload.username.clone())),
        password: Set(Some(password_hash)),
        email_verified: Set(None), // Will be set after verification
        image: Set(None),
        refresh_token: Set(None),
        role: Set("user".to_string()),
    };

    let inserted_user = new_user.insert(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Generate email verification token using SeaORM
    let verification_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::hours(24);
    let now = Utc::now();

    let verification_token_model = email_verification_token::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        user_id: Set(user_id.clone()),
        token: Set(verification_token.clone()),
        expires_at: Set(expires_at),
        created_at: Set(now),
    };

    verification_token_model
        .insert(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Convert to UserResponse
    let user_response: UserResponse = inserted_user.into();

        // Send verification email
    let email_service = EmailService::new();
    let user_name = user_response.name.clone().unwrap_or_else(|| "User".to_string());
    let user_email = user_response.email.clone().unwrap_or_else(|| payload.email.clone());

    if let Err(e) = email_service
        .send_verification_email(&user_email, &user_name, &verification_token)
        .await
    {
        tracing::warn!("Failed to send verification email: {}", e);
        // Don't fail registration if email fails
    }

    Ok((
        axum::http::StatusCode::CREATED,
        Json(RegisterResponse {
            success: true,
            message: "User registered successfully. Please check your email to verify your account.".to_string(),
            user: user_response,
            verification_token: Some(verification_token),
        }),
    ))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(register))
}