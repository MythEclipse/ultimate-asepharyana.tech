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
use crate::routes::ChatState;
use crate::routes::api::user::posts_dto::{Posts, PostRequest};
use crate::routes::api::user::user_dto::User;
use crate::routes::api::user::comments_dto::Comments;
use crate::routes::api::user::likes_dto::Likes;
use crate::utils::auth::{Claims, verify_jwt};
use chrono::Utc;
use sqlx::MySqlPool;
use sqlx::FromRow;

pub async fn posts_post_handler(
    State(state): State<Arc<ChatState>>,
    cookies: CookieJar,
    Json(payload): Json<PostRequest>,
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
            Json(json!({ "message": "Content is required and must be a string" })),
        )
            .into_response();
    }

    let new_post_id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().naive_utc();
    let image_url = payload.image_url.unwrap_or_default();

    match sqlx::query_as::<_, Posts>(
        "INSERT INTO Posts (id, content, authorId, image_url, userId, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&new_post_id)
    .bind(&payload.content)
    .bind(&user_id)
    .bind(&image_url)
    .bind(&user_id)
    .bind(&created_at)
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(post) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Post created successfully!", "post": post })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error creating post: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to create post" })),
            )
                .into_response()
        }
    }
}

#[derive(Debug, serde::Serialize, FromRow)]
struct PostWithUser {
    id: String,
    content: String,
    #[serde(rename = "authorId")]
    author_id: String,
    image_url: String,
    #[serde(rename = "userId")]
    user_id: String,
    created_at: chrono::NaiveDateTime,
    #[serde(rename = "user_name")]
    user_name: Option<String>,
    #[serde(rename = "user_image")]
    user_image: Option<String>,
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
    created_at: chrono::NaiveDateTime,
    #[serde(rename = "user_name")]
    user_name: Option<String>,
    #[serde(rename = "user_image")]
    user_image: Option<String>,
}

#[derive(Debug, serde::Serialize, FromRow)]
struct LikeData {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "postId")]
    post_id: String,
}


pub async fn posts_get_handler(
    State(state): State<Arc<ChatState>>,
    cookies: CookieJar,
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

    match sqlx::query_as::<_, PostWithUser>(
        r#"
        SELECT
            p.id, p.content, p.authorId, p.image_url, p.userId, p.created_at,
            u.name as user_name, u.image as user_image
        FROM Posts p
        LEFT JOIN User u ON u.id = p.userId
        ORDER BY p.created_at DESC
        "#
    )
    .fetch_all(db_pool.as_ref())
    .await
    {
        Ok(posts) => {
            let mut sanitized_posts = Vec::new();
            for post in posts {
                // Fetch comments for each post
                let comments = sqlx::query_as::<_, CommentWithUser>(
                    r#"
                    SELECT
                        c.id, c.postId, c.content, c.userId, c.authorId, c.created_at,
                        u.name as user_name, u.image as user_image
                    FROM Comments c
                    LEFT JOIN User u ON u.id = c.userId
                    WHERE c.postId = ?
                    ORDER BY c.created_at DESC
                    "#
                )
                .bind(&post.id)
                .fetch_all(db_pool.as_ref())
                .await
                .unwrap_or_default();

                // Fetch likes for each post
                let likes = sqlx::query_as::<_, LikeData>(
                    "SELECT userId, postId FROM Likes WHERE postId = ?"
                )
                .bind(&post.id)
                .fetch_all(db_pool.as_ref())
                .await
                .unwrap_or_default();

                // Manually construct the desired output structure
                let user_info = json!({
                    "id": post.user_id,
                    "name": post.user_name,
                    "image": post.user_image,
                });

                let comments_with_user_info: Vec<serde_json::Value> = comments.into_iter().map(|c| {
                    json!({
                        "id": c.id,
                        "postId": c.post_id,
                        "content": c.content,
                        "created_at": c.created_at,
                        "user": {
                            "id": c.user_id,
                            "name": c.user_name,
                            "image": c.user_image,
                        }
                    })
                }).collect();

                let likes_info: Vec<serde_json::Value> = likes.into_iter().map(|l| {
                    json!({
                        "userId": l.user_id,
                        "postId": l.post_id,
                    })
                }).collect();

                sanitized_posts.push(json!({
                    "id": post.id,
                    "content": post.content,
                    "image_url": post.image_url,
                    "created_at": post.created_at,
                    "user": user_info,
                    "comments": comments_with_user_info,
                    "likes": likes_info,
                }));
            }
            (StatusCode::OK, Json(json!({ "posts": sanitized_posts }))).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch posts" })),
            )
                .into_response()
        }
    }
}

pub async fn posts_put_handler(
    State(state): State<Arc<ChatState>>,
    cookies: CookieJar,
    Json(payload): Json<PostRequest>,
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
            Json(json!({ "message": "Post ID is required" })),
        )
            .into_response();
    };
    let content = payload.content;

    let post = match sqlx::query_as::<_, Posts>("SELECT * FROM Posts WHERE id = ?")
        .bind(&id)
        .fetch_optional(db_pool.as_ref())
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "Post not found" })),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Database error fetching post: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    if post.user_id != user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "User not authorized to edit this post" })),
        )
            .into_response();
    }

    match sqlx::query_as::<_, Posts>(
        "UPDATE Posts SET content = ? WHERE id = ?"
    )
    .bind(&content)
    .bind(&id)
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(updated_post) => (
            StatusCode::OK,
            Json(json!({ "message": "Post updated successfully!", "post": updated_post })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error updating post: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to update post" })),
            )
                .into_response()
        }
    }
}

pub async fn posts_delete_handler(
    State(state): State<Arc<ChatState>>,
    cookies: CookieJar,
    Json(payload): Json<PostRequest>,
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
            Json(json!({ "message": "Post ID is required" })),
        )
            .into_response();
    };

    let post = match sqlx::query_as::<_, Posts>("SELECT * FROM Posts WHERE id = ?")
        .bind(&id)
        .fetch_optional(db_pool.as_ref())
        .await
    {
        Ok(Some(p)) => p,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "Post not found" })),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Database error fetching post: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    if post.user_id != user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "User not authorized to delete this post" })),
        )
            .into_response();
    }

    match sqlx::query("DELETE FROM Posts WHERE id = ?")
        .bind(&id)
        .execute(db_pool.as_ref())
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Post deleted successfully!" })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Error deleting post: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to delete post" })),
            )
                .into_response()
        }
    }
}
