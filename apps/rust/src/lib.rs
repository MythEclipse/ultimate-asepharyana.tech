// Library root - organized module structure

// Core framework
pub mod core; // config, error, jwt, ratelimit

// Infrastructure
pub mod infra; // redis, http_client, proxy, image_proxy

// Features
pub mod browser; // Browser tab pooling
pub mod circuit_breaker; // Circuit breaker pattern
pub mod di; // Dependency injection
pub mod events; // Event bus (pub/sub)
pub mod extractors; // ValidatedJson, ValidatedQuery
pub mod graceful; // Graceful shutdown
pub mod graphql; // GraphQL API
pub mod health; // Health check endpoints
pub mod helpers; // Utility helpers
pub mod jobs; // Background job processing
pub mod middleware; // Auth, logging middleware
pub mod observability; // Metrics, request ID
pub mod scheduler; // Cron jobs
pub mod testing; // Test utilities
pub mod typescript; // TypeScript generation
pub mod versioning; // API versioning
pub mod ws; // WebSocket support

// Data
pub mod entities; // SeaORM entities
pub mod models; // Data models + types
pub mod seeder; // Database seeding
pub mod utils; // General utilities (for backward compat)

// Scraping utilities (app-specific)
pub mod scraping; // URLs, CDN, base URLs

// Build utilities
#[path = "../build_utils/mod.rs"]
pub mod build_utils;

// Routes
pub mod routes;

// ============================================================================
// Re-exports for backward compatibility
// ============================================================================

// Old module paths -> new paths
pub use core::config;
pub use core::error;
pub use core::jwt;
pub use core::ratelimit;
pub use infra::redis as redis_client;
pub use infra::proxy as fetch_with_proxy;
pub use infra::image_proxy;
pub use scraping::urls;
pub use scraping::komik_base_url;
pub use scraping::ryzen_cdn;
pub use seeder::seed;

// Re-exports for convenience
pub use core::config::CONFIG;
