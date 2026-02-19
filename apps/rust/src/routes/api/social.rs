use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, 
    ModelTrait,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::entities::{posts, user, likes, comments};
use crate::routes::AppState;
use crate::core::error::AppError;
use crate::middleware::auth::AuthMiddleware;

// ... DTOs preserved ...
// DTOs
#[derive(Debug, Serialize, ToSchema)]
pub struct PostResponse {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub image_url: Option<String>,
    pub created_at: String,
    pub likes: Vec<LikeResponse>,
    pub comments: Vec<CommentResponse>,
    pub user: Option<UserResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LikeResponse {
    pub user_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommentResponse {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
    pub user: Option<UserResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub content: String,
    pub image_url: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/social/posts",
    tag = "social",
    responses(
        (status = 200, description = "List all posts", body = Vec<PostResponse>),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn get_posts(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    // ... existing implementation ...
    // 1. Fetch all posts
    let posts = posts::Entity::find()
        .order_by_desc(posts::Column::CreatedAt)
        .all(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut response_posts = Vec::new();

    for post in posts {
        let author = user::Entity::find_by_id(&post.author_id)
            .one(state.sea_orm())
            .await
            .unwrap_or(None);

        let likes = likes::Entity::find()
            .filter(likes::Column::PostId.eq(&post.id))
            .all(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let comments_models = comments::Entity::find()
            .filter(comments::Column::PostId.eq(&post.id))
            .all(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut comments = Vec::new();
        for comment in comments_models {
             let comment_author = user::Entity::find_by_id(&comment.author_id)
                .one(state.sea_orm())
                .await
                .unwrap_or(None);

             comments.push(CommentResponse {
                id: comment.id,
                user_id: comment.user_id,
                content: comment.content,
                created_at: comment.created_at.to_string(),
                user: comment_author.map(|u| UserResponse {
                    id: u.id,
                    name: u.name.unwrap_or_default(),
                    image: u.image,
                }),
            });
        }

        let likes_resp = likes.into_iter().map(|l| LikeResponse {
            user_id: l.user_id,
        }).collect();

        response_posts.push(PostResponse {
            id: post.id,
            user_id: post.author_id.clone(),
            content: post.content,
            image_url: post.image_url,
            created_at: post.created_at.to_string(),
            likes: likes_resp,
            comments,
            user: author.map(|u| UserResponse {
                id: u.id,
                name: u.name.unwrap_or_default(),
                image: u.image,
            }),
        });
    }

    Ok(Json(response_posts))
}

#[utoipa::path(
    post,
    path = "/api/social/posts",
    tag = "social",
    security(("bearer_auth" = [])),
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Post created successfully", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    auth: AuthMiddleware, 
    Json(payload): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth.0.user_id;

    let new_post = posts::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        author_id: Set(user_id.clone()),
        user_id: Set(user_id), 
        content: Set(payload.content),
        image_url: Set(payload.image_url),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let _ = new_post.insert(state.sea_orm()).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json("Post created"))
}

#[utoipa::path(
    delete,
    path = "/api/social/posts/{id}",
    tag = "social",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post deleted successfully", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    auth: AuthMiddleware,
    Path(post_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let post = posts::Entity::find_by_id(&post_id)
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Post not found".to_string()))?;

    if post.author_id != auth.0.user_id {
        return Err(AppError::Unauthorized);
    }

    let _ = post.delete(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json("Post deleted"))
}

#[utoipa::path(
    post,
    path = "/api/social/posts/{id}/like",
    tag = "social",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post liked/unliked successfully", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn like_post(
    State(state): State<Arc<AppState>>,
    auth: AuthMiddleware,
    Path(post_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth.0.user_id;

    // Check if like exists
    let existing_like = likes::Entity::find()
        .filter(likes::Column::PostId.eq(&post_id))
        .filter(likes::Column::UserId.eq(&user_id))
        .one(state.sea_orm())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if let Some(like) = existing_like {
        // Unlike
        let _ = like.delete(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Json("Unliked"))
    } else {
         // Like
        let new_like = likes::ActiveModel {
            post_id: Set(post_id),
            user_id: Set(user_id),
            ..Default::default()
        };

        new_like.insert(state.sea_orm())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Json("Liked"))
    }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
}