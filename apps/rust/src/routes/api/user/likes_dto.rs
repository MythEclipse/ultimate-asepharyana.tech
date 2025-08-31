use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Likes {
    #[sqlx(rename = "postId")]
    pub post_id: String,
    #[sqlx(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikeRequest {
    pub post_id: String,
}
