use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Posts {
    pub id: String,
    pub content: String,
    #[sqlx(rename = "authorId")]
    pub author_id: String,
    #[sqlx(rename = "image_url")]
    pub image_url: String,
    #[sqlx(rename = "userId")]
    pub user_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRequest {
    pub id: Option<String>, // For PUT/DELETE
    pub content: String,
    pub image_url: Option<String>,
}
