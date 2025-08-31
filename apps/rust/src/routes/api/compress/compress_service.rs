use image::ImageFormat;
use std::io::Cursor;
use sha1::{Sha1, Digest};
use tokio::fs;
use std::env;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio_util::bytes::Bytes;
use crate::routes::api::uploader::upload_to_pomf2;

#[allow(dead_code)]
const CACHE_DIR: &str = "compress-cache"; // Relative to current working directory
#[allow(dead_code)]
const CACHE_EXPIRY_SECONDS: u64 = 3600; // 1 hour

#[allow(dead_code)]
fn generate_cache_key(url: &str, size_param: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_bytes());
    hasher.update(size_param.as_bytes());
    format!("{:x}.cache", hasher.finalize())
}

#[allow(dead_code)]
async fn get_cache_path(cache_key: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let cache_dir = current_dir.join(CACHE_DIR);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).await.expect("Failed to create cache directory");
    }
    cache_dir.join(cache_key)
}

#[allow(dead_code)]
async fn read_from_cache(cache_key: &str) -> Option<Vec<u8>> {
    let cache_path = get_cache_path(cache_key).await;
    if cache_path.exists() {
        let metadata = fs::metadata(&cache_path).await.ok()?;
        let modified_time = metadata.modified().ok()?;
        let now = SystemTime::now();
        let duration = now.duration_since(modified_time).ok()?;

        if duration.as_secs() < CACHE_EXPIRY_SECONDS {
            return fs::read(&cache_path).await.ok();
        }
    }
    None
}

#[allow(dead_code)]
async fn write_to_cache(cache_key: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = get_cache_path(cache_key).await;
    fs::write(&cache_path, data).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn compress_image_from_url(
    _url: &str,
    _size_param: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let cache_key = generate_cache_key(_url, _size_param);

    if let Some(_cached_data) = read_from_cache(&cache_key).await {
        // Return CDN link for cached data by uploading cached file if not already uploaded
        match upload_to_pomf2(Bytes::from(_cached_data)).await {
            Ok((cdn_url, _file_name)) => return Ok(cdn_url),
            Err(e) => return Err(format!("CDN upload failed for cached data: {}", e).into()),
        }
    }

    let client = reqwest::Client::new();
    let response = client.get(_url).send().await?.bytes().await?;
    let original_buffer = response.to_vec();

    let is_percentage = _size_param.ends_with('%');
    let size_value = _size_param.trim_end_matches('%').parse::<f64>()?;

    let img = image::ImageReader::new(Cursor::new(&original_buffer))
        .with_guessed_format()?
        .decode()?;

    let original_size_kb = original_buffer.len() as f64 / 1024.0;
    let target_kb = if is_percentage {
        (original_size_kb * size_value) / 100.0
    } else {
        size_value
    };

    let mut quality = 85;
    let mut best_buffer = original_buffer.clone();

    for _i in 0..8 {
        let mut compressed_buffer = Cursor::new(Vec::new());
        img.write_to(&mut compressed_buffer, ImageFormat::Jpeg)?; // Always compress to JPEG for now
        let compressed_bytes = compressed_buffer.into_inner();
        let size_kb = compressed_bytes.len() as f64 / 1024.0;

        if size_kb > target_kb * 1.05 {
            quality = (quality as f64 * 0.95) as u8; // Reduce quality
        } else if size_kb < target_kb * 0.95 {
            quality = (quality as f64 * 1.05) as u8; // Increase quality
            best_buffer = compressed_bytes;
        } else {
            best_buffer = compressed_bytes;
            break;
        }
    }

    write_to_cache(&cache_key, &best_buffer).await?;
    // Upload to CDN and return CDN link
    match upload_to_pomf2(Bytes::from(best_buffer)).await {
        Ok((cdn_url, _file_name)) => Ok(cdn_url),
        Err(e) => Err(format!("CDN upload failed: {}", e).into()),
    }
}

use ffmpeg_next as ffmpeg;

#[allow(dead_code)]
pub async fn compress_video_from_url(
    _url: &str,
    _size_param: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Download video
    let client = reqwest::Client::new();
    let response = client.get(_url).send().await?.bytes().await?;
    let original_buffer = response.to_vec();

    // Write to temp file
    let mut input_file = tempfile::NamedTempFile::new()?;
    use std::io::Write;
    input_file.write_all(&original_buffer)?;

    let input_path = input_file.path().to_owned();
    let output_file = tempfile::NamedTempFile::new()?;
    let output_path = output_file.path().to_owned();

    // Parse size_param for target bitrate (e.g., "1000k" or percent)
    let target_bitrate = if _size_param.ends_with('k') {
        _size_param.to_string()
    } else if _size_param.ends_with('%') {
        // For percent, fallback to 1000k as a safe default
        "1000k".to_string()
    } else {
        "1000k".to_string()
    };

    // Initialize ffmpeg
    ffmpeg::init().map_err(|e| format!("ffmpeg init failed: {e}"))?;

    // Build ffmpeg command
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-i")
        .arg(input_path.to_str().unwrap())
        .arg("-b:v")
        .arg(&target_bitrate)
        .arg("-y")
        .arg(output_path.to_str().unwrap());

    // Run ffmpeg
    let status = cmd.status().map_err(|e| format!("ffmpeg failed: {e}"))?;
    if !status.success() {
        return Err("ffmpeg compression failed".into());
    }

    // Read compressed video
    let compressed_data = std::fs::read(&output_path)?;

    // Upload to CDN
    match upload_to_pomf2(Bytes::from(compressed_data)).await {
        Ok((cdn_url, _file_name)) => Ok(cdn_url),
        Err(e) => Err(format!("CDN upload failed: {}", e).into()),
    }
}
