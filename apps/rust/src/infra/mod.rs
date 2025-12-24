//! Infrastructure utilities - Redis, HTTP clients, proxies.

pub mod redis;
pub mod http_client;
pub mod proxy;
pub mod image_proxy;

pub use redis::REDIS_POOL;
pub use http_client::{HttpClient, HTTP_CLIENT, http_client};
