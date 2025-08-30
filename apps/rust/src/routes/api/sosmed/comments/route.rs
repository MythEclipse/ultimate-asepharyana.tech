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
use rust_lib::models::{comments::Comments, comments::CommentRequest, user::User};
use jsonwebtoken::{decode, DecodingKey, Validation};
use chrono::Utc;
use sqlx::MySqlPool;

// Helper to get IP (simplified for Rust)
fn get_ip() -> String {
    "unknown".to_string() // Placeholder, actual IP extraction is more complex in Rust
}

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

pub async fn comments_post_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<CommentRequest>,
) -> Response {
    let ip = get_ip();
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

    if payload.content.is_empty() || payload.post_id.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Content and postId are required" })),
        )
            .into_response();
    }
    let post_id = payload.post_id.unwrap();

    let new_comment_id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().naive_utc();

    match sqlx::query_as::<_, Comments>(
        "INSERT INTO Comments (id, postId, content, userId, authorId, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&new_comment_id)
    .bind(&post_id)
    .bind(&payload.content)
    .bind(&user_id)
    .bind(&user_id) // Assuming authorId is same as userId
    .bind(&created_at)
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(comment) => (
            StatusCode::CREATED,
            Json(json!({
                "comment": {
                    "id": comment.id,
                    "postId": comment.post_id,
                    "content": comment.content,
                    "created_at": comment.created_at,
                }
            })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error creating comment: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to add comment" })),
            )
                .into_response()
        }
    }
}

pub async fn comments_get_handler(
    Query(params): Query<CommentRequest>,
    State(state): State<Arc<ChatState>>,
) -> Response {
    let ip = get_ip();
    let db_pool = &state.pool;

    let Some(post_id) = params.post_id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Post ID is required" })),
        )
            .into_response();
    };

    match sqlx::query_as::<_, Comments>(
        r#"
        SELECT
            Comments.id, Comments.postId, Comments.content, Comments.userId, Comments.authorId, Comments.created_at,
            User.name as user_name, User.image as user_image
        FROM Comments
        LEFT JOIN User ON User.id = Comments.userId
        WHERE Comments.postId = ?
        ORDER BY Comments.created_at DESC
        "#
    )
    .bind(&post_id)
    .fetch_all(db_pool.as_ref())
    .await
    {
        Ok(comments) => (StatusCode::OK, Json(json!({ "comments": comments }))).into_response(),
        Err(e) => {
            eprintln!("Error fetching comments: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch comments" })),
            )
                .into_response()
        }
    }
}

pub async fn comments_put_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<CommentRequest>,
) -> Response {
    let ip = get_ip();
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

    let Some(id) = payload.id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Comment ID is required" })),
        )
            .into_response();
    };
    let content = payload.content;

    let comment = match sqlx::query_as::<_, Comments>("SELECT * FROM Comments WHERE id = ?")
        .bind(&id)
        .fetch_optional(db_pool.as_ref())
        .await
    {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "Comment not found" })),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Database error fetching comment: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    if comment.user_id != user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "User not authorized to edit this comment" })),
        )
            .into_response();
    }

    match sqlx::query_as::<_, Comments>(
        "UPDATE Comments SET content = ? WHERE id = ?"
    )
    .bind(&format!("{} -edited", content))
    .bind(&id)
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(updated_comment) => (
            StatusCode::OK,
            Json(json!({ "message": "Comment updated successfully!", "comment": updated_comment })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error updating comment: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to update comment" })),
            )
                .into_response()
        }
    }
}

pub async fn comments_delete_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<CommentRequest>,
) -> Response {
    let ip = get_ip();
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

    let Some(id) = payload.id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Comment ID is required" })),
        )
            .into_response();
    };

    let comment = match sqlx::query_as::<_, Comments>("SELECT * FROM Comments WHERE id = ?")
        .bind(&id)
        .fetch_optional(db_pool.as_ref())
        .await
    {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "Comment not found" })),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Database error fetching comment: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    if comment.user_id != user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "User not authorized to delete this comment" })),
        )
            .into_response();
    }

    match sqlx::query("DELETE FROM Comments WHERE id = ?")
        .bind(&id)
        .execute(db_pool.as_ref())
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Comment deleted successfully!" })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error deleting comment: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to delete comment" })),
            )
                .into_response()
        }
    }
}
