use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Comments {
    pub id: String,
    #[sqlx(rename = "postId")]
    pub post_id: String,
    pub content: String,
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[sqlx(rename = "authorId")]
    pub author_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRequest {
    pub post_id: Option<String>,
    pub content: String,
    pub id: Option<String>, // For PUT/DELETE requests
}
