// Minimal DTO definitions to resolve unresolved import errors
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Likes {
    pub id: String,
    pub post_id: String,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LikeRequest {
    pub id: Option<String>,
    pub post_id: Option<String>,
}
 // Handlers for Likes API endpoints.
 //
 /// Provides endpoints for liking and unliking posts, including authentication and error handling.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState;
use rust_lib::utils::auth::verify_jwt;

/// Handler for liking a post.
pub async fn likes_post_handler(
    State(state): State<Arc<ChatState>>,
    jar: CookieJar,
    Json(payload): Json<LikeRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token_value = match jar.get("token").map(|cookie| cookie.value().to_string()) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Authentication required" })),
            )
                .into_response();
        }
    };

    let decoded_claims = match verify_jwt(&token_value).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Invalid token" })),
            )
                .into_response();
        }
    };
    let user_id = decoded_claims.user_id;
    let post_id = payload.post_id;

    let existing_like = match sqlx::query_as::<_, Likes>(
        "SELECT * FROM Likes WHERE userId = ? AND postId = ?"
    )
    .bind(&user_id)
    .bind(&post_id)
    .fetch_optional(db_pool.as_ref())
    .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to like post" })),
            )
                .into_response();
        }
    };

    if existing_like {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "message": "Already liked" })),
        )
            .into_response();
    }

    match sqlx::query_as::<_, Likes>(
        "INSERT INTO Likes (postId, userId) VALUES (?, ?)"
    )
    .bind(&post_id)
    .bind(&user_id)
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(like) => (
            StatusCode::CREATED,
            Json(json!({ "like": like })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to like post" })),
        )
            .into_response(),
    }
}

/// Handler for unliking a post.
pub async fn likes_delete_handler(
    State(state): State<Arc<ChatState>>,
    jar: CookieJar,
    Json(payload): Json<LikeRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token_value = match jar.get("token").map(|cookie| cookie.value().to_string()) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Authentication required" })),
            )
                .into_response();
        }
    };

    let decoded_claims = match verify_jwt(&token_value).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Invalid token" })),
            )
                .into_response();
        }
    };
    let user_id = decoded_claims.user_id;
    let post_id = payload.post_id;

    let existing_like = match sqlx::query_as::<_, Likes>(
        "SELECT * FROM Likes WHERE userId = ? AND postId = ?"
    )
    .bind(&user_id)
    .bind(&post_id)
    .fetch_optional(db_pool.as_ref())
    .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to remove like" })),
            )
                .into_response();
        }
    };

    if !existing_like {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Like not found" })),
        )
            .into_response();
    }

    match sqlx::query(
        "DELETE FROM Likes WHERE userId = ? AND postId = ?"
    )
    .bind(&user_id)
    .bind(&post_id)
    .execute(db_pool.as_ref())
    .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Like removed successfully" })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to remove like" })),
        )
            .into_response(),
    }
}
