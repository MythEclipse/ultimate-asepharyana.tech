/// Build script for generating API routes and OpenAPI documentation.
///
/// This script performs the following tasks:
/// - Creates the API routes directory if it doesn't exist.
/// - Generates module files for the API routes.
/// - Generates the root API module with OpenAPI schemas and handlers.
/// - Ensures rebuilds occur when build utilities or API routes change.
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use anyhow::{Result, Context};
use env_logger;
use itertools::Itertools;

mod build_utils;
use build_utils::{hash_utils::compute_directory_hash, mod_generator, openapi_generator};

/// Configuration for the build process
#[derive(Debug)]
struct BuildConfig {
    api_routes_path: String,
    build_utils_path: String,
    enable_logging: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            api_routes_path: "src/routes/api".to_string(),
            build_utils_path: "build_utils/".to_string(),
            enable_logging: true,
        }
    }
}

/// Type alias for API handlers
type ApiHandlers = Vec<build_utils::handler_updater::HandlerRouteInfo>;


/// Custom error types for build process
#[derive(Debug, thiserror::Error)]
enum BuildError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

/// Result type for build operations
type BuildResult<T> = Result<T, BuildError>;

fn main() -> Result<()> {
    // Initialize logging if enabled
    let config = BuildConfig::default();
    if config.enable_logging {
        env_logger::init();
        log::info!("Starting build process");
    }

    let start_time = SystemTime::now();

    // Setup build environment
    let api_routes_path = setup_build_environment(&config)?;

    // Collect API data
    let (api_handlers, openapi_schemas, modules) = collect_api_data(&api_routes_path, &config)?;

    // Generate API modules
    generate_api_modules(&api_routes_path, &modules, &api_handlers, &openapi_schemas)?;

    // Output metrics
    if let Ok(elapsed) = start_time.elapsed() {
        println!("cargo:warning=Build completed in {:.2}s, {} handlers, {} schemas, {} modules",
                 elapsed.as_secs_f64(), api_handlers.len(), openapi_schemas.len(), modules.len());
    }

    log::info!("Build process completed successfully");
    Ok(())
}

/// Setup the build environment: create directories and set Cargo rerun instructions
fn setup_build_environment(config: &BuildConfig) -> BuildResult<std::path::PathBuf> {
    log::debug!("Setting up build environment");

    // Instruct Cargo to rerun the build script if this file changes
    println!("cargo:rerun-if-changed=build.rs");

    // Define the path to the API routes directory
    let api_routes_path = Path::new(&config.api_routes_path);

    // Create the API routes directory and its parents if they don't exist
    fs::create_dir_all(&api_routes_path)
        .with_context(|| format!("Failed to create API routes directory: {:?}", api_routes_path))?;

    // Instruct Cargo to rerun if the API routes directory changes
    println!("cargo:rerun-if-changed={}/", config.api_routes_path);

    // Instruct Cargo to rerun if the build utilities directory changes
    println!("cargo:rerun-if-changed={}", config.build_utils_path);

    Ok(api_routes_path.to_path_buf())
}

/// Collect API data: handlers, schemas, and modules
fn collect_api_data(api_routes_path: &Path, _config: &BuildConfig) -> BuildResult<(ApiHandlers, HashSet<String>, Vec<String>)> {
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
fn generate_api_modules(api_routes_path: &Path, modules: &Vec<String>, api_handlers: &ApiHandlers, openapi_schemas: &HashSet<String>) -> BuildResult<()> {
    log::debug!("Generating API modules");

    openapi_generator::generate_root_api_mod(
        api_routes_path,
        modules,
        api_handlers,
        openapi_schemas,
    )?;

    Ok(())
}

fn should_regenerate(api_routes_path: &Path) -> Result<bool, BuildError> {
    let hash_result = compute_directory_hash(api_routes_path);
    let hash_file = api_routes_path.with_file_name("api_routes.hash");

    let should_regenerate = match &hash_result {
        Ok(current_hash) => {
            if let Ok(previous_hash_str) = fs::read_to_string(&hash_file) {
                if let Ok(previous_hash) = previous_hash_str.trim().parse::<u64>() {
                    if previous_hash == *current_hash {
                        log::info!("API routes unchanged, skipping regeneration");
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                log::debug!("No previous hash file found, proceeding with regeneration");
                true
            }
        }
        Err(e) => {
            log::error!("Failed to compute directory hash: {}", e);
            true
        }
    };

    // Save the new hash if computation was successful
    if should_regenerate {
        if let Ok(current_hash) = hash_result {
            fs::write(&hash_file, current_hash.to_string())
                .with_context(|| format!("Failed to save hash file: {:?}", hash_file))?;
        }
    }

    Ok(should_regenerate)
}
