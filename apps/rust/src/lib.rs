// Library root - clean organized module structure
// All modules organized into logical folders

// ============================================================================
// Core Framework
// ============================================================================
pub mod core; // config, error, jwt, ratelimit

// ============================================================================
// Infrastructure
// ============================================================================
pub mod infra; // redis, http_client, proxy, image_proxy

// ============================================================================
// Features
// ============================================================================
pub mod browser; // Browser tab pooling for scraping
pub mod circuit_breaker; // Circuit breaker for external services
pub mod di; // Dependency injection container
pub mod events; // Event bus (pub/sub)
pub mod extractors; // ValidatedJson, ValidatedQuery
pub mod graceful; // Graceful shutdown with signals
pub mod graphql; // GraphQL API (async-graphql)
pub mod health; // Health check endpoints
pub mod helpers; // Utility helpers (string, datetime, crypto, file)
pub mod jobs; // Background job processing
pub mod middleware; // Auth, logging middleware
pub mod observability; // Metrics, request ID, tracing
pub mod routing; // API versioning
pub mod scheduler; // Cron jobs
pub mod testing; // Test utilities (TestApp)
pub mod typescript; // TypeScript type generation
pub mod ws; // WebSocket with room management

// ============================================================================
// Data Layer
// ============================================================================
pub mod entities; // SeaORM entities
pub mod models; // Data models + types
pub mod seeder; // Database seeding
pub mod utils; // General utilities

// ============================================================================
// Application-Specific (Scraping)
// ============================================================================
pub mod scraping; // URLs, CDN, base URLs

// ============================================================================
// Build & Routes
// ============================================================================
#[path = "../build_utils/mod.rs"]
pub mod build_utils;
pub mod routes;
