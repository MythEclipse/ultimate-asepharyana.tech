# Headless Chrome Library

A reusable, async, thread-safe headless Chrome library for Rust with the following features:

## Features

- **Reusable tabs**: Tab closes and pool semaphore is released, but browser instance stays alive
- **Retry + Timeout**: Handles Cloudflare challenges and timeouts automatically
- **Multi-browser instance**: Round-robin load balancing for scalability
- **Stealth**: User-Agent rotation, viewport randomization, random delays
- **Proxy support**: Optional proxy configuration per browser instance
- **Async + thread-safe**: Suitable for hundreds of concurrent fetches
- **Comprehensive logging**: Full tracing support

## Usage

```rust
use rust_lib::headless_chrome::{BrowserConfig, HeadlessChrome};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create browser configuration
    let config = BrowserConfig {
        browser_instances: 2,
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
        proxy: None,
        log_level: rust_lib::headless_chrome::LogLevel::Info,
    };

    // Create headless Chrome instance
    let chrome = HeadlessChrome::new(config).await?;

    // Get a tab manager
    let tab = chrome.get_tab_manager().await?;

    // Navigate to a URL
    tab.navigate("https://example.com").await?;

    // Get page content
    let content = tab.get_content().await?;
    println!("Page content: {}", content);

    // Execute JavaScript
    let result = tab.evaluate_script("document.title").await?;
    println!("Page title: {}", result);

    Ok(())
}
```

## Configuration

### BrowserConfig

- `browser_instances`: Number of browser instances to maintain (default: 2)
- `max_concurrent_tabs`: Maximum concurrent tabs across all browsers (default: 10)
- `max_tabs_per_browser`: Maximum tabs per browser instance (default: 5)
- `chrome_args`: Chrome startup arguments
- `default_timeout`: Default timeout for operations (default: 30s)
- `max_retries`: Maximum retry attempts (default: 3)
- `retry_delay`: Delay between retries (default: 1s)
- `stealth_enabled`: Enable stealth features (default: true)
- `proxy`: Optional proxy configuration
- `log_level`: Logging level

### Proxy Support

```rust
use rust_lib::headless_chrome::{BrowserConfig, ProxyConfig, ProxyType};

let config = BrowserConfig {
    proxy: Some(ProxyConfig {
        server: "proxy.example.com:8080".to_string(),
        username: Some("user".to_string()),
        password: Some("pass".to_string()),
        proxy_type: ProxyType::Http,
    }),
    ..Default::default()
};
```

## Architecture

### Browser Pool
- Manages multiple browser instances
- Round-robin load balancing
- Semaphore-based concurrency control

### Tab Manager
- Handles individual tab operations
- Automatic retry with exponential backoff
- Cloudflare challenge detection
- Timeout handling

### Stealth Features
- Random User-Agent strings
- Viewport randomization
- WebRTC disabling
- WebGL spoofing
- Timezone spoofing
- Random delays between operations

## Error Handling

The library provides comprehensive error handling with specific error types:

- `BrowserStartupError`: Browser initialization failures
- `TabCreationError`: Tab creation failures
- `NavigationError`: Navigation failures
- `TimeoutError`: Operation timeouts
- `RetryLimitExceeded`: Maximum retries exceeded
- `CloudflareChallenge`: Cloudflare challenge detected
- `ProxyError`: Proxy-related errors

## Logging

All operations are logged using the `tracing` crate. Configure logging level through `BrowserConfig::log_level`.

## Dependencies

- `chromiumoxide`: Headless Chrome control
- `tokio`: Async runtime
- `serde`: Serialization
- `tracing`: Logging
- `rand`: Random number generation
- `thiserror`: Error handling

## Thread Safety

All components are thread-safe and can be used concurrently from multiple async tasks.
