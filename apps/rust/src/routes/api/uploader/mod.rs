// Handles POST /api/uploader: receives a file via multipart/form-data, uploads it to Pomf2, and returns a formatted URL or error.

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde_json::json;
use std::env;
use reqwest::Client;
use tokio::io::AsyncReadExt;

const MAX_FILE_SIZE: u64 = 1 * 1024 * 1024 * 1024; // 1GB

pub fn router() -> Router {
    Router::new().route("/api/uploader", post(uploader_post))
}

async fn uploader_post(mut multipart: Multipart) -> impl IntoResponse {
    // Extract file part
    let mut file_bytes = Vec::new();
    let mut file_name = None;
    let mut file_size = 0u64;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("file") {
            file_name = field.file_name().map(|s| s.to_string());
            let mut data = field;
            while let Ok(Some(chunk)) = data.chunk().await {
                file_size += chunk.len() as u64;
                if file_size > MAX_FILE_SIZE {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": "File size exceeds 1GB" })),
                    );
                }
                file_bytes.extend_from_slice(&chunk);
            }
            break;
        }
    }

    if file_bytes.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No file uploaded" })),
        );
    }

    // Upload to Pomf2
    let upload_result = upload_to_pomf2(&file_bytes, file_name.as_deref()).await;
    match upload_result {
        Ok((url, uploaded_file_name)) => {
            let production_url = env::var("PRODUCTION_URL")
                .unwrap_or_else(|_| "https://asepharyana.tech".to_string());
            let formatted_url = format!("{}/api/uploader/{}", production_url, uploaded_file_name);
            (
                StatusCode::OK,
                Json(json!({ "url": formatted_url })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Upload failed: {}", e) })),
        ),
    }
}

async fn upload_to_pomf2(
    file_bytes: &[u8],
    orig_file_name: Option<&str>,
) -> Result<(String, String), String> {
    let client = Client::new();
    let ext = orig_file_name
        .and_then(|n| n.split('.').last())
        .unwrap_or("bin");
    let file_name = format!("upload_{}.{}", chrono::Utc::now().timestamp_millis(), ext);

    let form = reqwest::multipart::Form::new().part(
        "files[]",
        reqwest::multipart::Part::bytes(file_bytes.to_vec())
            .file_name(file_name.clone())
            .mime_str("application/octet-stream")
            .unwrap(),
    );

    let res = client
        .post("https://pomf2.lain.la/upload.php")
        .multipart(form)
        .header("Accept", "*/*")
        .header("Origin", "https://pomf2.lain.la")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        )
        .timeout(std::time::Duration::from_secs(600))
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let status = res.status();
    let json: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;

    if !json.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
        return Err(
            json.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Upload failed")
                .to_string(),
        );
    }

    let uploaded_url = json["files"][0]["url"]
        .as_str()
        .ok_or("No URL in response")?
        .to_string();
    let uploaded_file_name = uploaded_url
        .split('/')
        .last()
        .unwrap_or(&file_name)
        .to_string();

    Ok((uploaded_url, uploaded_file_name))
}
