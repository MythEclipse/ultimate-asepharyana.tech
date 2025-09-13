//! Handler for the uploader endpoint.

use axum::{
  extract::{ Multipart, Path },
  http::{ header, HeaderMap, StatusCode },
  response::{ IntoResponse, Response },
  routing::{ get, post },
  Json,
  Router,
};
use reqwest::multipart;
use serde::{ Deserialize, Serialize };
use std::sync::Arc;
use crate::routes::AppState;
use utoipa::ToSchema;

pub const ENDPOINT_PATH: &str = "/{file_name}";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "uploader";

const PRODUCTION_URL: &str = "https://asepharyana.tech";
const MAX_FILE_SIZE: u64 = 1024 * 1024 * 1024; // 1GB
const POMF2_URL: &str = "https://pomf2.lain.la";

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UploadResponse {
  pub url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ErrorResponse {
  pub error: String,
}

#[utoipa::path(
  post,
  path = "/api/uploader",
  tag = "uploader",
  operation_id = "upload_file",
  request_body(
    content = String,
    description = "File to upload",
    content_type = "multipart/form-data"
  ),
  responses(
    (status = 200, description = "File uploaded successfully", body = UploadResponse),
    (status = 400, description = "Bad Request", body = ErrorResponse),
    (status = 500, description = "Internal Server Error", body = ErrorResponse)
  )
)]
pub async fn upload_file(
  mut multipart: Multipart
) -> Result<Json<UploadResponse>, (StatusCode, Json<ErrorResponse>)> {
  let mut file_data = None;

  while
    let Some(field) = multipart.next_field().await.map_err(|e| {
      (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
          error: format!("Failed to read multipart: {}", e),
        }),
      )
    })?
  {
    if field.name() == Some("file") {
      let data = field.bytes().await.map_err(|e| {
        (
          StatusCode::BAD_REQUEST,
          Json(ErrorResponse {
            error: format!("Failed to read file: {}", e),
          }),
        )
      })?;

      if (data.len() as u64) > MAX_FILE_SIZE {
        return Err((
          StatusCode::BAD_REQUEST,
          Json(ErrorResponse {
            error: "File size exceeds 1GB".to_string(),
          }),
        ));
      }

      file_data = Some(data);
      break;
    }
  }

  let file_data = file_data.ok_or((
    StatusCode::BAD_REQUEST,
    Json(ErrorResponse {
      error: "No file uploaded".to_string(),
    }),
  ))?;

  let upload_result = upload_to_pomf2(&file_data).await.map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Upload failed: {}", e),
      }),
    )
  })?;

  let formatted_url = format!("{}/api/uploader/{}", PRODUCTION_URL, upload_result.file_name);

  Ok(Json(UploadResponse { url: formatted_url }))
}

pub async fn download_file(Path(file_name): Path<String>) -> Result<
  Response,
  (StatusCode, Json<ErrorResponse>)
> {
  if file_name.is_empty() {
    return Err((
      StatusCode::NOT_FOUND,
      Json(ErrorResponse {
        error: "File not found".to_string(),
      }),
    ));
  }

  let client = reqwest::Client::new();
  let url = format!("{}/f/{}", POMF2_URL, file_name);

  let response = client
    .get(&url)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 Edg/139.0.0.0")
    .send().await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          error: format!("Failed to fetch file: {}", e),
        }),
      )
    })?;

  if response.status() != reqwest::StatusCode::OK {
    return Err((
      StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
      Json(ErrorResponse {
        error: "Failed to fetch file".to_string(),
      }),
    ));
  }

  let content_type = response
    .headers()
    .get("content-type")
    .and_then(|v| v.to_str().ok())
    .unwrap_or("application/octet-stream")
    .to_string();

  let bytes = response.bytes().await.map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Failed to read response: {}", e),
      }),
    )
  })?;

  let mut headers = HeaderMap::new();
  headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
  headers.insert(
    header::CONTENT_DISPOSITION,
    format!("attachment; filename=\"{}\"", file_name).parse().unwrap()
  );
  headers.insert(header::CONTENT_LENGTH, bytes.len().to_string().parse().unwrap());
  headers.insert(header::CACHE_CONTROL, "public, max-age=31536000".parse().unwrap());

  Ok((StatusCode::OK, headers, bytes.to_vec()).into_response())
}

async fn upload_to_pomf2(
  buffer: &[u8]
) -> Result<UploadResult, Box<dyn std::error::Error + Send + Sync>> {
  let file_type = infer
    ::get(buffer)
    .unwrap_or_else(|| {
      infer::Type::new(infer::MatcherType::App, "bin", "application/octet-stream", |_| false)
    });

  let file_name = format!(
    "upload_{}.{}",
    chrono::Utc::now().timestamp_millis(),
    file_type.extension()
  );

  let client = reqwest::Client::new();
  let form = multipart::Form
    ::new()
    .part(
      "files[]",
      multipart::Part
        ::bytes(buffer.to_vec())
        .file_name(file_name.clone())
        .mime_str(file_type.mime_type())?
    );

  let response = client
    .post(format!("{}/upload.php", POMF2_URL))
    .multipart(form)
    .header("Accept", "*/*")
    .header("Origin", POMF2_URL)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 Edg/139.0.0.0")
    .timeout(std::time::Duration::from_secs(600))
    .send().await?;

  let json: serde_json::Value = response.json().await?;

  if !json["success"].as_bool().unwrap_or(false) {
    return Err(
      format!("Upload failed: {}", json["error"].as_str().unwrap_or("Unknown error")).into()
    );
  }

  let uploaded_file_name = json["files"][0]["url"]
    .as_str()
    .and_then(|url| url.split('/').next_back())
    .unwrap_or(&file_name);

  Ok(UploadResult {
    url: json["files"][0]["url"].as_str().unwrap_or("").to_string(),
    file_name: uploaded_file_name.to_string(),
  })
}

#[derive(Debug)]
struct UploadResult {
  #[allow(dead_code)]
  url: String,
  file_name: String,
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
  // This register_routes is manually maintained for multiple handlers
  // build.rs doesn't handle multiple handlers in one file yet
  router
    .route(ENDPOINT_PATH, post(upload_file))
    .route("/api/uploader/{file_name}", get(download_file))
}
