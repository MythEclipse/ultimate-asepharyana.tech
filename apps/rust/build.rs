/// Build script for generating API routes and OpenAPI documentation.
///
/// This script performs the following tasks:
/// - Creates the API routes directory if it doesn't exist.
/// - Generates module files for the API routes.
/// - Generates the root API module with OpenAPI schemas and handlers.
/// - Ensures rebuilds occur when build utilities or API routes change.
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use env_logger;
use itertools::Itertools;
use openapiv3;
use tempfile::NamedTempFile;
use utoipa::openapi::OpenApi;

mod build_utils;
use build_utils::{mod_generator, openapi_generator};


/// Configuration for the build process
#[derive(Debug)]
struct BuildConfig {
    api_routes_path: PathBuf,
    build_utils_path: PathBuf,
    enable_logging: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            api_routes_path: "src/routes/api".into(),
            build_utils_path: "build_utils/".into(),
            enable_logging: true,
        }
    }
}

/// Type alias for API handlers
type ApiHandlers = Vec<build_utils::handler_updater::HandlerRouteInfo>;

fn main() -> Result<()> {
    // Initialize configuration and logging
    let config = BuildConfig::default();
    if config.enable_logging {
        env_logger::init();
        log::info!("Starting API build process");
    }

    // Setup environment and collect API data
    let api_routes_path = setup_build_environment(&config)?;
    let (api_handlers, openapi_schemas, modules) = collect_api_data(&api_routes_path)?;

    // Generate and validate OpenAPI specification
    let openapi_doc = generate_api_modules(&api_routes_path, &modules, &api_handlers, &openapi_schemas)?;
    validate_openapi_spec(&openapi_doc)?;

    // Write OpenAPI spec to output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").context("OUT_DIR environment variable not set")?);
    let openapi_spec_path = out_dir.join("openapi_spec.json");

    let mut temp_file = NamedTempFile::new_in(&out_dir)
        .context("Failed to create temporary OpenAPI spec file")?;

    serde_json::to_writer_pretty(&mut temp_file, &openapi_doc)
        .context("Failed to serialize OpenAPI specification")?;

    temp_file.persist(&openapi_spec_path)
        .map_err(|e| anyhow::anyhow!("Failed to save OpenAPI spec: {:?}", e))?;

    // Print build metrics
    println!("cargo:warning=API build completed - {} handlers, {} schemas, {} modules",
             api_handlers.len(), openapi_schemas.len(), modules.len());

    log::info!("API build process completed successfully");
    Ok(())
}

/// Setup build environment: create directories and configure Cargo reruns
fn setup_build_environment(config: &BuildConfig) -> Result<PathBuf> {
    log::debug!("Setting up build environment");

    // Configure Cargo rerun triggers
    println!("cargo:rerun-if-env-changed=FORCE_API_REGEN");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}/", config.api_routes_path.display());
    println!("cargo:rerun-if-changed={}", config.build_utils_path.display());

    // Create API routes directory if it doesn't exist
    fs::create_dir_all(&config.api_routes_path)
        .with_context(|| format!("Failed to create API routes directory: {:?}", config.api_routes_path))?;

    Ok(config.api_routes_path.clone())
}

/// Collect API data: handlers, schemas, and modules
fn collect_api_data(api_routes_path: &Path) -> Result<(ApiHandlers, HashSet<String>, Vec<String>)> {
    log::debug!("Collecting API data");

    // Always regenerate (hash checking disabled)
    // The previous `should_regenerate` function always returned true, so this is inlined.

    let mut api_handlers = Vec::new();
    let mut openapi_schemas = HashSet::new();
    let mut modules = Vec::new();

    // Pre-allocate capacity for better performance
    openapi_schemas.reserve(100);

    // Generate module files and collect data
    mod_generator::generate_mod_for_directory(
        api_routes_path,
        api_routes_path,
        &mut api_handlers,
        &mut openapi_schemas,
        &mut modules,
    )?;

    // Deduplicate and sort modules using functional pattern
    let modules = modules.into_iter()
        .unique()
        .sorted()
        .collect();

    Ok((api_handlers, openapi_schemas, modules))
}

/// Generate root API module and OpenAPI documentation
fn generate_api_modules(
    api_routes_path: &Path,
    modules: &Vec<String>,
    api_handlers: &ApiHandlers,
    openapi_schemas: &HashSet<String>
) -> Result<OpenApi> {
    log::debug!("Generating API modules and OpenAPI documentation");

    Ok(openapi_generator::generate_root_api_mod(
        api_routes_path,
        modules,
        api_handlers,
        openapi_schemas,
    )?)
}


fn validate_openapi_spec(openapi: &OpenApi) -> Result<()> {
    log::debug!("Validating OpenAPI specification");

    let json = serde_json::to_string(openapi)
        .context("Failed to serialize OpenAPI spec for validation")?;

    let _: openapiv3::OpenAPI = serde_json::from_str(&json)
        .context("OpenAPI specification validation failed")?;

    log::info!("OpenAPI specification validation passed");
    Ok(())
}
