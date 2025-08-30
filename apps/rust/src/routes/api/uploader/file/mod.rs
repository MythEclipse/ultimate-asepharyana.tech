use axum::{
    extract::{Multipart, Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::routes::ChatState; // Updated path to ChatState
use crate::routes::api::uploader::infer_service as infer;
use reqwest::Client;
use tokio_util::bytes::Bytes;

const PRODUCTION_URL: &str = "https://asepharyana.tech"; // TODO: Load from environment
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

pub async fn uploader_post_handler(
    mut multipart: Multipart,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
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
        Ok((file_url, file_name)) => {
            let formatted_url = format!("{}/api/uploader/{}", PRODUCTION_URL, file_name);
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

pub async fn uploader_get_handler(
    Path(file_name): Path<String>,
    State(_state): State<Arc<ChatState>>, // State is not used here, but kept for consistency
) -> Response {
    if file_name.is_empty() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "File not found" })),
        )
            .into_response();
    }

    let original_url = format!("https://pomf2.lain.la/f/{}", file_name);
    let client = Client::new();

    match client.get(&original_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .send()
        .await
    {
        Ok(res) => {
            let status = res.status();
            let content_type = res.headers().get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();
            let content_length = res.headers().get(reqwest::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("0")
                .to_string();

            let body = res.bytes().await;

            match body {
                Ok(bytes) => {
                    Response::builder()
                        .status(status)
                        .header("Content-Type", content_type)
                        .header("Content-Disposition", format!("attachment; filename=\"{}\"", file_name))
                        .header("Content-Length", content_length)
                        .header("Cache-Control", "public, max-age=31536000")
                        .body(bytes.into())
                        .unwrap_or_else(|e| {
                            eprintln!("Error building response: {:?}", e);
                            StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        })
                }
                Err(e) => {
                    eprintln!("Error reading response body: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "Failed to fetch file" })),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Uploader GET error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch file" })),
            )
                .into_response()
        }
    }
}
