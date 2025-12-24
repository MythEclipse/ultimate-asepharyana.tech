//! Build utilities for generating API routes and OpenAPI documentation.
//!
//! This module provides a comprehensive set of tools for automatically generating
//! Rust API handlers, routes, and OpenAPI specifications. It includes:
//!
//! - Handler template generation
//! - Route registration
//! - OpenAPI schema generation
//! - Path parameter extraction
//! - Type-safe HTTP method handling
//! - **Automatic file-based routing (Next.js-style)**
//!
//! ## Modules
//!
//! - `constants`: Common constants and regex patterns
//! - `errors`: Custom error types with helpful messages
//! - `types`: Type definitions for HTTP methods, templates, and metadata
//! - `route_scanner`: Automatic route discovery from file system
//! - `handler_template`: Template generation for new handlers
//! - `handler_updater`: Updates existing handler files with OpenAPI annotations
//! - `mod_generator`: Generates module files for API routes
//! - `openapi_generator`: Generates OpenAPI specifications
//! - `path_utils`: Utilities for path manipulation and sanitization
//! - `template_generator`: High-level template generation logic

pub mod auto_mod_generator;
pub mod constants;
pub mod handler_template;
pub mod handler_updater;
pub mod mod_generator;
pub mod openapi_generator;
pub mod path_utils;
pub mod route_scanner;
pub mod template_generator;
pub mod types;

/// Build operation tracker for errors and warnings during the build process.
///
/// This structure accumulates errors and warnings that occur during API generation,
/// allowing for comprehensive reporting at the end of the build.
#[derive(Debug)]
pub struct BuildOperation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for BuildOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildOperation {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
