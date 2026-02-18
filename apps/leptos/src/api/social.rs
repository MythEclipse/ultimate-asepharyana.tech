
use serde::{Deserialize, Serialize};
use crate::types::Post;
use crate::api::types::ApiError;
use reqwest::Client;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub image_url: Option<String>,
}

pub async fn get_posts() -> Result<Vec<Post>, ApiError> {
    let client = Client::new();
    let response = client
        .get("http://localhost:4091/api/social/posts")
        .send()
        .await
        .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;

    if response.status().is_success() {
        let posts = response.json::<Vec<Post>>().await
            .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;
        Ok(posts)
    } else {
        let err = response.json::<ApiError>().await
            .unwrap_or_else(|_| ApiError { message: "Unknown error".to_string(), code: None, details: None });
        Err(err)
    }
}

pub async fn create_post(token: String, request: CreatePostRequest) -> Result<String, ApiError> {
    let client = Client::new();
    let response = client
        .post("http://localhost:4091/api/social/posts")
        .bearer_auth(token)
        .json(&request)
        .send()
        .await
        .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;

    if response.status().is_success() {
        let msg = response.json::<String>().await
            .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;
        Ok(msg)
    } else {
        let err = response.json::<ApiError>().await
            .unwrap_or_else(|_| ApiError { message: "Unknown error".to_string(), code: None, details: None });
        Err(err)
    }
}

pub async fn like_post(token: String, post_id: String) -> Result<String, ApiError> {
    let client = Client::new();
    let response = client
        .post(format!("http://localhost:4091/api/social/posts/{}/like", post_id))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;

    if response.status().is_success() {
        let msg = response.json::<String>().await
            .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;
        Ok(msg)
    } else {
        let err = response.json::<ApiError>().await
            .unwrap_or_else(|_| ApiError { message: "Unknown error".to_string(), code: None, details: None });
        Err(err)
    }
}

pub async fn delete_post(token: String, post_id: String) -> Result<String, ApiError> {
    let client = Client::new();
    let response = client
        .delete(format!("http://localhost:4091/api/social/posts/{}", post_id))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;

    if response.status().is_success() {
        let msg = response.json::<String>().await
            .map_err(|e| ApiError { message: e.to_string(), code: None, details: None })?;
        Ok(msg)
    } else {
        let err = response.json::<ApiError>().await
            .unwrap_or_else(|_| ApiError { message: "Unknown error".to_string(), code: None, details: None });
        Err(err)
    }
}
