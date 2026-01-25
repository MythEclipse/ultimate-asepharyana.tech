//! GraphQL support using async-graphql.
//!
//! Provides GraphQL schema, handlers, and playground endpoint.

pub mod handlers;
pub mod schema;

pub use handlers::{graphql_handler, graphql_playground};
pub use schema::{create_schema, AppSchema, MutationRoot, QueryRoot};
