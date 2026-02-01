//! Handler for the register endpoint - Enhanced with form_request validation.

use axum::{extract::State, response::IntoResponse, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use crate::entities::{email_verification_token, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::models::user::UserResponse;
use crate::routes::AppState;
use crate::helpers::mailer::EmailService;
use crate::core::error::AppError;

// New helpers
use crate::helpers::email_template::welcome_email;
use crate::helpers::form_request::{validate, ValidationRules};


/// Register request payload
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: Option<String>,
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
    // Validate input using form_request helper
    let data = serde_json::to_value(&payload).unwrap_or_default();
    let mut rules = ValidationRules::new();
    rules
        .required("email")
        .email("email")
        .required("username")
        .min_length("username", 3)
        .max_length("username", 50)
        .required("password")
        .min_length("password", 8);

    // Add password confirmation check if provided
    if payload.password_confirmation.is_some() {
        rules.confirmed("password", "password_confirmation");
    }

    let validation = validate(&data, &rules);
    if !validation.is_valid() {
        return Err(AppError::Other(format!(
            "Validation failed: {}",
            validation
                .errors
                .iter()
                .map(|e| e.message.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    // Validate password strength
    validate_password_strength(&payload.password)?;

    // Check if email already exists
    let email_exists = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .is_some();

    if email_exists {
        return Err(AppError::EmailAlreadyExists);
    }

    // Check if username already exists
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

    // Insert user
    let new_user = user::ActiveModel {
        id: Set(user_id.clone()),
        email: Set(Some(payload.email.clone())),
        name: Set(Some(payload.username.clone())),
        password: Set(Some(password_hash)),
        email_verified: Set(None),
        image: Set(None),
        refresh_token: Set(None),
        role: Set("user".to_string()),
    };

    let inserted_user = new_user
        .insert(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Generate verification token
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

    tracing::info!("New user registered: {}", user_id);

    // Convert to response
    let user_response: UserResponse = inserted_user.into();

    // Send verification email using email_template helper
    let user_name = user_response
        .name
        .clone()
        .unwrap_or_else(|| "User".to_string());
    let user_email = user_response
        .email
        .clone()
        .unwrap_or_else(|| payload.email.clone());
    let verify_url = format!(
        "https://asepharyana.cloud/verify?token={}",
        verification_token
    );

    let _email_template = welcome_email(&user_name, &verify_url);

    let email_service = EmailService::new();
    if let Err(e) = email_service
        .send_verification_email(&user_email, &user_name, &verification_token)
        .await
    {
        tracing::warn!("Failed to send verification email: {}", e);
    }

    Ok((
        axum::http::StatusCode::CREATED,
        Json(RegisterResponse {
            success: true,
            message:
                "User registered successfully. Please check your email to verify your account."
                    .to_string(),
            user: user_response,
            verification_token: Some(verification_token),
        }),
    ))
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

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(AppError::WeakPassword(
            "Password must contain uppercase, lowercase, and numbers".to_string(),
        ));
    }

    Ok(())
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}