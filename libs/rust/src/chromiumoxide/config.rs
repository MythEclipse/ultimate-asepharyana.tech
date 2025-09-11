//! Browser configuration

use std::time::Duration;

/// Browser configuration
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Remote debugging port
    pub port: u16,
    /// Arguments to pass to Chrome
    pub chrome_args: Vec<String>,
    /// Whether to enable stealth features
    pub stealth_enabled: bool,
    /// Default timeout for operations in milliseconds
    pub default_timeout: Duration,
    /// Maximum number of retries for operations
    pub max_retries: u32,
    /// Delay between retries in milliseconds (exponential backoff base)
    pub retry_delay: Duration,
    /// Maximum number of concurrent tabs per browser instance (if using a pool)
    pub max_tabs_per_browser: usize,
    /// Maximum number of concurrent tabs across all browser instances
    pub max_concurrent_tabs: usize,
    /// Proxy configuration
    pub proxy: Option<ProxyConfig>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            port: 9222,
            chrome_args: vec![
                "--no-sandbox".to_string(),
                "--disable-gpu".to_string(),
                "--disable-dev-shm-usage".to_string(),
                "--disable-setuid-sandbox".to_string(),
                "--disable-background-networking".to_string(),
                "--disable-default-apps".to_string(),
                "--disable-extensions".to_string(),
                "--disable-sync".to_string(),
                "--disable-translate".to_string(),
                "--hide-scrollbars".to_string(),
                "--metrics-recording-only".to_string(),
                "--mute-audio".to_string(),
                "--no-first-run".to_string(),
                "--enable-automation".to_string(),
                "--disk-cache-size=1".to_string(),
                "--aggressive-cache-discard".to_string(),
                "--disable-cache".to_string(),
            ],
            stealth_enabled: true,
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            max_tabs_per_browser: 5,
            max_concurrent_tabs: 10,
            proxy: None,
        }
    }
}

/// Proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub server: String,
    pub proxy_type: ProxyType,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Proxy type
#[derive(Debug, Clone)]
pub enum ProxyType {
    Http,
    Https,
    Socks4,
    Socks5,
}
