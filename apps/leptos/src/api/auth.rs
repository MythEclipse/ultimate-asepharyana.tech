use crate::api::types::{LoginRequest, LoginResponse, UserResponse, ApiError};
use crate::api::API_BASE_URL;
use reqwest::Client;

pub async fn login(request: LoginRequest) -> Result<LoginResponse, String> {
    let client = Client::new();
    let url = format!("{}/auth/login", API_BASE_URL);

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        response.json::<LoginResponse>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ApiError>().await.map_err(|_| "Unknown error".to_string())?;
        Err(error.message)
    }
}

pub async fn me(token: &str) -> Result<UserResponse, String> {
    let client = Client::new();
    let url = format!("{}/auth/me", API_BASE_URL);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        response.json::<UserResponse>().await.map_err(|e| e.to_string())
    } else {
        Err("Failed to fetch user profile".to_string())
    }
}
