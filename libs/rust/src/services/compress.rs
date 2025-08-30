use image::{io::Reader as ImageReader, ImageFormat};
use reqwest::Url;
use std::io::Cursor;
use sha1::{Sha1, Digest};
use std::path::{Path, PathBuf};
use tokio::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

const CACHE_DIR: &str = "compress-cache"; // Relative to current working directory
const CACHE_EXPIRY_SECONDS: u64 = 3600; // 1 hour

fn generate_cache_key(url: &str, size_param: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_bytes());
    hasher.update(size_param.as_bytes());
    format!("{:x}.cache", hasher.finalize())
}

async fn get_cache_path(cache_key: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let cache_dir = current_dir.join(CACHE_DIR);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).await.expect("Failed to create cache directory");
    }
    cache_dir.join(cache_key)
}

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

async fn write_to_cache(cache_key: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = get_cache_path(cache_key).await;
    fs::write(&cache_path, data).await?;
    Ok(())
}

pub async fn compress_image_from_url(
    url: &str,
    size_param: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let cache_key = generate_cache_key(url, size_param);

    if let Some(cached_data) = read_from_cache(&cache_key).await {
        // TODO: Return CDN link for cached data
        return Ok(format!("cached_cdn_link_for_{}", cache_key));
    }

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.bytes().await?;
    let original_buffer = response.to_vec();

    let is_percentage = size_param.ends_with('%');
    let size_value = size_param.trim_end_matches('%').parse::<f64>()?;

    let mut img = ImageReader::new(Cursor::new(&original_buffer))
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
    // TODO: Upload to CDN and return CDN link
    Ok(format!("cdn_link_for_{}", cache_key))
}

pub async fn compress_video_from_url(
    url: &str,
    size_param: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implement video compression using ffmpeg-next
    // This will be significantly more complex due to external process management
    // and handling video streams.
    Err("Video compression not yet implemented".into())
}
