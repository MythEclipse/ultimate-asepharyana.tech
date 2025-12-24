//! TypeScript type generation from Rust structs.
//!
//! Uses ts-rs to generate TypeScript definitions for API types.

pub mod generator;

pub use generator::generate_typescript_types;
