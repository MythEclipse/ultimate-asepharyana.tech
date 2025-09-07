//! Headless Chrome Library
//!
//! A reusable, async, thread-safe headless Chrome library with:
//! - Browser pool with round-robin load balancing
//! - Reusable tabs with semaphore-based pooling
//! - Retry and timeout mechanisms for Cloudflare challenges
//! - Stealth features (User-Agent, viewport, random delays)
//! - Proxy support per browser instance
//! - Comprehensive logging

pub mod browser_pool;
pub mod config;
pub mod error;
pub mod stealth;
pub mod tab_manager;

pub use browser_pool::BrowserPool;
pub use config::BrowserConfig;
pub use error::BrowserError;
pub use stealth::StealthConfig;
pub use tab_manager::TabManager;

use std::sync::Arc;
use tokio::sync::Semaphore;

/// Main entry point for the headless Chrome library
pub struct HeadlessChrome {
    pool: Arc<BrowserPool>,
    semaphore: Arc<Semaphore>,
}

impl HeadlessChrome {
    /// Create a new HeadlessChrome instance with the given configuration
    pub async fn new(config: BrowserConfig) -> Result<Self, BrowserError> {
        let pool = Arc::new(BrowserPool::new(config.clone()).await?);
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tabs));

        Ok(Self { pool, semaphore })
    }

    /// Get a tab manager for performing operations
    pub async fn get_tab_manager(&self) -> Result<TabManager, BrowserError> {
        let _permit = self.semaphore.acquire().await
            .map_err(|e| BrowserError::SemaphoreError(e.to_string()))?;

        self.pool.get_tab_manager().await
    }
}
