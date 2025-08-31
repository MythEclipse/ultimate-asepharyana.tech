use axum::{
    extract::{State},
    http::{StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use crate::routes::ChatState;
use crate::routes::api::user::likes_dto::{Likes, LikeRequest};
use crate::utils::auth::{Claims, verify_jwt};
use sqlx::MySqlPool;

pub async fn likes_post_handler(
    State(state): State<Arc<ChatState>>,
    jar: CookieJar,
    Json(payload): Json<LikeRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token = jar.get("token").map(|cookie| cookie.value().to_string());
    let token_value = match token {
        Some(t) => t,
        None => {
            eprintln!("Authentication error: No token provided");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Authentication required" })),
            )
                .into_response();
        }
    };

    let decoded_claims = match verify_jwt(&token_value, jwt_secret).await {
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
                Json(json!({ "message": "Failed to like post" })),
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
    jar: CookieJar,
    Json(payload): Json<LikeRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token = jar.get("token").map(|cookie| cookie.value().to_string());
    let token_value = match token {
        Some(t) => t,
        None => {
            eprintln!("Authentication error: No token provided");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Authentication required" })),
            )
                .into_response();
        }
    };

    let decoded_claims = match verify_jwt(&token_value, jwt_secret).await {
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
                Json(json!({ "message": "Failed to remove like" })),
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
