//! GraphQL support using async-graphql.
//!
//! Provides GraphQL schema, handlers, and playground endpoint.

pub mod schema;
pub mod handlers;

pub use schema::{create_schema, AppSchema, QueryRoot, MutationRoot};
pub use handlers::{graphql_handler, graphql_playground};
