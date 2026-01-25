//! File utilities.

use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Read file contents as string.
pub async fn read_file(path: &str) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path).await?)
}

/// Read file contents as bytes.
pub async fn read_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path).await?)
}

/// Write string to file.
pub async fn write_file(path: &str, contents: &str) -> anyhow::Result<()> {
    fs::write(path, contents).await?;
    Ok(())
}

/// Write bytes to file.
pub async fn write_bytes(path: &str, contents: &[u8]) -> anyhow::Result<()> {
    fs::write(path, contents).await?;
    Ok(())
}

/// Append to file.
pub async fn append_file(path: &str, contents: &str) -> anyhow::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    file.write_all(contents.as_bytes()).await?;
    Ok(())
}

/// Check if file exists.
pub async fn file_exists(path: &str) -> bool {
    fs::metadata(path).await.is_ok()
}

/// Check if path is a directory.
pub async fn is_directory(path: &str) -> bool {
    fs::metadata(path)
        .await
        .map(|m| m.is_dir())
        .unwrap_or(false)
}

/// Create directory (including parents).
pub async fn create_dir(path: &str) -> anyhow::Result<()> {
    fs::create_dir_all(path).await?;
    Ok(())
}

/// Delete file.
pub async fn delete_file(path: &str) -> anyhow::Result<()> {
    fs::remove_file(path).await?;
    Ok(())
}

/// Delete directory recursively.
pub async fn delete_dir(path: &str) -> anyhow::Result<()> {
    fs::remove_dir_all(path).await?;
    Ok(())
}

/// Get file extension.
pub fn get_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Get filename without extension.
pub fn get_filename(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(String::from)
}

/// Get file size in bytes.
pub async fn file_size(path: &str) -> anyhow::Result<u64> {
    let metadata = fs::metadata(path).await?;
    Ok(metadata.len())
}

/// Format file size to human readable string.
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Get MIME type from file extension.
pub fn mime_from_extension(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    }
}
