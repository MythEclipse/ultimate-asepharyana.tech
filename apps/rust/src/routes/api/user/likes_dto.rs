use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Likes {
    pub id: String,
    #[sqlx(rename = "postId")]
    pub post_id: String,
    #[sqlx(rename = "userId")]
    pub user_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikeRequest {
    pub post_id: String,
}
