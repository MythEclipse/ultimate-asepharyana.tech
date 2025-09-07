//! Stealth features for headless Chrome

use crate::headless_chrome::error::BrowserResult;
use chromiumoxide::Page;
use rand::Rng;
use std::time::Duration;

/// Stealth configuration
#[derive(Debug, Clone)]
pub struct StealthConfig {
    /// Random User-Agent strings
    pub user_agents: Vec<String>,
    /// Viewport configurations
    pub viewports: Vec<ViewportConfig>,
    /// Random delay range (min, max) in milliseconds
    pub random_delay_range: (u64, u64),
    /// Enable WebRTC disabling
    pub disable_webrtc: bool,
    /// Enable WebGL spoofing
    pub spoof_webgl: bool,
    /// Enable timezone spoofing
    pub spoof_timezone: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            ],
            viewports: vec![
                ViewportConfig { width: 1920, height: 1080 },
                ViewportConfig { width: 1366, height: 768 },
                ViewportConfig { width: 1536, height: 864 },
            ],
            random_delay_range: (500, 2000),
            disable_webrtc: true,
            spoof_webgl: true,
            spoof_timezone: true,
        }
    }
}

/// Viewport configuration
#[derive(Debug, Clone)]
pub struct ViewportConfig {
    pub width: u32,
    pub height: u32,
}

/// Stealth manager for applying stealth features to a page
#[derive(Clone)]
pub struct StealthManager {
    config: StealthConfig,
}

impl StealthManager {
    pub fn new(config: StealthConfig) -> Self {
        Self { config }
    }

    /// Apply all stealth features to a page
    pub async fn apply_stealth(&self, page: &Page) -> BrowserResult<()> {
        self.set_random_user_agent(page).await?;
        self.set_random_viewport(page).await?;
        self.disable_webrtc(page).await?;
        self.spoof_webgl(page).await?;
        self.spoof_timezone(page).await?;
        self.add_random_delay().await;
        Ok(())
    }

    /// Set a random User-Agent
    async fn set_random_user_agent(&self, page: &Page) -> BrowserResult<()> {
        if self.config.user_agents.is_empty() {
            return Ok(());
        }

        let mut rng = rand::rng();
        let user_agent = &self.config.user_agents[rng.random_range(0..self.config.user_agents.len())];

        page.evaluate(format!(
            "Object.defineProperty(navigator, 'userAgent', {{value: '{}'}})",
            user_agent
        )).await?;

        tracing::debug!("Set User-Agent: {}", user_agent);
        Ok(())
    }

    /// Set a random viewport
    async fn set_random_viewport(&self, page: &Page) -> BrowserResult<()> {
        if self.config.viewports.is_empty() {
            return Ok(());
        }

        let mut rng = rand::rng();
        let viewport = &self.config.viewports[rng.random_range(0..self.config.viewports.len())];

        page.evaluate(format!(
            "Object.defineProperty(screen, 'width', {{value: {}}}); Object.defineProperty(screen, 'height', {{value: {}}})",
            viewport.width, viewport.height
        )).await?;

        tracing::debug!("Set viewport: {}x{}", viewport.width, viewport.height);
        Ok(())
    }

    /// Disable WebRTC
    async fn disable_webrtc(&self, page: &Page) -> BrowserResult<()> {
        if !self.config.disable_webrtc {
            return Ok(());
        }

        page.evaluate(r#"
            Object.defineProperty(navigator, 'mediaDevices', {
                get: () => undefined
            });
            Object.defineProperty(navigator, 'webkitGetUserMedia', {
                get: () => undefined
            });
        "#).await?;

        tracing::debug!("Disabled WebRTC");
        Ok(())
    }

    /// Spoof WebGL
    async fn spoof_webgl(&self, page: &Page) -> BrowserResult<()> {
        if !self.config.spoof_webgl {
            return Ok(());
        }

        page.evaluate(r#"
            const getParameter = WebGLRenderingContext.prototype.getParameter;
            WebGLRenderingContext.prototype.getParameter = function(parameter) {
                if (parameter === 37445) {
                    return 'Intel Inc.';
                }
                if (parameter === 37446) {
                    return 'Intel(R) Iris(TM) Graphics 6100';
                }
                return getParameter.call(this, parameter);
            };
        "#).await?;

        tracing::debug!("Spoofed WebGL");
        Ok(())
    }

    /// Spoof timezone
    async fn spoof_timezone(&self, page: &Page) -> BrowserResult<()> {
        if !self.config.spoof_timezone {
            return Ok(());
        }

        page.evaluate(r#"
            Object.defineProperty(Intl, 'DateTimeFormat', {
                value: class extends Intl.DateTimeFormat {
                    resolvedOptions() {
                        const options = super.resolvedOptions();
                        options.timeZone = 'America/New_York';
                        return options;
                    }
                }
            });
        "#).await?;

        tracing::debug!("Spoofed timezone");
        Ok(())
    }

    /// Add random delay
    async fn add_random_delay(&self) {
        let mut rng = rand::rng();
        let delay = rng.random_range(self.config.random_delay_range.0..=self.config.random_delay_range.1);
        tokio::time::sleep(Duration::from_millis(delay)).await;
        tracing::debug!("Added random delay: {}ms", delay);
    }
}
