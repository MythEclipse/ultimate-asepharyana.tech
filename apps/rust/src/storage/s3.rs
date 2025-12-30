//! S3-compatible storage driver.
//!
//! Provides storage for AWS S3 and compatible services (MinIO, DigitalOcean Spaces, etc.).
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::storage::{Storage, S3Driver, S3Config};
//!
//! let config = S3Config {
//!     bucket: "my-bucket".to_string(),
//!     region: "us-east-1".to_string(),
//!     endpoint: None, // Use AWS default
//!     access_key: "AKIAIOSFODNN7EXAMPLE".to_string(),
//!     secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
//! };
//!
//! let storage = Storage::new(S3Driver::new(config));
//! storage.put("images/photo.jpg", &bytes).await?;
//! ```

use super::driver::{StorageDriver, StorageError};
use super::FileMetadata;
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::{Digest, Sha256};

/// S3 storage configuration.
#[derive(Debug, Clone)]
pub struct S3Config {
    /// Bucket name.
    pub bucket: String,
    /// AWS region (e.g., "us-east-1").
    pub region: String,
    /// Custom endpoint URL (for MinIO, etc.).
    pub endpoint: Option<String>,
    /// Access key ID.
    pub access_key: String,
    /// Secret access key.
    pub secret_key: String,
    /// Use path-style URLs (for MinIO compatibility).
    pub path_style: bool,
    /// Base URL for public file access.
    pub public_url: Option<String>,
}

impl Default for S3Config {
    fn default() -> Self {
        Self {
            bucket: String::new(),
            region: "us-east-1".to_string(),
            endpoint: None,
            access_key: String::new(),
            secret_key: String::new(),
            path_style: false,
            public_url: None,
        }
    }
}

/// S3-compatible storage driver.
pub struct S3Driver {
    config: S3Config,
    client: Client,
}

impl S3Driver {
    /// Create a new S3 driver.
    pub fn new(config: S3Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Get the base endpoint URL.
    fn endpoint(&self) -> String {
        match &self.config.endpoint {
            Some(ep) => ep.trim_end_matches('/').to_string(),
            None => format!("https://s3.{}.amazonaws.com", self.config.region),
        }
    }

    /// Get the full URL for a path.
    fn url(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        if self.config.path_style {
            format!("{}/{}/{}", self.endpoint(), self.config.bucket, path)
        } else {
            format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                self.config.bucket, self.config.region, path
            )
        }
    }

    /// Sign a request using AWS Signature Version 4.
    fn sign_request(
        &self,
        method: &str,
        path: &str,
        headers: &[(&str, &str)],
        payload_hash: &str,
    ) -> Vec<(String, String)> {
        let now = chrono::Utc::now();
        let date_stamp = now.format("%Y%m%d").to_string();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();

        let host = if self.config.path_style {
            self.endpoint()
                .replace("https://", "")
                .replace("http://", "")
        } else {
            format!(
                "{}.s3.{}.amazonaws.com",
                self.config.bucket, self.config.region
            )
        };

        let path_for_signing = if self.config.path_style {
            format!("/{}/{}", self.config.bucket, path.trim_start_matches('/'))
        } else {
            format!("/{}", path.trim_start_matches('/'))
        };

        // Canonical headers
        let mut header_list: Vec<(&str, String)> = vec![
            ("host", host.clone()),
            ("x-amz-content-sha256", payload_hash.to_string()),
            ("x-amz-date", amz_date.clone()),
        ];

        for (k, v) in headers {
            header_list.push((k, v.to_string()));
        }

        header_list.sort_by(|a, b| a.0.cmp(b.0));

        let canonical_headers: String = header_list
            .iter()
            .map(|(k, v)| format!("{}:{}\n", k.to_lowercase(), v))
            .collect();

        let signed_headers: String = header_list
            .iter()
            .map(|(k, _)| k.to_lowercase())
            .collect::<Vec<_>>()
            .join(";");

        // Canonical request
        let canonical_request = format!(
            "{}\n{}\n\n{}\n{}\n{}",
            method, path_for_signing, canonical_headers, signed_headers, payload_hash
        );

        let canonical_request_hash =
            hex::encode(sha2::Sha256::digest(canonical_request.as_bytes()));

        // String to sign
        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, canonical_request_hash
        );

        // Signing key
        let k_date = hmac_sha256(
            format!("AWS4{}", self.config.secret_key).as_bytes(),
            date_stamp.as_bytes(),
        );
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");

        // Signature
        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        // Authorization header
        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key, credential_scope, signed_headers, signature
        );

        vec![
            ("Host".to_string(), host),
            ("x-amz-date".to_string(), amz_date),
            ("x-amz-content-sha256".to_string(), payload_hash.to_string()),
            ("Authorization".to_string(), authorization),
        ]
    }
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key length is always valid");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[async_trait]
impl StorageDriver for S3Driver {
    async fn put(&self, path: &str, content: &[u8]) -> Result<(), StorageError> {
        let url = self.url(path);
        let payload_hash = hex::encode(sha2::Sha256::digest(content));

        let headers = self.sign_request("PUT", path, &[], &payload_hash);

        let mut req = self.client.put(&url).body(content.to_vec());

        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        let response = req.send().await.map_err(|e| {
            tracing::error!("S3 PUT error: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            tracing::error!("S3 PUT failed: {} - {}", status, body);
            return Err(StorageError::IoError(format!("S3 error: {}", status)));
        }

        Ok(())
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        let url = self.url(path);
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"; // Empty payload

        let headers = self.sign_request("GET", path, &[], payload_hash);

        let mut req = self.client.get(&url);

        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        let response = req.send().await.map_err(|e| {
            tracing::error!("S3 GET error: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        if response.status().as_u16() == 404 {
            return Err(StorageError::NotFound(path.to_string()));
        }

        if !response.status().is_success() {
            let status = response.status();
            return Err(StorageError::IoError(format!("S3 error: {}", status)));
        }

        let bytes = response.bytes().await.map_err(|e| {
            tracing::error!("S3 GET body error: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        Ok(bytes.to_vec())
    }

    async fn exists(&self, path: &str) -> Result<bool, StorageError> {
        let url = self.url(path);
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let headers = self.sign_request("HEAD", path, &[], payload_hash);

        let mut req = self.client.head(&url);

        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        let response = req.send().await.map_err(|e| {
            tracing::error!("S3 HEAD error: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        Ok(response.status().is_success())
    }

    async fn delete(&self, path: &str) -> Result<(), StorageError> {
        let url = self.url(path);
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let headers = self.sign_request("DELETE", path, &[], payload_hash);

        let mut req = self.client.delete(&url);

        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        let response = req.send().await.map_err(|e| {
            tracing::error!("S3 DELETE error: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status();
            return Err(StorageError::IoError(format!("S3 error: {}", status)));
        }

        Ok(())
    }

    async fn url(&self, path: &str) -> Result<String, StorageError> {
        match &self.config.public_url {
            Some(base) => {
                let path = path.trim_start_matches('/');
                Ok(format!("{}/{}", base.trim_end_matches('/'), path))
            }
            None => Ok(self.url(path)),
        }
    }

    async fn metadata(&self, path: &str) -> Result<FileMetadata, StorageError> {
        let url = self.url(path);
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let headers = self.sign_request("HEAD", path, &[], payload_hash);

        let mut req = self.client.head(&url);

        for (key, value) in headers {
            req = req.header(&key, &value);
        }

        let response = req.send().await.map_err(|e| {
            tracing::error!("S3 HEAD error: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        if response.status().as_u16() == 404 {
            return Err(StorageError::NotFound(path.to_string()));
        }

        let size = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let mime_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let modified = response
            .headers()
            .get("last-modified")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| chrono::DateTime::parse_from_rfc2822(s).ok())
            .map(|dt| dt.timestamp());

        Ok(FileMetadata {
            size,
            mime_type,
            modified,
            created: None,
        })
    }

    async fn list(&self, _directory: &str) -> Result<Vec<String>, StorageError> {
        // S3 list objects requires more complex implementation
        // This is a simplified version
        Err(StorageError::Other(
            "S3 list not implemented in simplified driver".to_string(),
        ))
    }
}
