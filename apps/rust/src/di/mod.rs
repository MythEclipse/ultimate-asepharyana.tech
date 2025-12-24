//! Dependency Injection container and service management.
//!
//! Provides a type-safe container for registering and resolving services,
//! similar to Laravel's Service Container or NestJS's DI system.

pub mod container;

pub use container::{ServiceContainer, ServiceProvider};
