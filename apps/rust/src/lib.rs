// Library root. Modules can access static config via crate::config::CONFIG

pub mod config;
pub mod utils;
pub mod urls;
pub mod jwt;
pub mod redis_client;
pub mod ratelimit;
pub mod fetch_with_proxy;
pub mod komik_base_url;
pub mod image_proxy;
pub mod ryzen_cdn;
pub mod error;

pub mod routes;
#[path = "../build_utils/mod.rs"]
pub mod build_utils;
