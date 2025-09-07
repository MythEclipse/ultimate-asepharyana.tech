//! Configuration structures for the headless Chrome library

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Main browser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Number of browser instances to maintain in the pool
    pub browser_instances: usize,
    /// Maximum concurrent tabs across all browsers
    pub max_concurrent_tabs: usize,
    /// Maximum tabs per browser instance
    pub max_tabs_per_browser: usize,
    /// Browser startup arguments
    pub chrome_args: Vec<String>,
    /// Default timeout for operations
    pub default_timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay between attempts
    pub retry_delay: Duration,
    /// Enable stealth mode
    pub stealth_enabled: bool,
    /// Proxy configuration (optional)
    pub proxy: Option<ProxyConfig>,
    /// Logging level
    pub log_level: LogLevel,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
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
                "--disable-javascript".to_string(),
                "--disable-web-security".to_string(),
                "--disable-features=VizDisplayCompositor".to_string(),
            ],
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
            stealth_enabled: true,
            proxy: None,
            log_level: LogLevel::Info,
        }
    }
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy server URL
    pub server: String,
    /// Proxy username (optional)
    pub username: Option<String>,
    /// Proxy password (optional)
    pub password: Option<String>,
    /// Proxy type
    pub proxy_type: ProxyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyType {
    Http,
    Https,
    Socks4,
    Socks5,
}

/// Logging levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Viewport configuration for stealth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportConfig {
    pub width: u32,
    pub height: u32,
    pub device_scale_factor: f64,
    pub is_mobile: bool,
    pub has_touch: bool,
    pub is_landscape: bool,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            device_scale_factor: 1.0,
            is_mobile: false,
            has_touch: false,
            is_landscape: false,
        }
    }
}
