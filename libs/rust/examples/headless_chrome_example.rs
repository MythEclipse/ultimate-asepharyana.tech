//! Example usage of the headless Chrome library

use rust_lib::headless_chrome::{BrowserConfig, HeadlessChrome};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create browser configuration with single shared browser instance
    let config = BrowserConfig {
        browser_instances: 1, // Single shared browser instance
        max_concurrent_tabs: 10,
        max_tabs_per_browser: 5,
        chrome_args: vec![
            "--no-sandbox".to_string(),
            "--disable-dev-shm-usage".to_string(),
            "--disable-gpu".to_string(),
            "--disable-extensions".to_string(),
            "--disable-plugins".to_string(),
            "--disable-images".to_string(),
            "--headless".to_string(),
        ],
        default_timeout: Duration::from_secs(30),
        max_retries: 3,
        retry_delay: Duration::from_millis(1000),
        stealth_enabled: true,
        proxy: None, // No proxy for this example
        log_level: rust_lib::headless_chrome::LogLevel::Info,
    };

    // Create headless Chrome instance
    let chrome = HeadlessChrome::new(config.clone()).await?;
    tracing::info!("Headless Chrome initialized");

    // Example 1: Simple navigation and content retrieval
    let tab = chrome.get_tab_manager().await?;
    tab.navigate("https://httpbin.org/html").await?;
    let content = tab.get_content().await?;
    tracing::info!("Page content length: {} characters", content.len());

    // Example 2: Wait for element and execute JavaScript
    let tab2 = chrome.get_tab_manager().await?;
    tab2.navigate("https://httpbin.org/json").await?;
    let json_data = tab2.evaluate_script("JSON.stringify(document.body.innerText)").await?;
    tracing::info!("JSON data: {}", json_data);

    // Example 3: Concurrent operations using shared global browser
    let urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/user-agent",
        "https://httpbin.org/headers",
    ];

    let mut handles = vec![];

    for url in urls {
        let config_clone = config.clone();
        let handle = tokio::spawn(async move {
            // Create a new HeadlessChrome instance that shares the global browser
            let chrome_instance = HeadlessChrome::new(config_clone).await?;
            let tab = chrome_instance.get_tab_manager().await?;
            tab.navigate(url).await?;
            let content = tab.get_content().await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>((url.to_string(), content.len()))
        });
        handles.push(handle);
    }

    // Wait for all concurrent operations to complete
    for handle in handles {
        match handle.await {
            Ok(Ok((url, length))) => {
                tracing::info!("Fetched {}: {} characters", url, length);
            }
            Ok(Err(e)) => {
                tracing::error!("Error fetching content: {}", e);
            }
            Err(e) => {
                tracing::error!("Task panicked: {}", e);
            }
        }
    }

    // Get global shared browser statistics
    let stats = chrome.get_stats().await;
    tracing::info!("Global browser stats: {:?}", stats);

    tracing::info!("Example completed successfully");
    Ok(())
}
