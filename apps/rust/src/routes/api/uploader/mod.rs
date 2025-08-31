use axum::{
    extract::{Multipart, State},
    http::{StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState;
use reqwest::Client;
use tokio_util::bytes::Bytes;

use once_cell::sync::Lazy;
use rust_lib::config::CONFIG_MAP;

static PRODUCTION_URL: Lazy<String> = Lazy::new(|| {
    CONFIG_MAP.get("PRODUCTION_URL").cloned().unwrap_or_else(|| "https://asepharyana.tech".to_string())
});
const MAX_FILE_SIZE: usize = 1 * 1024 * 1024 * 1024; // 1GB

async fn upload_to_pomf2(buffer: Bytes) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file_type = infer::get(&buffer);
    let (ext, mime) = match file_type {
        Some(ft) => (ft.extension(), ft.mime_type()),
        None => ("bin", "application/octet-stream"),
    };

    let client = Client::new();
    let form = reqwest::multipart::Form::new().part(
        "files[]",
        reqwest::multipart::Part::bytes(buffer.to_vec())
            .file_name(format!("upload_{}.{}", chrono::Utc::now().timestamp_millis(), ext))
            .mime_str(mime)?,
    );

    let res = client
        .post("https://pomf2.lain.la/upload.php")
        .multipart(form)
        .header("Accept", "*/*")
        .header("Origin", "https://pomf2.lain.la")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(600)) // 10 minutes
        .send()
        .await?;

    let json_response: serde_json::Value = res.json().await?;

    if !json_response["success"].as_bool().unwrap_or(false) {
        return Err(format!("Upload failed: {}", json_response["error"].as_str().unwrap_or("unknown error")).into());
    }

    let file_url = json_response["files"][0]["url"].as_str().ok_or("File URL not found")?.to_string();
    let file_name = file_url.split('/').last().ok_or("File name not found")?.to_string();

    Ok((file_url, file_name))
}

#[utoipa::path(
    post,
    path = "/api/uploader/",
    request_body = Multipart,
    responses(
        (status = 200, description = "File uploaded successfully", body = inline),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Uploader"
)]
#[axum::debug_handler]
#[allow(dead_code)]
pub async fn uploader_post_handler(
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
    mut multipart: Multipart,
) -> Response {
    let mut file_bytes: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("file") {
            let data = field.bytes().await.unwrap_or_default();
            if data.len() > MAX_FILE_SIZE {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "File size exceeds 1GB" })),
                )
                    .into_response();
            }
            file_bytes = Some(data);
            break;
        }
    }

    let Some(buffer) = file_bytes else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No file uploaded" })),
        )
            .into_response();
    };

    match upload_to_pomf2(buffer).await {
        Ok((_file_url, file_name)) => {
            let formatted_url = format!("{}/api/uploader/{}", PRODUCTION_URL.as_str(), file_name);
            (
                StatusCode::OK,
                Json(json!({ "url": formatted_url })),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("Uploader POST error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Upload failed: {}", e) })),
            )
                .into_response()
        }
    }
}

use axum::{routing::{post}, Router};

#[allow(dead_code)]
pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", post(uploader_post_handler))
}
