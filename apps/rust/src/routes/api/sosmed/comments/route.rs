use axum::{
    extract::{Query, State},
    http::{StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use crate::routes::mod_::ChatState;
use crate::routes::api::user::comments_dto::{Comments, CommentRequest};
use crate::routes::api::user::user_dto::User;
use crate::utils::auth::{Claims, verify_jwt};
use chrono::Utc;
use sqlx::MySqlPool;
use sqlx::FromRow;
use serde::Serialize;
use chrono::NaiveDateTime;


pub async fn comments_post_handler(
    State(state): State<Arc<ChatState>>,
    cookies: CookieJar,
    Json(payload): Json<CommentRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token_value = match cookies.get("token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            eprintln!("No token cookie found");
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
                Json(json!({ "message": "Invalid token" })),
            )
                .into_response();
        }
    };
    let user_id = decoded_claims.user_id;

    if payload.content.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Content is required" })),
        )
            .into_response();
    }
    let Some(post_id) = payload.post_id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Post ID is required" })),
        )
            .into_response();
    };

    let new_comment_id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().naive_utc();

    match sqlx::query(
        "INSERT INTO Comments (id, postId, content, userId, authorId, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&new_comment_id)
    .bind(&post_id)
    .bind(&payload.content)
    .bind(&user_id)
    .bind(&user_id) // Assuming authorId is same as userId
    .bind(&created_at)
    .execute(db_pool.as_ref())
    .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "comment": {
                    "id": new_comment_id,
                    "postId": post_id,
                    "content": payload.content,
                    "created_at": created_at,
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

#[derive(Debug, serde::Serialize, FromRow)]
struct CommentWithUser {
    id: String,
    #[serde(rename = "postId")]
    post_id: String,
    content: String,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "authorId")]
    author_id: String,
    created_at: NaiveDateTime,
    #[serde(rename = "user_id")]
    user_id_from_join: Option<String>, // Add this field to capture the User.id from the join
    #[serde(rename = "user_name")]
    user_name: Option<String>,
    #[serde(rename = "user_image")]
    user_image: Option<String>,
}

pub async fn comments_get_handler(
    Query(params): Query<CommentRequest>,
    State(state): State<Arc<ChatState>>,
) -> Response {
    let db_pool = &state.pool;

    let Some(post_id) = params.post_id else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Post ID is required" })),
        )
            .into_response();
    };

    match sqlx::query_as::<_, CommentWithUser>(
        r#"
        SELECT
            Comments.id, Comments.postId, Comments.content, Comments.userId, Comments.authorId, Comments.created_at,
            User.id as user_id, User.name as user_name, User.image as user_image
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
        Ok(comments_with_user) => {
            let formatted_comments: Vec<serde_json::Value> = comments_with_user.into_iter().map(|c| {
                json!({
                    "id": c.id,
                    "postId": c.post_id,
                    "content": c.content,
                    "created_at": c.created_at,
                    "user_id": c.user_id_from_join,
                    "user_name": c.user_name,
                    "user_image": c.user_image,
                })
            }).collect();
            (StatusCode::OK, Json(json!({ "comments": formatted_comments }))).into_response()
        },
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
    cookies: CookieJar,
    Json(payload): Json<CommentRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token_value = match cookies.get("token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            eprintln!("No token cookie found");
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
                Json(json!({ "message": "Invalid token" })),
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
    if content.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Content is required" })),
        )
            .into_response();
    }


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
        "UPDATE Comments SET content = ? WHERE id = ? AND userId = ?"
    )
    .bind(&content)
    .bind(&id)
    .bind(&user_id)
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
    cookies: CookieJar,
    Json(payload): Json<CommentRequest>,
) -> Response {
    let db_pool = &state.pool;
    let jwt_secret = &state.jwt_secret;

    let token_value = match cookies.get("token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            eprintln!("No token cookie found");
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
                Json(json!({ "message": "Invalid token" })),
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

    match sqlx::query("DELETE FROM Comments WHERE id = ? AND userId = ?")
        .bind(&id)
        .bind(&user_id)
        .execute(db_pool.as_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "message": "Comment not found or user not authorized" })),
                )
                    .into_response();
            }
            (
                StatusCode::OK,
                Json(json!({ "message": "Comment deleted successfully!" })),
            )
                .into_response()
        },
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
