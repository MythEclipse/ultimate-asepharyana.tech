//! # Uploader API
//!
//! This module provides API endpoints for file uploads.
//! It supports multipart form data and integrates with a file hosting service.

use axum::{
    routing::{post, get},
    Router,
    response::IntoResponse,
    Json,
    extract::{Multipart, Path},
};
use serde::{Serialize, Deserialize};
use reqwest::{Client, StatusCode};
use rust_lib::config::CONFIG_MAP;
use crate::routes::ChatState;
use std::sync::Arc;

// Maximum file size allowed for uploads (1GB)
const MAX_FILE_SIZE: u64 = 1 * 1024 * 1024 * 1024;


pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", post(uploader_post))
        .route("/{file_name}", get(uploader_get)) // Example route for retrieving uploaded files
}

#[derive(Debug, Deserialize, Serialize)]
struct UploadResponse {
    success: bool,
    files: Option<Vec<UploadedFile>>,
    message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UploadedFile {
    url: String,
    name: String,
    size: u64,
}

pub async fn uploader_post(mut multipart: Multipart) -> impl IntoResponse {
    let client = Client::new();
    let pomf2_url = CONFIG_MAP.get("POMF2_URL").expect("POMF2_URL not set");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        tracing::info!("Name: {}, FileName: {}, ContentType: {}, Size: {}", name, file_name, content_type, data.len());

        if data.len() as u64 > MAX_FILE_SIZE {
            return (
                StatusCode::BAD_REQUEST,
                Json(UploadResponse {
                    success: false,
                    files: None,
                    message: Some(format!("File size exceeds limit of {} bytes", MAX_FILE_SIZE)),
                }),
            ).into_response();
        }

        let form = reqwest::multipart::Form::new()
            .part("files[]", reqwest::multipart::Part::bytes(data.to_vec()).file_name(file_name.clone()).mime_str(&content_type).unwrap());

        let res = client.post(pomf2_url)
            .multipart(form)
            .send()
            .await
            .expect("Failed to send request to pomf2");

        let status = res.status();
        let text = res.text().await.unwrap_or_default();

        if status.is_success() {
            let upload_response: UploadResponse = serde_json::from_str(&text).unwrap();
            return (StatusCode::OK, Json(upload_response)).into_response();
        } else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadResponse {
                    success: false,
                    files: None,
                    message: Some(format!("Failed to upload to pomf2: {} - {}", status, text)),
                }),
            ).into_response();
        }
    }

    (
        StatusCode::BAD_REQUEST,
        Json(UploadResponse {
            success: false,
            files: None,
            message: Some("No file uploaded".to_string()),
        }),
    ).into_response()
}

pub async fn uploader_get(Path(file_name): Path<String>) -> impl IntoResponse {
    // This is a placeholder. In a real application, you would retrieve the file
    // from storage (e.g., S3, local disk) and return it.
    // For now, we'll just return a dummy response.
    tracing::info!("Attempting to retrieve file: {}", file_name);

    // Simulate file not found for any file other than "example.txt"
    if file_name != "example.txt" {
        return (StatusCode::NOT_FOUND, "File not found".to_string()).into_response();
    }

    // Simulate file content
    let dummy_content = "This is a dummy file content.";
    (StatusCode::OK, dummy_content.to_string()).into_response()
}
