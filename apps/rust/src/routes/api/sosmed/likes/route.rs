use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::routes::mod_::ChatState; // Updated path to ChatState
use rust_lib::models::{likes::Likes, likes::LikeRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use chrono::Utc;
use sqlx::MySqlPool;

// Claims struct for JWT decoding
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    user_id: String,
    email: String,
    name: String,
    exp: usize,
}

// Helper to verify JWT
async fn verify_jwt(token: &str, jwt_secret: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let validation = Validation::default();
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret.as_bytes()), &validation)?;
    Ok(decoded.claims)
}

pub async fn likes_post_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<LikeRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token = HeaderMap::new(); // Placeholder for cookie extraction
    let token_value = "dummy_token"; // Replace with actual cookie extraction

    let decoded_claims = match verify_jwt(token_value, jwt_secret).await {
        Ok(claims) => claims,
        Err(e) => {
            eprintln!("Authentication error: {:?}", e);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Authentication required" })),
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
        Ok(Some(like)) => Some(like),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Database error checking existing like: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    if existing_like.is_some() {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "message": "Already liked" })),
        )
            .into_response();
    }

    let new_like_id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().naive_utc();

    match sqlx::query_as::<_, Likes>(
        "INSERT INTO Likes (id, postId, userId, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&new_like_id)
    .bind(&post_id)
    .bind(&user_id)
    .bind(&created_at)
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(like) => (
            StatusCode::CREATED,
            Json(json!({ "like": like })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error creating like: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to like post" })),
            )
                .into_response()
        }
    }
}

pub async fn likes_delete_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<LikeRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token = HeaderMap::new(); // Placeholder for cookie extraction
    let token_value = "dummy_token"; // Replace with actual cookie extraction

    let decoded_claims = match verify_jwt(token_value, jwt_secret).await {
        Ok(claims) => claims,
        Err(e) => {
            eprintln!("Authentication error: {:?}", e);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Authentication required" })),
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
        Ok(Some(like)) => Some(like),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Database error checking existing like: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    if existing_like.is_none() {
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
        Err(e) => {
            eprintln!("Error deleting like: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to remove like" })),
            )
                .into_response()
        }
    }
}
