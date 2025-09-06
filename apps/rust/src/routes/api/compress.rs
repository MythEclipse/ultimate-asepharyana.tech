//! Handler for the compress endpoint.
#![allow(dead_code)]

use axum::{ extract::Query, response::IntoResponse, routing::get, Json, Router };
use std::sync::Arc;
use crate::routes::AppState;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use std::collections::VecDeque;
use tokio::sync::Mutex;
use std::path::Path;
use sha1::{ Sha1, Digest };
use std::fs;
use std::time::{ SystemTime, UNIX_EPOCH };
use tempfile::NamedTempFile;
use rust_lib::utils::cdn::ryzen_cdn;
use std::io::Cursor;
use image::ImageFormat;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/compress";
pub const ENDPOINT_DESCRIPTION: &str = "Compress images and videos from URL";
pub const ENDPOINT_TAG: &str = "compress";
pub const OPERATION_ID: &str = "compress";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<CompressResponse>";

const CACHE_DIR: &str = "/tmp/compress-cache";
const CACHE_EXPIRY: u64 = 3600 * 1000; // 1 hour
const MAX_QUEUE_SIZE: usize = 10;

lazy_static::lazy_static! {
  static ref IS_PROCESSING: Mutex<bool> = Mutex::new(false);
  static ref QUEUE: Mutex<
    VecDeque<
      Box<dyn (FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>) + Send>
    >
  > = Mutex::new(VecDeque::new());
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct CompressResponse {
  /// CDN link to compressed file
  pub link: Option<String>,
  /// Error message if compression failed
  pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct CompressQuery {
  pub url: String,
  pub size: String,
}

fn generate_cache_key(url: &str, size_param: &str) -> String {
  let mut hasher = Sha1::new();
  hasher.update(format!("{}{}", url, size_param));
  format!("{:x}.cache", hasher.finalize())
}

async fn process_next() {
  let mut processing = IS_PROCESSING.lock().await;
  if *processing {
    return;
  }

  let mut queue = QUEUE.lock().await;
  if let Some(task) = queue.pop_front() {
    *processing = true;
    drop(processing);
    drop(queue);

    task().await;

    let mut processing = IS_PROCESSING.lock().await;
    *processing = false;
    // Box the future to prevent infinite size
    Box::pin(process_next()).await;
  }
}

async fn compress_image(
  buffer: &[u8],
  target_kb: f64,
  cache_key: &str
) -> Result<(Vec<u8>, f64), Box<dyn std::error::Error + Send + Sync>> {
  let cache_path = Path::new(CACHE_DIR).join(cache_key);

  // Check cache
  if cache_path.exists() {
    if let Ok(metadata) = fs::metadata(&cache_path) {
      if let Ok(modified) = metadata.modified()?.duration_since(UNIX_EPOCH) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        if now.as_millis() - modified.as_millis() < (CACHE_EXPIRY as u128) {
          let cached = fs::read(&cache_path)?;
          let size_reduction =
            (((buffer.len() - cached.len()) as f64) / (buffer.len() as f64)) * 100.0;
          return Ok((cached, size_reduction));
        }
      }
    }
  }

  // Load image
  let img = image::load_from_memory(buffer)?;
  let mut quality = 85;
  let mut best_buffer = Vec::new();

  for _ in 0..8 {
    let mut output = Cursor::new(Vec::new());
    img.write_to(&mut output, ImageFormat::Jpeg)?;
    let output_vec = output.into_inner();
    let size_kb = (output_vec.len() as f64) / 1024.0;

    if size_kb > target_kb * 1.05 {
      quality = ((quality as f64) * 0.9) as u8;
    } else if size_kb < target_kb * 0.95 {
      quality = ((quality as f64) * 1.1).min(100.0) as u8;
      best_buffer = output_vec;
    } else {
      fs::write(&cache_path, &output_vec)?;
      let size_reduction =
        (((buffer.len() - output_vec.len()) as f64) / (buffer.len() as f64)) * 100.0;
      return Ok((output_vec, size_reduction));
    }
  }

  fs::write(&cache_path, &best_buffer)?;
  let size_reduction =
    (((buffer.len() - best_buffer.len()) as f64) / (buffer.len() as f64)) * 100.0;
  Ok((best_buffer, size_reduction))
}

#[cfg(feature = "ffmpeg")]
async fn compress_video(
  buffer: &[u8],
  target_mb: f64,
  is_percentage: bool,
  original_mb: f64,
  cache_key: &str
) -> Result<(Vec<u8>, f64), Box<dyn std::error::Error + Send + Sync>> {
  let cache_path = Path::new(CACHE_DIR).join(cache_key);

  // Check cache
  if cache_path.exists() {
    if let Ok(metadata) = fs::metadata(&cache_path) {
      if let Ok(modified) = metadata.modified()?.duration_since(UNIX_EPOCH) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        if now.as_millis() - modified.as_millis() < (CACHE_EXPIRY as u128) {
          let cached = fs::read(&cache_path)?;
          let size_reduction =
            (((buffer.len() - cached.len()) as f64) / (buffer.len() as f64)) * 100.0;
          return Ok((cached, size_reduction));
        }
      }
    }
  }

  let temp_input = NamedTempFile::new()?;
  let temp_output = NamedTempFile::new()?;
  fs::write(&temp_input, buffer)?;

  let final_target_mb = if is_percentage { (original_mb * target_mb) / 100.0 } else { target_mb };
  let mut attempts = 0;
  let min_size_mb = final_target_mb - 3.5;
  let max_size_mb = final_target_mb + 0.5;

  loop {
    let _ratio = (final_target_mb / original_mb).max(0.6);
    let crf = (24.0 - (original_mb - final_target_mb) * 0.5).max(18.0).min(32.0) as u32;

    // Use ffmpeg to compress
    let mut command = std::process::Command::new("ffmpeg");
    command
      .arg("-i")
      .arg(temp_input.path())
      .arg("-c:v")
      .arg("libx264")
      .arg("-crf")
      .arg(crf.to_string())
      .arg("-preset")
      .arg("medium")
      .arg("-c:a")
      .arg("aac")
      .arg("-y")
      .arg(temp_output.path());

    command.output()?;

    let result_buffer = fs::read(&temp_output)?;
    let actual_mb = (result_buffer.len() as f64) / 1024.0 / 1024.0;

    if actual_mb < min_size_mb {
      // Too small, increase target
    } else if actual_mb > max_size_mb {
      // Too large, decrease target
    } else {
      fs::write(&cache_path, &result_buffer)?;
      let size_reduction =
        (((buffer.len() - result_buffer.len()) as f64) / (buffer.len() as f64)) * 100.0;
      return Ok((result_buffer, size_reduction));
    }

    attempts += 1;
    if attempts >= 5 {
      fs::write(&cache_path, &result_buffer)?;
      let size_reduction =
        (((buffer.len() - result_buffer.len()) as f64) / (buffer.len() as f64)) * 100.0;
      return Ok((result_buffer, size_reduction));
    }
  }
}

#[cfg(not(feature = "ffmpeg"))]
async fn compress_video(
  _buffer: &[u8],
  _target_mb: f64,
  _is_percentage: bool,
  _original_mb: f64,
  _cache_key: &str
) -> Result<(Vec<u8>, f64), Box<dyn std::error::Error + Send + Sync>> {
  Err("Video compression requires ffmpeg feature".into())
}

#[utoipa::path(
  get,
  path = "/api/compress",
  tag = "compress",
  operation_id = "compress",
  responses(
    (status = 200, description = "Compress images and videos from URL", body = CompressResponse),
    (status = 500, description = "Internal Server Error", body = String)
  )
)]
pub async fn compress(Query(params): Query<CompressQuery>) -> impl IntoResponse {
  // Validate parameters
  if params.url.is_empty() || params.size.is_empty() {
    return Json(CompressResponse {
      link: None,
      error: Some("Parameter url dan size diperlukan".to_string()),
    });
  }

  // Check queue size
  {
    let queue = QUEUE.lock().await;
    if queue.len() >= MAX_QUEUE_SIZE {
      return Json(CompressResponse {
        link: None,
        error: Some("Server sibuk, coba lagi nanti".to_string()),
      });
    }
  }

  // Create task
  let url = params.url.clone();
  let size_param = params.size.clone();

  // This would be the queued task implementation
  // For now, we'll process synchronously for simplicity
  match process_compression(url, size_param).await {
    Ok(link) =>
      Json(CompressResponse {
        link: Some(link),
        error: None,
      }),
    Err(e) =>
      Json(CompressResponse {
        link: None,
        error: Some(e.to_string()),
      }),
  }
}

async fn process_compression(
  url: String,
  size_param: String
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  // Fetch file
  let client = reqwest::Client::new();
  let response = client
    .get(&url)
    .header("User-Agent", "Mozilla/5.0")
    .timeout(std::time::Duration::from_secs(45))
    .send().await?;

  let buffer = response.bytes().await?;
  let ext = Path::new(&url)
    .extension()
    .and_then(|e| e.to_str())
    .unwrap_or("")
    .to_lowercase();

  let cache_key = generate_cache_key(&url, &size_param);
  let is_percentage = size_param.contains('%');
  let size_value: f64 = size_param.replace('%', "").parse()?;

  let compressed_buffer = match ext.as_str() {
    "jpg" | "jpeg" | "png" => {
      let target_kb = if is_percentage {
        ((buffer.len() as f64) / 1024.0) * (size_value / 100.0)
      } else {
        size_value * 1024.0
      };
      compress_image(&buffer, target_kb, &cache_key).await?.0
    }
    "mp4" | "mov" | "avi" => {
      let original_mb = (buffer.len() as f64) / 1024.0 / 1024.0;
      compress_video(&buffer, size_value, is_percentage, original_mb, &cache_key).await?.0
    }
    _ => {
      return Err("Format tidak didukung".into());
    }
  };

  // Upload to CDN
  let filename = format!("compressed.{}", ext);
  let cdn_link = ryzen_cdn(&compressed_buffer, Some(&filename)).await?;

  Ok(cdn_link)
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(compress))
}