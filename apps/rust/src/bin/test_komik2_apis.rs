use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use tokio;

const BASE_URL: &str = "http://localhost:4091/api";

async fn test_search_api() -> Result<()> {
    let client = Client::new();
    let query = "naruto";
    let url = format!("{}/komik/search?query={}", BASE_URL, query);

    println!("Testing URL: {}", url);

    let response = client.get(&url).send().await?;
    let status = response.status();
    let body = response.text().await?;

    println!("Status: {}", status);
    println!("Body: {}", body);

    assert!(status.is_success(), "API call failed with status: {}", status);

    let json: Value = serde_json::from_str(&body)?;
    assert!(json["data"].is_array(), "Data is not an array");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running API tests...");

    // Run tests sequentially
    test_search_api().await?;

    println!("All tests passed!");

    Ok(())
}
