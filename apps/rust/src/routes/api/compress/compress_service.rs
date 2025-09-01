//! Compression service for images and videos from a URL.
//! Uses `image` crate for images and `ffmpeg-cli` for videos. Returns a CDN link after upload.

use std::{fs, io::Write, path::PathBuf};
use reqwest::Client;
use tempfile::NamedTempFile;
use sha1::{Sha1, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};
use tokio::process::Command;
use std::io::Cursor;

const CACHE_DIR: &str = "/tmp/compress-cache";
const CACHE_EXPIRY: u64 = 3600; // seconds

// Dummy CDN upload function (replace with real CDN logic)
async fn upload_to_cdn(file_path: &str) -> Result<String> {
    // In production, upload the file and return the CDN URL.
    Ok(format!("https://cdn.example.com/{}", file_path))
}

// Generate a cache key based on URL and size param
fn generate_cache_key(url: &str, size_param: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_bytes());
    hasher.update(size_param.as_bytes());
    format!("{:x}.cache", hasher.finalize())
}

// Check if cache exists and is fresh
fn get_cached_file(cache_key: &str) -> Option<Vec<u8>> {
    let path = PathBuf::from(CACHE_DIR).join(cache_key);
    if let Ok(meta) = fs::metadata(&path) {
        if let Ok(modified) = meta.modified() {
            if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
                if let Ok(mod_secs) = modified.duration_since(UNIX_EPOCH) {
                    if now.as_secs() - mod_secs.as_secs() < CACHE_EXPIRY {
                        return fs::read(&path).ok();
                    }
                }
            }
        }
    }
    None
}

// Save buffer to cache
fn save_to_cache(cache_key: &str, buf: &[u8]) -> Result<()> {
    let path = PathBuf::from(CACHE_DIR).join(cache_key);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(buf)?;
    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/compress/image",
    summary = "Compress image from URL",
    description = "Compresses an image from a given URL to a target size (in KB or %) and returns a CDN link.",
    request_body(
        content = CompressImageRequest,
        description = "Image URL and target size parameter"
    ),
    responses(
        (status = 200, description = "Image compressed and uploaded", body = String),
        (status = 500, description = "Compression or upload failed", body = String)
    ),
    tag = "Compress"
)]
/// Compress an image from a URL to a target size (in KB or %), return CDN link.
pub async fn compress_image_from_url(url: &str, size_param: &str) -> Result<String> {
    let cache_key = generate_cache_key(url, size_param);
    if let Some(cached) = get_cached_file(&cache_key) {
        let tmp = NamedTempFile::new()?;
        tmp.as_file().write_all(&cached)?;
        return upload_to_cdn(tmp.path().to_str().unwrap()).await;
    }

    let client = Client::new();
    let resp = client.get(url).send().await?.bytes().await?;
    let img = image::load_from_memory(&resp).context("Failed to decode image")?;

    let mut quality = 85u8;
    let mut best_buf = Vec::new();
    let orig_len = resp.len() as f64;
    let target_kb = if size_param.contains('%') {
        let pct: f64 = size_param.replace('%', "").parse().unwrap_or(100.0);
        orig_len * (pct / 100.0) / 1024.0
    } else {
        size_param.parse().unwrap_or(100.0)
    };

    for _ in 0..8 {
        let mut buf = Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Jpeg)?;
        let size_kb = buf.get_ref().len() as f64 / 1024.0;
        if size_kb > target_kb * 1.05 {
            quality = quality.saturating_sub(5);
        } else if size_kb < target_kb * 0.95 {
            quality = quality.saturating_add(5);
            best_buf = buf.get_ref().clone();
        } else {
            save_to_cache(&cache_key, buf.get_ref())?;
            let tmp = NamedTempFile::new()?;
            tmp.as_file().write_all(buf.get_ref())?;
            return upload_to_cdn(tmp.path().to_str().unwrap()).await;
        }
    }
    save_to_cache(&cache_key, &best_buf)?;
    let tmp = NamedTempFile::new()?;
    tmp.as_file().write_all(&best_buf)?;
    upload_to_cdn(tmp.path().to_str().unwrap()).await
}

#[utoipa::path(
    post,
    path = "/api/compress/video",
    summary = "Compress video from URL",
    description = "Compresses a video from a given URL to a target size (in MB or %) and returns a CDN link.",
    request_body(
        content = CompressVideoRequest,
        description = "Video URL and target size parameter"
    ),
    responses(
        (status = 200, description = "Video compressed and uploaded", body = String),
        (status = 500, description = "Compression or upload failed", body = String)
    ),
    tag = "Compress"
)]
/// Compress a video from a URL to a target size (in MB or %), return CDN link.
pub async fn compress_video_from_url(url: &str, size_param: &str) -> Result<String> {
    let cache_key = generate_cache_key(url, size_param);
    if let Some(cached) = get_cached_file(&cache_key) {
        let tmp = NamedTempFile::new()?;
        tmp.as_file().write_all(&cached)?;
        return upload_to_cdn(tmp.path().to_str().unwrap()).await;
    }

    let client = Client::new();
    let resp = client.get(url).send().await?.bytes().await?;
    let orig_mb = resp.len() as f64 / 1024.0 / 1024.0;
    let is_pct = size_param.contains('%');
    let size_value: f64 = size_param.replace('%', "").parse().unwrap_or(1.0);
    let target_mb = if is_pct {
        orig_mb * (size_value / 100.0)
    } else {
        size_value
    };

    let input_file = NamedTempFile::new()?;
    input_file.as_file().write_all(&resp)?;
    let output_file = NamedTempFile::new()?;

    // Use ffmpeg to compress video (requires ffmpeg installed)
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file.path())
        .arg("-b:v")
        .arg(format!("{:.0}k", (target_mb * 8192.0).max(500.0)))
        .arg("-y")
        .arg(output_file.path())
        .status()
        .await?;

    if !status.success() {
        anyhow::bail!("ffmpeg failed");
    }

    let out_buf = fs::read(output_file.path())?;
    save_to_cache(&cache_key, &out_buf)?;
    upload_to_cdn(output_file.path().to_str().unwrap()).await
}
