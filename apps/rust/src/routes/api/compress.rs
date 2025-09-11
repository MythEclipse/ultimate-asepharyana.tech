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
use std::time::{ SystemTime, UNIX_EPOCH };
use std::io::Cursor;
use image::ImageFormat;
use tokio::fs; // Use tokio's fs for async operations
use std::path::PathBuf;
use uuid;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/compress";
pub const ENDPOINT_DESCRIPTION: &str = "Compress images and videos from URL";
pub const ENDPOINT_TAG: &str = "compress";
pub const OPERATION_ID: &str = "compress";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<CompressResponse>";

lazy_static::lazy_static! {
  static ref CACHE_DIR: PathBuf = {
    let mut path = std::env::temp_dir();
    path.push("compress-cache");
    path
  };
  static ref CACHE_EXPIRY: u64 = 0; // 0 for debugging, forces cache invalidation
  static ref MAX_QUEUE_SIZE: usize = 10;
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

#[derive(Deserialize, ToSchema)]
pub struct CompressQuery {
  pub url: String,
  pub size: String,
}

#[derive(Debug)]
enum SizeUnit {
    Percentage,
    KB,
    MB,
}

fn parse_size_param(
    size_param: &str
) -> Result<(f64, SizeUnit), Box<dyn std::error::Error + Send + Sync>> {
    let s = size_param.trim();
    tracing::info!("parse_size_param: input '{}'", size_param);
    tracing::info!("parse_size_param: trimmed input '{}'", s);

    if s.ends_with('%') {
        let trimmed_s = s.trim_end_matches('%').trim();
        tracing::info!("parse_size_param: percentage trimmed_s '{}'", trimmed_s);
        if trimmed_s.is_empty() {
            tracing::error!("parse_size_param: Numeric value missing for percentage for input '{}'", size_param);
            return Err("Invalid size format. Numeric value missing for percentage.".into());
        }
        let value = trimmed_s.parse::<f64>()?;
        tracing::info!("parse_size_param: parsed value {} for percentage", value);
        Ok((value, SizeUnit::Percentage))
    } else if s.to_lowercase().ends_with("mb") {
        let lower_s = s.to_lowercase();
        let trimmed_s = lower_s.trim_end_matches("mb").trim();
        tracing::info!("parse_size_param: MB trimmed_s '{}'", trimmed_s);
        if trimmed_s.is_empty() {
            tracing::error!("parse_size_param: Numeric value missing for MB for input '{}'", size_param);
            return Err("Invalid size format. Numeric value missing for MB.".into());
        }
        let value = trimmed_s.parse::<f64>()?;
        tracing::info!("parse_size_param: parsed value {} for MB", value);
        Ok((value, SizeUnit::MB))
    } else if s.to_lowercase().ends_with("kb") {
        let lower_s = s.to_lowercase();
        let trimmed_s = lower_s.trim_end_matches("kb").trim();
        tracing::info!("parse_size_param: KB trimmed_s '{}'", trimmed_s);
        if trimmed_s.is_empty() {
            tracing::error!("parse_size_param: Numeric value missing for KB for input '{}'", size_param);
            return Err("Invalid size format. Numeric value missing for KB.".into());
        }
        let value = trimmed_s.parse::<f64>()?;
        tracing::info!("parse_size_param: parsed value {} for KB", value);
        Ok((value, SizeUnit::KB))
    } else {
        tracing::error!("parse_size_param: Invalid size format for input '{}'", size_param);
        Err("Invalid size format. Please specify percentage (e.g., '50%'), MB (e.g., '5MB'), or KB (e.g., '500KB').".into())
    }
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
  target_bytes: f64,
  cache_key: &str
) -> Result<(Vec<u8>, f64), Box<dyn std::error::Error + Send + Sync>> {
  tracing::info!("Starting image compression for cache key: {}", cache_key);
  fs::create_dir_all(CACHE_DIR.as_path()).await?;
  let cache_path = CACHE_DIR.join(cache_key);

  // Check cache
  if cache_path.exists() {
    tracing::info!("Cache hit for image: {}", cache_key);
    if let Ok(metadata) = fs::metadata(&cache_path).await {
      if let Ok(modified) = metadata.modified()?.duration_since(UNIX_EPOCH) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        if now.as_millis() - modified.as_millis() < (*CACHE_EXPIRY as u128) {
          let cached = fs::read(&cache_path).await?;
          if cached.is_empty() {
            tracing::warn!("Cached image file is empty, invalidating cache and re-compressing.");
            fs::remove_file(&cache_path).await?; // Invalidate empty cache
          } else {
            let size_reduction =
              (((buffer.len() - cached.len()) as f64) / (buffer.len() as f64)) * 100.0;
            tracing::info!("Returning cached image for cache key: {}", cache_key);
            return Ok((cached, size_reduction));
          }
        }
      }
    }
  }

  // Load image
  let img = image::load_from_memory(buffer)?;
  let mut quality = 85;
  let mut best_buffer = Vec::new();

  for i in 0..8 {
    tracing::info!("Attempt {} for image compression, current quality: {}", i + 1, quality);
    let mut output = Cursor::new(Vec::new());
    img.write_to(&mut output, ImageFormat::Jpeg)?;
    let output_vec = output.into_inner();
    let size_bytes = output_vec.len() as f64;

    if size_bytes > target_bytes * 1.05 {
      quality = ((quality as f64) * 0.9) as u8;
    } else if size_bytes < target_bytes * 0.95 {
      quality = ((quality as f64) * 1.1).min(100.0) as u8;
      best_buffer = output_vec;
    } else {
      fs::write(&cache_path, &output_vec).await?;
      let size_reduction =
        (((buffer.len() - output_vec.len()) as f64) / (buffer.len() as f64)) * 100.0;
      tracing::info!("Image compressed successfully for cache key: {}", cache_key);
      return Ok((output_vec, size_reduction));
    }
  }

  fs::write(&cache_path, &best_buffer).await?;
  let size_reduction =
    (((buffer.len() - best_buffer.len()) as f64) / (buffer.len() as f64)) * 100.0;
  tracing::info!("Image compression finished after all attempts for cache key: {}", cache_key);
  Ok((best_buffer, size_reduction))
}

#[cfg(feature = "ffmpeg")]
async fn compress_video(
  buffer: &[u8],
  target_bytes: f64,
  _original_bytes: f64,
  cache_key: &str,
  ext: &str
) -> Result<(Vec<u8>, f64), Box<dyn std::error::Error + Send + Sync>> {
  tracing::info!("Starting video compression for cache key: {}", cache_key);
  fs::create_dir_all(CACHE_DIR.as_path()).await?;
  let cache_path = CACHE_DIR.join(cache_key);

  // Check cache
  if cache_path.exists() {
    tracing::info!("Cache hit for video: {}", cache_key);
    if let Ok(metadata) = fs::metadata(&cache_path).await {
      if let Ok(modified) = metadata.modified()?.duration_since(UNIX_EPOCH) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        if now.as_millis() - modified.as_millis() < (*CACHE_EXPIRY as u128) {
          let cached = fs::read(&cache_path).await?;
          if cached.is_empty() {
            tracing::warn!("Cached video file is empty, invalidating cache and re-compressing.");
            fs::remove_file(&cache_path).await?; // Invalidate empty cache
          } else {
            let size_reduction =
              (((buffer.len() - cached.len()) as f64) / (buffer.len() as f64)) * 100.0;
            tracing::info!("Returning cached video for cache key: {}", cache_key);
            return Ok((cached, size_reduction));
          }
        }
      }
    }
  }

  // Create temporary input and output files with the correct extension
  let temp_input = tempfile::Builder::new()
    .suffix(&format!(".{}", ext))
    .tempfile_in(CACHE_DIR.as_path())?;
  fs::write(temp_input.path(), buffer).await?;

  let output_filename = format!("ffmpeg_output_{}.{}", uuid::Uuid::new_v4(), ext);
  let temp_output_path = CACHE_DIR.join(&output_filename);

  // Get video duration
  #[cfg(target_os = "windows")]
  #[cfg(target_os = "windows")]
  let duration_output = std::process::Command::new("cmd")
      .arg("/C")
      .arg("ffmpeg")
      .arg("-i")
      .arg(temp_input.path())
      .current_dir(CACHE_DIR.as_path())
      .output()?;

  #[cfg(not(target_os = "windows"))]
  let duration_output = std::process::Command::new("ffmpeg")
      .arg("-i")
      .arg(temp_input.path())
      .current_dir(CACHE_DIR.as_path())
      .output()?;
  let stderr = String::from_utf8_lossy(&duration_output.stderr);
  let duration_regex = regex::Regex::new(r"Duration: (\d{2}):(\d{2}):(\d{2})\.(\d{2})")?;
  let caps = duration_regex.captures(&stderr).ok_or("Could not find duration in ffmpeg output")?;
  let hours: f64 = caps[1].parse()?;
  let minutes: f64 = caps[2].parse()?;
  let seconds: f64 = caps[3].parse()?;
  let centi_seconds: f64 = caps[4].parse()?;
  let total_seconds = hours * 3600.0 + minutes * 60.0 + seconds + centi_seconds / 100.0;

  // Calculate target bitrate (bits per second)
  // target_bytes is in bytes, convert to bits
  let target_bits = target_bytes * 8.0;
  let target_bitrate = target_bits / total_seconds; // bits per second

  // First Pass
  let log_file_path = CACHE_DIR.join(format!("{}_passlog", cache_key));
  #[cfg(target_os = "windows")]
  #[cfg(target_os = "windows")]
  let status_pass1 = std::process::Command::new("cmd")
      .arg("/C")
      .arg("ffmpeg")
      .arg("-y") // Overwrite output files without asking
      .arg("-i")
      .arg(temp_input.path())
      .arg("-c:v")
      .arg("libx264")
      .arg("-b:v")
      .arg(format!("{:.0}", target_bitrate)) // Target bitrate for video
      .arg("-pass")
      .arg("1")
      .arg("-preset")
      .arg("medium")
      .arg("-an") // No audio
      .arg("-f")
      .arg("mp4") // Output format for pass 1 (can be null or dummy)
      .arg(format!("{}", log_file_path.display())) // Output to log file (dummy)
      .current_dir(CACHE_DIR.as_path())
      .output()?;

  #[cfg(not(target_os = "windows"))]
  let status_pass1 = std::process::Command::new("ffmpeg")
      .arg("-y") // Overwrite output files without asking
      .arg("-i")
      .arg(temp_input.path())
      .arg("-c:v")
      .arg("libx264")
      .arg("-b:v")
      .arg(format!("{:.0}", target_bitrate)) // Target bitrate for video
      .arg("-pass")
      .arg("1")
      .arg("-preset")
      .arg("medium")
      .arg("-an") // No audio
      .arg("-f")
      .arg("mp4") // Output format for pass 1 (can be null or dummy)
      .arg(format!("{}", log_file_path.display())) // Output to log file (dummy)
      .current_dir(CACHE_DIR.as_path())
      .output()?;

  if !status_pass1.status.success() {
      return Err(format!("FFmpeg pass 1 failed: {}", String::from_utf8_lossy(&status_pass1.stderr)).into());
  }

  // Second Pass
  #[cfg(target_os = "windows")]
  #[cfg(target_os = "windows")]
  let status_pass2 = std::process::Command::new("cmd")
      .arg("/C")
      .arg("ffmpeg")
      .arg("-y") // Overwrite output files without asking
      .arg("-i")
      .arg(temp_input.path())
      .arg("-c:v")
      .arg("libx264")
      .arg("-b:v")
      .arg(format!("{:.0}", target_bitrate)) // Target bitrate for video
      .arg("-pass")
      .arg("2")
      .arg("-preset")
      .arg("medium")
      .arg("-c:a")
      .arg("aac")
      .arg(&temp_output_path)
      .current_dir(CACHE_DIR.as_path())
      .output()?;

  #[cfg(not(target_os = "windows"))]
  let status_pass2 = std::process::Command::new("ffmpeg")
      .arg("-y") // Overwrite output files without asking
      .arg("-i")
      .arg(temp_input.path())
      .arg("-c:v")
      .arg("libx264")
      .arg("-b:v")
      .arg(format!("{:.0}", target_bitrate)) // Target bitrate for video
      .arg("-pass")
      .arg("2")
      .arg("-preset")
      .arg("medium")
      .arg("-c:a")
      .arg("aac")
      .arg(&temp_output_path)
      .current_dir(CACHE_DIR.as_path())
      .output()?;

  if !status_pass2.status.success() {
      return Err(format!("FFmpeg pass 2 failed: {}", String::from_utf8_lossy(&status_pass2.stderr)).into());
  }

  // Clean up log file
  fs::remove_file(&log_file_path).await?;
  fs::remove_file(CACHE_DIR.join("ffmpeg2pass-0.log")).await?; // ffmpeg creates this by default
  fs::remove_file(CACHE_DIR.join("ffmpeg2pass-0.log.mbtree")).await?; // and this

  let result_buffer = fs::read(&temp_output_path).await?;
  let _actual_bytes = result_buffer.len() as f64;
  let size_reduction =
    (((buffer.len() - result_buffer.len()) as f64) / (buffer.len() as f64)) * 100.0;
  tracing::info!("Video compressed successfully for cache key: {}", cache_key);
  fs::remove_file(&temp_output_path).await?; // Cleanup
  Ok((result_buffer, size_reduction))
}

#[cfg(not(feature = "ffmpeg"))]
async fn compress_video(
  _buffer: &[u8],
  _target_bytes: f64,
  _original_bytes: f64,
  _cache_key: &str,
  _ext: &str
) -> Result<(Vec<u8>, f64), Box<dyn std::error::Error + Send + Sync>> {
  tracing::warn!("Video compression attempted but ffmpeg feature is not enabled.");
  Err("Video compression requires ffmpeg feature".into())
}

#[utoipa::path(
    get,
    params(
        ("url" = String, Query, description = "Parameter for resource identification", example = "sample_value"),
        ("size" = String, Query, description = "Parameter for resource identification", example = "sample_value")
    ),
    path = "/api/compress",
    tag = "compress",
    operation_id = "compress",
    responses(
        (status = 200, description = "Compress images and videos from URL", body = CompressResponse),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn compress(Query(params): Query<CompressQuery>) -> impl IntoResponse {
  tracing::info!("Received compress request for URL: {} with size: {}", params.url, params.size);
  // Validate parameters
  if params.url.is_empty() || params.size.is_empty() {
    tracing::warn!("Missing URL or size parameter in compress request.");
    return Json(CompressResponse {
      link: None,
      error: Some("Parameter url dan size diperlukan".to_string()),
    });
  }

  // Check queue size
  {
    let queue = QUEUE.lock().await;
    if queue.len() >= *MAX_QUEUE_SIZE {
      tracing::warn!("Compression queue is full. Current queue size: {}", queue.len());
      return Json(CompressResponse {
        link: None,
        error: Some("Server sibuk, coba lagi nanti".to_string()),
      });
    }
    tracing::info!("Compression queue size: {}", queue.len());
  }

  // Create task
  let url = params.url.clone();
  let size_param = params.size.clone();

  tracing::info!("Processing compression for URL: {}", url);
  match process_compression(url, size_param).await {
    Ok(link) => {
      tracing::info!("Compression successful. Link: {}", link);
      Json(CompressResponse {
        link: Some(link),
        error: None,
      })
    }
    Err(e) => {
      tracing::error!("Compression failed: {}", e);
      Json(CompressResponse {
        link: None,
        error: Some(e.to_string()),
      })
    }
  }
}

async fn process_compression(
  url: String,
  size_param: String
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  tracing::info!("Fetching file from URL: {}", url);
  // Fetch file
  let client = reqwest::Client::new();
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(
    reqwest::header::USER_AGENT,
    reqwest::header::HeaderValue::from_static(
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 Edg/139.0.0.0"
    )
  );
  headers.insert(
    reqwest::header::ACCEPT,
    reqwest::header::HeaderValue::from_static(
      "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
    )
  );

  let response = client
    .get(&url)
    .headers(headers)
    .timeout(std::time::Duration::from_secs(45))
    .send().await?;

  let status = response.status();
  let content_length = response.headers()
    .get(reqwest::header::CONTENT_LENGTH)
    .and_then(|h| h.to_str().ok())
    .unwrap_or("unknown");
  let content_type = response.headers()
    .get(reqwest::header::CONTENT_TYPE)
    .and_then(|h| h.to_str().ok())
    .unwrap_or("unknown")
    .to_string(); // Clone content_type to resolve borrow checker issue

  tracing::info!("HTTP Status: {}", status);
  tracing::info!("Content-Length: {}", content_length);
  tracing::info!("Downloaded file content type: {}", content_type);

  // Check for successful HTTP status before proceeding
  if !status.is_success() {
    tracing::error!("Failed to download file. HTTP Status: {}", status);
    return Err(format!("Failed to download file. HTTP Status: {}", status).into());
  }

  let buffer = response.bytes().await?;
  tracing::info!("Downloaded buffer size: {} bytes", buffer.len());

  // Save original buffer to temporary file for debugging
  let ext = Path::new(&url)
    .extension()
    .and_then(|e| e.to_str())
    .unwrap_or("")
    .to_lowercase();

  // Save original buffer to temporary file for debugging
  let original_filename = format!("original_debug.{}.{}", uuid::Uuid::new_v4(), ext);
  let original_path = CACHE_DIR.join(&original_filename);
  fs::write(&original_path, &buffer).await?;
  tracing::info!("Original buffer saved to: {}", original_path.display());

  // Validate buffer content type for video files using infer
  if ext == "mp4" || ext == "mov" || ext == "avi" {
    let kind = infer::get(&buffer);
    if let Some(k) = kind {
      tracing::info!("Inferred buffer type: {}", k.mime_type());
      if !k.mime_type().starts_with("video/") {
        tracing::error!("Buffer content type mismatch. Expected video/*, got: {}", k.mime_type());
        return Err(format!("Invalid content type for video file: {}", k.mime_type()).into());
      }
    } else {
      tracing::warn!("Could not determine content type from buffer for video file. Proceeding based on extension.");
    }
  }

  tracing::info!("File extension detected: {}", ext);
  let cache_key = generate_cache_key(&url, &size_param);
  let (size_value, unit) = parse_size_param(&size_param)?;
  let original_bytes = buffer.len() as f64;

  let compressed_buffer = match ext.as_str() {
    "jpg" | "jpeg" | "png" => {
      tracing::info!("Compressing image file.");
      let target_bytes = match unit {
          SizeUnit::Percentage => original_bytes * (size_value / 100.0),
          SizeUnit::MB => size_value * 1024.0 * 1024.0,
          SizeUnit::KB => size_value * 1024.0,
      };
      compress_image(&buffer, target_bytes, &cache_key).await?.0
    }
    "mp4" | "mov" | "avi" => {
      tracing::info!("Compressing video file.");
      let target_bytes = match unit {
          SizeUnit::Percentage => original_bytes * (size_value / 100.0),
          SizeUnit::MB => size_value * 1024.0 * 1024.0,
          SizeUnit::KB => size_value * 1024.0,
      };
      compress_video(&buffer, target_bytes, original_bytes, &cache_key, &ext).await?.0
    }
    _ => {
      tracing::warn!("Unsupported file format: {}", ext);
      return Err("Format tidak didukung".into());
    }
  };

  if compressed_buffer.is_empty() {
    tracing::error!("Compressed buffer is empty after processing.");
    return Err("Compressed file is empty".into());
  }

  // Save to local file for debugging
  let filename = format!("compressed_debug.{}.{}", uuid::Uuid::new_v4(), ext);
  let local_path = CACHE_DIR.join(&filename);
  tracing::info!("Saving compressed file to local path: {}", local_path.display());
  fs::write(&local_path, &compressed_buffer).await?;

  // Return the local file path as the link for debugging
  Ok(local_path.to_string_lossy().into_owned())
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(compress))
}
