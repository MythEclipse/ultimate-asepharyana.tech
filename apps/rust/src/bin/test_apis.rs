use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use tokio;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt; // Add this for Windows-specific process creation flags

const BASE_URL: &str = "http://localhost:4091/api";

async fn test_anime_flow() -> Result<()> {
    let client = Client::new();

    // 1. Test /api/anime endpoint to get a list of anime and extract a slug
    println!("Testing /api/anime endpoint...");
    let anime_list_url = format!("{}/anime", BASE_URL);
    let anime_list_response = client.get(&anime_list_url).send().await?;
    let anime_list_status = anime_list_response.status();
    let anime_list_body = anime_list_response.text().await?;

    println!("Status for /api/anime: {}", anime_list_status);
    println!("Body for /api/anime: {}", anime_list_body);

    assert!(anime_list_status.is_success(), "API call to /api/anime failed with status: {}", anime_list_status);

    let anime_list_json: Value = serde_json::from_str(&anime_list_body)?;
    let mut anime_slug: Option<String> = None;

    if let Some(data) = anime_list_json["data"].as_object() {
        if let Some(ongoing_anime) = data["ongoing_anime"].as_array() {
            if let Some(first_anime) = ongoing_anime.first() {
                if let Some(slug) = first_anime["slug"].as_str() {
                    anime_slug = Some(slug.to_string());
                }
            }
        }
        if anime_slug.is_none() {
            if let Some(complete_anime) = data["complete_anime"].as_array() {
                if let Some(first_anime) = complete_anime.first() {
                    if let Some(slug) = first_anime["slug"].as_str() {
                        anime_slug = Some(slug.to_string());
                    }
                }
            }
        }
    }

    let slug = anime_slug.expect("No anime slug found in /api/anime response. Cannot proceed with detail test.");
    println!("Extracted slug: {}", slug);

    // 2. Test /api/anime/detail/{slug} endpoint using the extracted slug
    println!("Testing /api/anime/detail/{} endpoint...", slug);
    let anime_detail_url = format!("{}/anime/detail/{}", BASE_URL, slug);
    let anime_detail_response = client.get(&anime_detail_url).send().await?;
    let anime_detail_status = anime_detail_response.status();
    let anime_detail_body = anime_detail_response.text().await?;

    println!("Status for /api/anime/detail/{}: {}", slug, anime_detail_status);
    println!("Body for /api/anime/detail/{}: {}", slug, anime_detail_body);

    assert!(anime_detail_status.is_success(), "API call to /api/anime/detail/{} failed with status: {}", slug, anime_detail_status);

    let anime_detail_json: Value = serde_json::from_str(&anime_detail_body)?;
    assert!(anime_detail_json["data"].is_object(), "Detail data is not an object");
    assert!(anime_detail_json["data"]["title"].is_string(), "Detail data does not contain a title");

    println!("Successfully retrieved details for anime with slug: {}", slug);

    Ok(())
}

use std::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Rust project in development mode...");
    println!("Starting server with cargo run...");

    let mut command = Command::new("cargo");
    command.args(&["run", "--bin", "rust"]);

    #[cfg(target_os = "windows")]
    {
        const DETACHED_PROCESS: u32 = 0x00000008;
        command.creation_flags(DETACHED_PROCESS);
    }

    let mut server_process = command.spawn()?;

    println!("Waiting for server to start...");
    let mut attempts = 0;
    let max_attempts = 30; // 30 * 1 second = 30 seconds timeout
    let mut server_ready = false;

    while attempts < max_attempts {
        match reqwest::get(format!("{}/anime", BASE_URL)).await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Server is ready!");
                    server_ready = true;
                    break;
                }
            }
            Err(_) => {
                // Server not yet up or connection refused
            }
        }
        sleep(Duration::from_secs(1)).await;
        attempts += 1;
    }

    if !server_ready {
        server_process.kill()?;
        anyhow::bail!("Server did not start in time.");
    }

    println!("Running API tests...");
    let test_result = test_anime_flow().await;

    println!("Stopping server...");
    server_process.kill()?;
    println!("Server stopped.");

    test_result?; // Propagate any error from the test_anime_flow

    println!("All anime API tests passed!");

    Ok(())
}
