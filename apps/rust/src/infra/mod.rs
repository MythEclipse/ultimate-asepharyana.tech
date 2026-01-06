//! Infrastructure utilities - Redis, HTTP clients, proxies.

pub mod db_setup;
pub mod http_client;
pub mod image_proxy;
pub mod proxy;
pub mod redis;

pub use http_client::{http_client, HttpClient, HTTP_CLIENT};
pub use redis::REDIS_POOL;
