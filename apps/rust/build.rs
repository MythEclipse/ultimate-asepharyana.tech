/// Build script for generating API routes and OpenAPI documentation.
///
/// This script performs the following tasks:
/// - Creates the API routes directory if it doesn't exist.
/// - Generates module files for the API routes.
/// - Generates the root API module with OpenAPI schemas and handlers.
/// - Ensures rebuilds occur when build utilities or API routes change.
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

use tempfile::NamedTempFile;

use anyhow::{Result, Context};
use env_logger;
use itertools::Itertools;
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
    // Initialize logging if enabled
    let config = BuildConfig::default();
    if config.enable_logging {
        env_logger::init();
        log::info!("Starting build process");
    }


    // Setup build environment
    let api_routes_path = setup_build_environment(&config)?;

    // Collect API data
    let (api_handlers, openapi_schemas, modules) = collect_api_data(&api_routes_path, &config)?;

    // Generate API modules and get the OpenAPI spec
    let openapi_doc = generate_api_modules(&api_routes_path, &modules, &api_handlers, &openapi_schemas)?;

    // Serialize OpenAPI spec to a temporary JSON file
    let out_dir = PathBuf::from(env::var("OUT_DIR").context("OUT_DIR not set")?);
    let openapi_spec_file_path = out_dir.join("openapi_spec.json");
    let mut openapi_spec_file = NamedTempFile::new_in(&out_dir)
        .context("Failed to create temporary OpenAPI spec file")?;
    serde_json::to_writer_pretty(&mut openapi_spec_file, &openapi_doc)
        .context("Failed to serialize OpenAPI spec to file")?;
    openapi_spec_file.persist(&openapi_spec_file_path)
        .map_err(|e| anyhow::anyhow!("Failed to persist OpenAPI spec file: {:?}", e))?;

    // Output metrics
    println!("cargo:warning=Build completed, {} handlers, {} schemas, {} modules",
             api_handlers.len(), openapi_schemas.len(), modules.len());

    log::info!("Build process completed successfully");
    Ok(())
}

/// Setup the build environment: create directories and set Cargo rerun instructions
fn setup_build_environment(config: &BuildConfig) -> Result<PathBuf> {
    log::debug!("Setting up build environment");

    // Instruct Cargo to rerun the build script if the FORCE_API_REGEN environment variable changes
    println!("cargo:rerun-if-env-changed=FORCE_API_REGEN");
    // Instruct Cargo to rerun the build script if this file changes
    println!("cargo:rerun-if-changed=build.rs");

    // Define the path to the API routes directory
    let api_routes_path = &config.api_routes_path;

    // Create the API routes directory and its parents if they don't exist
    fs::create_dir_all(&api_routes_path)
        .with_context(|| format!("Failed to create API routes directory: {:?}", api_routes_path))?;

    // Instruct Cargo to rerun if the API routes directory changes
    println!("cargo:rerun-if-changed={}/", config.api_routes_path.display());

    // Instruct Cargo to rerun if the build utilities directory changes
    println!("cargo:rerun-if-changed={}", config.build_utils_path.display());

    Ok(api_routes_path.clone())
}

/// Collect API data: handlers, schemas, and modules
fn collect_api_data(api_routes_path: &Path, _config: &BuildConfig) -> Result<(ApiHandlers, HashSet<String>, Vec<String>)> {
    log::debug!("Collecting API data");

    let should_regenerate_value = should_regenerate(api_routes_path)?;

    if !should_regenerate_value {
        // Return empty collections to indicate no change
        return Ok((Vec::new(), HashSet::new(), Vec::new()));
    }

    // Initialize collections for handlers, schemas, and modules
    let mut api_handlers = Vec::new();
    let mut openapi_schemas = HashSet::new();
    let mut modules = Vec::new();

    // Pre-allocate capacity based on expected route count (estimate)
    openapi_schemas.reserve(100);

    // Generate module files for the API routes directory
    mod_generator::generate_mod_for_directory(
        api_routes_path,
        api_routes_path,
        &mut api_handlers,
        &mut openapi_schemas,
        &mut modules,
    )?;

    // Sort and deduplicate the modules list using functional style
    let modules = modules.into_iter().collect::<HashSet<_>>().into_iter().sorted().collect();

    Ok((api_handlers, openapi_schemas, modules))
}

/// Generate the root API module with OpenAPI documentation
fn generate_api_modules(api_routes_path: &Path, modules: &Vec<String>, api_handlers: &ApiHandlers, openapi_schemas: &HashSet<String>) -> Result<OpenApi> {
    log::debug!("Generating API modules");

    let openapi_doc = openapi_generator::generate_root_api_mod(
        api_routes_path,
        modules,
        api_handlers,
        openapi_schemas,
    )?;

    Ok(openapi_doc)
}

fn should_regenerate(_api_routes_path: &Path) -> Result<bool> {
    // Check FORCE_API_REGEN environment variable first
    if env::var("FORCE_API_REGEN").is_ok() {
        log::info!("FORCE_API_REGEN environment variable is set, forcing regeneration.");
        return Ok(true);
    }

    // Always regenerate since hash checking is disabled
    Ok(true)
}
