use reqwest::Client;
use tracing::{info, error};
use bytes::Bytes;
use crate::utils::error::AppError;

pub struct ImageProxyResult {
    pub data: Bytes,
    pub content_type: Option<String>,
    pub status: u16,
}

pub async fn image_proxy(url: &str) -> Result<ImageProxyResult, AppError> {
    info!("Attempting to proxy image from: {}", url);

    // Try CDN image v1
    let cdn_response = fetch_cdn_image(url, 1).await?;
    if cdn_response.status == 200 {
        return Ok(cdn_response);
    }

    // Try CDN image v2 if v1 fails
    let cdn_response_v2 = fetch_cdn_image(url, 2).await?;
    if cdn_response_v2.status == 200 {
        return Ok(cdn_response_v2);
    }

    // Fallback to manual fetch if CDN fails
    let manual_response = fetch_manual(url).await?;
    if manual_response.status == 200 {
        return Ok(manual_response);
    }

    // Fallback to upload image if all else fails (requires external uploader service)
    // This part cannot be directly re-implemented without knowing the Rust equivalent of the uploader service
    // For now, return an error if all previous attempts fail.
    error!("All image proxy attempts failed for URL: {}", url);
    Err(AppError::Other("Failed to proxy image after all attempts".to_string()))
}

async fn fetch_cdn_image(url: &str, version: u8) -> Result<ImageProxyResult, AppError> {
    let client = Client::new();
    let encoded_url = urlencoding::encode(url);
    let cdn_url = format!("https://imagecdn.app/v{version}/images/{encoded_url}");

    match client.get(&cdn_url).send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            let content_type = response.headers().get(reqwest::header::CONTENT_TYPE)
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            if status == 200 {
                if let Some(ct) = &content_type {
                    if ct.starts_with("image/") {
                        let data = response.bytes().await?;
                        return Ok(ImageProxyResult { data, content_type, status });
                    } else {
                        error!("CDN URL does not point to an image: {}", url);
                        return Ok(ImageProxyResult { data: Bytes::new(), content_type: None, status: 400 });
                    }
                }
            }
            error!("Failed to fetch image from CDN v{}: {}, Status: {}", version, url, status);
            Ok(ImageProxyResult { data: Bytes::new(), content_type: None, status })
        },
        Err(e) => {
            error!("Internal server error during CDN v{} fetch for {}: {}", version, url, e);
            Err(AppError::ReqwestError(e))
        }
    }
}

async fn fetch_manual(url: &str) -> Result<ImageProxyResult, AppError> {
    let client = Client::new();
    match client.get(url).send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            let content_type = response.headers().get(reqwest::header::CONTENT_TYPE)
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            if status == 200 {
                if let Some(ct) = &content_type {
                    if ct.starts_with("image/") {
                        let data = response.bytes().await?;
                        return Ok(ImageProxyResult { data, content_type, status });
                    } else {
                        error!("URL does not point to an image: {}", url);
                        return Ok(ImageProxyResult { data: Bytes::new(), content_type: None, status: 400 });
                    }
                }
            }
            error!("Failed to fetch image manually from URL: {}, Status: {}", url, status);
            Ok(ImageProxyResult { data: Bytes::new(), content_type: None, status })
        },
        Err(e) => {
            error!("Internal server error during manual fetch for {}: {}", url, e);
            Err(AppError::ReqwestError(e))
        }
    }
}

// The `uploadImage` function from TypeScript relies on a local uploader service
// and `FormData` which is part of web APIs. Re-implementing this would require
// either a Rust equivalent of the uploader service or a different approach for file uploads.
// For now, it's not re-implemented here.
