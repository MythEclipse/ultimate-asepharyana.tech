use reqwest::Client;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing komik2 APIs. Starting the server on http://127.0.0.1:4092");

    println!("Starting the server...");
    let mut child = Command::new("cargo")
        .args(&["run", "--bin", "rust"])
        .env("PORT", "4092")  // Set environment variable for port
        .spawn()
        .expect("Failed to start server");

    let client = Client::new();
    let base_url = "http://127.0.0.1:4092"; // Assuming your server runs on this address and port

    println!("Server started. Waiting for it to be ready...");
    let mut attempts = 0;
    let max_attempts = 60; // Wait up to 60 seconds
    loop {
        match client.get(format!("{}/docs", base_url)).send().await {
            Ok(_) => {
                println!("Server is ready!");
                break;
            }
            Err(_) => {
                attempts += 1;
                if attempts >= max_attempts {
                    panic!("Server did not start within {} seconds", max_attempts);
                }
                println!("Waiting for server... attempt {}/{}", attempts, max_attempts);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    println!("Starting API tests...");

    // Test /api/komik2/detail
    println!("Testing /api/komik2/detail...");
    let detail_url = format!("{}/api/komik2/detail?komik_id=boku-to-kimi-gyaru-ga-fufu-ni-naru-made", base_url);
    let response = client.get(&detail_url).send().await?.error_for_status()?;
    let json_response: serde_json::Value = response.json().await?;
    println!("Detail API Response: {:?}", json_response);
    assert!(json_response["status"].as_bool().unwrap_or(false));
    assert!(json_response["data"].is_object());
    println!("Detail API test passed.");

    // Test /api/komik2/chapter
    println!("Testing /api/komik2/chapter...");
    let chapter_url = format!("{}/api/komik2/chapter?chapter_url=boku-to-kimi-gyaru-ga-fufu-ni-naru-made-chapter-1", base_url);
    let response = client.get(&chapter_url).send().await?.error_for_status()?;
    let json_response: serde_json::Value = response.json().await?;
    println!("Chapter API Response: {:?}", json_response);
    assert!(json_response["message"].as_str().unwrap_or("") == "Ok");
    assert!(json_response["data"].is_object());
    println!("Chapter API test passed.");

    // Test /api/komik2/search
    println!("Testing /api/komik2/search...");
    let search_url = format!("{}/api/komik2/search?query=naruto", base_url);
    let response = client.get(&search_url).send().await?.error_for_status()?;
    let json_response: serde_json::Value = response.json().await?;
    println!("Search API Response: {:?}", json_response);
    assert!(json_response["data"].is_array());
    assert!(json_response["pagination"].is_object());
    println!("Search API test passed.");

    // Test /api/komik2/manga
    println!("Testing /api/komik2/manga...");
    let manga_url = format!("{}/api/komik2/manga?page=1", base_url);
    let response = client.get(&manga_url).send().await?.error_for_status()?;
    let json_response: serde_json::Value = response.json().await?;
    println!("Manga API Response: {:?}", json_response);
    assert!(json_response["data"].is_array());
    assert!(json_response["pagination"].is_object());
    println!("Manga API test passed.");

    // Test /api/komik2/manhua
    println!("Testing /api/komik2/manhua...");
    let manhua_url = format!("{}/api/komik2/manhua?page=1", base_url);
    let response = client.get(&manhua_url).send().await?.error_for_status()?;
    let json_response: serde_json::Value = response.json().await?;
    println!("Manhua API Response: {:?}", json_response);
    assert!(json_response["data"].is_array());
    assert!(json_response["pagination"].is_object());
    println!("Manhua API test passed.");

    // Test /api/komik2/manhwa
    println!("Testing /api/komik2/manhwa...");
    let manhwa_url = format!("{}/api/komik2/manhwa?page=1", base_url);
    let response = client.get(&manhwa_url).send().await?.error_for_status()?;
    let json_response: serde_json::Value = response.json().await?;
    println!("Manhwa API Response: {:?}", json_response);
    assert!(json_response["data"].is_array());
    assert!(json_response["pagination"].is_object());
    println!("Manhwa API test passed.");

    println!("All komik2 API tests passed successfully!");

    println!("Shutting down the server...");
    child.kill().await?;

    Ok(())
}
