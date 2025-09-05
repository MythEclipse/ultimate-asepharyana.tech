/// Build script for generating API routes and OpenAPI documentation.
///
/// This script performs the following tasks:
/// - Creates the API routes directory if it doesn't exist.
/// - Generates module files for the API routes.
/// - Generates the root API module with OpenAPI schemas and handlers.
/// - Ensures rebuilds occur when build utilities or API routes change.
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::env;

use tempfile::NamedTempFile;

use anyhow::{Result, Context};
use env_logger;
use itertools::Itertools;

mod build_utils;
use build_utils::{hash_utils::compute_directory_hash, mod_generator, openapi_generator};

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
fn generate_api_modules(api_routes_path: &Path, modules: &Vec<String>, api_handlers: &ApiHandlers, openapi_schemas: &HashSet<String>) -> Result<()> {
    log::debug!("Generating API modules");

    openapi_generator::generate_root_api_mod(
        api_routes_path,
        modules,
        api_handlers,
        openapi_schemas,
    )?;

    Ok(())
}

fn should_regenerate(api_routes_path: &Path) -> Result<bool> {
    // Check FORCE_API_REGEN environment variable first
    if env::var("FORCE_API_REGEN").is_ok() {
        log::info!("FORCE_API_REGEN environment variable is set, forcing regeneration.");
        return Ok(true);
    }

    let hash_result = compute_directory_hash(api_routes_path);
    let out_dir = PathBuf::from(env::var("OUT_DIR").context("OUT_DIR not set")?);
    let hash_file = out_dir.join("api_routes.hash");

    let should_regenerate = match &hash_result {
        Ok(current_hash) => {
            if let Ok(previous_hash_str) = fs::read_to_string(&hash_file) {
                if previous_hash_str.trim() == current_hash {
                    log::info!("API routes unchanged, skipping regeneration");
                    false
                } else {
                    true
                }
            } else { // Failed to read previous hash, so regenerate
                log::debug!("No previous hash file found, proceeding with regeneration");
                true
            }
        },
        Err(e) => { // Failed to compute current hash, so regenerate
            log::error!("Failed to compute directory hash: {}", e);
            true
        }
    };

    // Save the new hash if computation was successful
    if should_regenerate {
        if let Ok(current_hash) = hash_result {
            let mut temp_file = NamedTempFile::new()
                .with_context(|| format!("Failed to create temporary file for hash: {:?}", hash_file))?;
            temp_file.write_all(current_hash.as_bytes())
                .with_context(|| format!("Failed to write hash to temporary file: {:?}", hash_file))?;
            temp_file.persist(&hash_file)
                .with_context(|| format!("Failed to persist temporary hash file to: {:?}", hash_file))?;
        }
    }

    Ok(should_regenerate)
}
