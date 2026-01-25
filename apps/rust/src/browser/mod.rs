//! Browser automation with tab pooling.
//!
//! This module provides a browser pool that maintains a single browser
//! process with multiple reusable tabs for efficient web scraping.

pub mod pool;

pub use pool::{BrowserPool, BrowserPoolConfig, PooledTab};
