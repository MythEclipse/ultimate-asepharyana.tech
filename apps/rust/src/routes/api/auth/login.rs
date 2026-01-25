//! Handler for the login endpoint - Enhanced with form_request validation.

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::verify;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// SeaORM imports
use crate::entities::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::models::user::{LoginResponse, UserResponse};
use crate::routes::AppState;
use crate::utils::auth::{encode_jwt, Claims};
use crate::utils::error::AppError;

// New helpers
use crate::helpers::form_request::{validate, ValidationRules};

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/api/auth/login";
pub const ENDPOINT_DESCRIPTION: &str = "Authenticate user and return JWT tokens";
pub const ENDPOINT_TAG: &str = "auth";
pub const OPERATION_ID: &str = "auth_login";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<LoginResponse>";

/// Login request payload
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
    /// User email address
    pub email: String,
    /// User password
    pub password: String,
    /// Remember me option (extends token expiry)
    #[serde(default)]
    pub remember_me: bool,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    operation_id = "auth_login",
    responses(
        (status = 200, description = "Authenticate user and return JWT tokens", body = LoginResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input using form_request helper
    let data = serde_json::to_value(&payload).unwrap_or_default();
    let mut rules = ValidationRules::new();
    rules
        .required("email")
        .email("email")
        .required("password")
        .min_length("password", 1);

    let validation = validate(&data, &rules);
    if !validation.is_valid() {
        return Err(AppError::Other(format!(
            "Validation failed: {:?}",
            validation.errors.first().map(|e| &e.message)
        )));
    }

    // Find user by email using SeaORM
    let user_model: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let user_model = user_model.ok_or(AppError::InvalidCredentials)?;

    // Verify password
    let password_valid = verify(
        &payload.password,
        user_model
            .password
            .as_ref()
            .ok_or(AppError::InvalidCredentials)?,
    )?;

    if !password_valid {
        tracing::warn!("Login failed for user {}: invalid password", user_model.id);
        return Err(AppError::InvalidCredentials);
    }

    // Generate JWT tokens
    let token_expiry = if payload.remember_me {
        30 * 24 * 3600
    } else {
        24 * 3600
    };
    let exp = (Utc::now().timestamp() + token_expiry) as usize;

    let claims = Claims {
        user_id: user_model.id.clone(),
        email: user_model.email.clone().unwrap_or_default(),
        name: user_model.name.clone().unwrap_or_default(),
        exp,
    };

    let access_token = encode_jwt(claims)?;

    // Generate refresh token
    let refresh_token = Uuid::new_v4().to_string();

    // Store refresh token
    let mut user_active: user::ActiveModel = user_model.clone().into();
    user_active.refresh_token = Set(Some(refresh_token.clone()));
    user_active
        .update(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    tracing::info!("User {} logged in successfully", user_model.id);

    // Convert to response
    let user_response: UserResponse = user_model.into();

    Ok(Json(LoginResponse {
        user: user_response,
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: token_expiry,
    }))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, post(login))
}
