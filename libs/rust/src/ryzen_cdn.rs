use reqwest::{Client, multipart};
use tracing::error;
use serde::{Deserialize, Serialize};
use infer; // For file type detection
use crate::utils::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RyzenCDNResponse {
    pub success: bool,
    pub message: Option<String>,
    pub url: Option<String>,
}

pub async fn ryzen_cdn(
    inp: Vec<u8>, // Simplified input to a single byte vector for now
    original_name: Option<String>,
) -> Result<String, AppError> {
    let client = Client::new();
    let form = multipart::Form::new();

    let file_type = infer::get(&inp);
    let mime_type = file_type.as_ref().map(|t| t.mime_type());
    let extension = file_type.as_ref().map(|t| t.extension());

    let file_name = if let Some(name) = original_name {
        if let Some(ext) = extension {
            format!("{}.{}", name.split('.').next().unwrap_or("file"), ext)
        } else {
            name
        }
    } else {
        "file".to_string()
    };

    let part = multipart::Part::bytes(inp)
        .file_name(file_name)
        .mime_str(mime_type.unwrap_or("application/octet-stream"))?;

    let form = form.part("file", part);

    let res = client.post("https://api.ryzumi.vip/api/uploader/ryzencdn")
        .multipart(form)
        .header("accept", "application/json")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .header("Connection", "keep-alive")
        .header("Accept-Encoding", "gzip, deflate, br")
        .send()
        .await?;

    let json_response: RyzenCDNResponse = res.json().await?;

    if !json_response.success {
        let error_message = json_response.message.unwrap_or_else(|| "Upload failed".to_string());
        error!("RyzenCDN Error: {}", error_message);
        return Err(AppError::Other(error_message));
    }

    if let Some(url) = json_response.url {
        Ok(url)
    } else {
        Err(AppError::Other("RyzenCDN Error: URL not found in response".to_string()))
    }
}
