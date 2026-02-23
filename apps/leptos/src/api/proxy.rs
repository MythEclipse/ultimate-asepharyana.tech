use crate::api::API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditImageCacheRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditImageCacheResponse {
    pub success: bool,
    pub original_url: String,
    pub cdn_url: Option<String>,
    pub was_accessible: bool,
    pub re_uploaded: bool,
    pub message: String,
}

pub async fn audit_image_cache(url: String) -> Result<AuditImageCacheResponse, String> {
    let client = Client::new();
    let api_url = format!("{}/proxy/image-cache/audit", API_BASE_URL);
    
    let request = AuditImageCacheRequest { url };
    
    let response = client
        .post(&api_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if status.is_success() {
        let res = response
            .json::<AuditImageCacheResponse>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(res)
    } else {
        let err_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Audit failed ({}): {}", status, err_text))
    }
}
