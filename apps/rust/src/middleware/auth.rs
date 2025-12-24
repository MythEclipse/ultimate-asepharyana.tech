// JWT Authentication middleware for Axum

use crate::routes::AppState;
use crate::utils::auth::{decode_jwt, Claims};
use axum::{
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use redis::AsyncCommands;
use serde_json::json;
use std::sync::Arc;

pub struct AuthMiddleware(pub Claims);

impl<S> FromRequestParts<S> for AuthMiddleware
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(token) => token,
            None => return Err(AuthError::MissingToken),
        };

        match decode_jwt(token) {
            Ok(claims) => Ok(AuthMiddleware(claims)),
            Err(_) => Err(AuthError::InvalidToken),
        }
    }
}

pub enum AuthError {
    MissingToken,
    InvalidToken,
    TokenRevoked,
    AccountInactive,
    UserNotFound,
    InsufficientPermissions,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::TokenRevoked => (StatusCode::UNAUTHORIZED, "Token has been revoked"),
            AuthError::AccountInactive => (StatusCode::FORBIDDEN, "Account is inactive"),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found"),
            AuthError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
        };
        (status, message).into_response()
    }
}

/// Extract Bearer token from Authorization header
fn extract_token(headers: &HeaderMap) -> Result<String, AuthError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::MissingToken);
    }

    Ok(auth_header[7..].to_string())
}

/// Advanced authentication middleware with Redis blacklist check
pub async fn auth_layer(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extract token from headers
    let token = extract_token(req.headers()).map_err(|e| e.into_response())?;

    // Decode JWT token
    let claims = decode_jwt(&token).map_err(|_| AuthError::InvalidToken.into_response())?;

    // Check if token is blacklisted in Redis
    let mut redis_conn = state.redis_pool.get().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("Redis error: {}", e)
            })),
        )
            .into_response()
    })?;

    let blacklist_key = format!("blacklist:token:{}", token);
    let is_blacklisted: bool = redis_conn
        .exists(&blacklist_key)
        .await
        .unwrap_or(false);

    if is_blacklisted {
        return Err(AuthError::TokenRevoked.into_response());
    }

    // Verify user still exists in database using SeaORM
    use crate::entities::{user};
    use sea_orm::EntityTrait;

    let user_exists = user::Entity::find_by_id(&claims.user_id)
        .one(state.sea_orm())
        .await
        .ok()
        .flatten()
        .is_some();

    if user_exists {
        // User exists, add claims to request extensions
        req.extensions_mut().insert(claims);
        Ok(next.run(req).await)
    } else {
        Err(AuthError::UserNotFound.into_response())
    }
}

/// Optional authentication middleware - doesn't fail if no token
pub async fn optional_auth_layer(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    // Try to extract token, but don't fail if not present
    if let Ok(token) = extract_token(req.headers()) {
        // Try to decode and validate
        if let Ok(claims) = decode_jwt(&token) {
            // Check if token is blacklisted
            if let Ok(mut redis_conn) = state.redis_pool.get().await {
                let blacklist_key = format!("blacklist:token:{}", token);
                let is_blacklisted: bool = redis_conn
                    .exists(&blacklist_key)
                    .await
                    .unwrap_or(false);

                if !is_blacklisted {
                    // Check if user still exists using SeaORM
                    use crate::entities::user;
                    use sea_orm::EntityTrait;

                    if let Ok(Some(_)) = user::Entity::find_by_id(&claims.user_id)
                        .one(state.sea_orm())
                        .await
                    {
                        // Add claims to request extensions
                        req.extensions_mut().insert(claims);
                    }
                }
            }
        }
    }

    next.run(req).await
}

/// Extract claims from request extensions (use in handlers after auth middleware)
pub fn get_claims_from_request(req: &Request) -> Option<&Claims> {
    req.extensions().get::<Claims>()
}
