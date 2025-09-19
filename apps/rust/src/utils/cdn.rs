use reqwest::multipart;
use mime_guess::from_path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RyzenCDNResponse {
    success: bool,
    message: Option<String>,
    url: Option<String>,
}

pub async fn ryzen_cdn(buffer: &[u8], filename: Option<&str>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    // Determine MIME type
    let mime_type = if let Some(fname) = filename {
        from_path(fname).first_or_octet_stream().to_string()
    } else {
        "application/octet-stream".to_string()
    };

    // Create multipart form
    let part = multipart::Part::bytes(buffer.to_vec())
        .file_name(filename.unwrap_or("file").to_string())
        .mime_str(&mime_type)?;

    let form = multipart::Form::new().part("file", part);

    // Send request
    let response = client
        .post("https://api.ryzumi.vip/api/uploader/ryzencdn")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 Edg/139.0.0.0")
        .header("Accept", "application/json")
        .multipart(form)
        .send()
        .await?;

    let json: RyzenCDNResponse = response.json().await?;

    if !json.success {
        return Err(format!("Upload failed: {}", json.message.unwrap_or_else(|| "Unknown error".to_string())).into());
    }

    json.url.ok_or_else(|| "No URL in response".into())
}
