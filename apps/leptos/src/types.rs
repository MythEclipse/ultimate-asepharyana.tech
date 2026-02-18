use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Like {
    pub user_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String, // simplified from Date
    pub user: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub image_url: Option<String>,
    pub created_at: String,
    pub likes: Vec<Like>,
    pub comments: Vec<Comment>,
    pub user: Option<User>,
}
