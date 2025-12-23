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

pub mod constants;
pub mod errors;
pub mod types;
pub mod route_scanner;
pub mod route_registry;
pub mod auto_mod_generator;
pub mod openapi_auto_generator;
pub mod handler_template;
pub mod handler_updater;
pub mod mod_generator;
pub mod openapi_generator;
pub mod path_utils;
pub mod template_generator;


/// Build operation tracker for errors and warnings during the build process.
///
/// This structure accumulates errors and warnings that occur during API generation,
/// allowing for comprehensive reporting at the end of the build.
#[allow(dead_code)]
#[derive(Debug)]
pub struct BuildOperation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[allow(dead_code)]
impl Default for BuildOperation {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl BuildOperation {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
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

    /// Get a summary of all errors and warnings
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        
        if self.has_errors() {
            summary.push_str(&format!("Errors ({}): \n", self.errors.len()));
            for (i, error) in self.errors.iter().enumerate() {
                summary.push_str(&format!("  {}. {}\n", i + 1, error));
            }
        }
        
        if self.has_warnings() {
            summary.push_str(&format!("Warnings ({}): \n", self.warnings.len()));
            for (i, warning) in self.warnings.iter().enumerate() {
                summary.push_str(&format!("  {}. {}\n", i + 1, warning));
            }
        }
        
        summary
    }
}
