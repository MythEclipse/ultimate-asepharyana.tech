// Library root. Modules can access static config via crate::config::CONFIG

pub mod config;
pub mod di; // Dependency Injection container
pub mod entities; // SeaORM generated entities
pub mod error;
pub mod extractors; // Custom extractors (ValidatedJson, etc.)
pub mod fetch_with_proxy;
pub mod image_proxy;
pub mod jobs; // Background job system
pub mod jwt;
pub mod komik_base_url;
pub mod middleware;
pub mod models;
pub mod ratelimit;
pub mod redis_client;
pub mod ryzen_cdn;
pub mod seed;
pub mod testing; // Testing utilities (TestApp, etc.)
pub mod urls;
pub mod utils;

#[path = "../build_utils/mod.rs"]
pub mod build_utils;
pub mod routes;
