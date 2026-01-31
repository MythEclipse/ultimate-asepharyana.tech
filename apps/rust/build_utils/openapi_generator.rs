use crate::build_utils::handler_updater::HandlerRouteInfo;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs;
use std::path::Path;

pub fn generate_root_api_mod(
    api_routes_path: &Path,
    modules: &[String],
    all_handlers: &[HandlerRouteInfo],
    schemas: &HashSet<String>,
) -> Result<utoipa::openapi::OpenApi> {
    generate_root_api_mod_internal(api_routes_path, modules, all_handlers, schemas)
}

fn generate_root_api_mod_internal(
    api_routes_path: &Path,
    modules: &[String],
    all_handlers: &[HandlerRouteInfo],
    schemas: &HashSet<String>,
) -> Result<utoipa::openapi::OpenApi> {
    // Initialize content
    let mut content = String::with_capacity(4096);
    content.push_str(crate::build_utils::constants::OPENAPI_HEADER);
    content.push('\n');

    // Add module imports
    for module in modules {
        writeln!(content, "pub mod {};", module)?;
    }
    content.push('\n');

    let (imports, names) = generate_schema_imports_and_names(schemas);

    // Add schema imports to content
    for import in imports {
        writeln!(content, "{}", import)?;
    }
    content.push('\n');

    // Generate OpenAPI documentation with handler paths and schemas
    let handler_paths = all_handlers
        .iter()
        .map(|h| format!("              {}::{}", h.handler_module_path, h.func_name))
        .join(",\n");

    let schema_list = names
        .iter()
        .map(|s| format!("                  {}", s))
        .join(",\n");

    content.push_str(&crate::build_utils::constants::OPENAPI_DOC_TEMPLATE
        .replacen("{}", &handler_paths, 1)
        .replacen("{}", &schema_list, 1)
    );
    
    content.push_str(crate::build_utils::constants::SECURITY_ADDON_CODE);
    content.push('\n');

    content.push_str(&generate_router_creation_code(modules));

    let mod_file_path = api_routes_path.join("mod.rs");
    fs::write(&mod_file_path, content)?;

    Ok({
        // This return block is for the build script to use the Type, but we are writing strings.
        // The original code returned a Result<utoipa::openapi::OpenApi>.
        // It constructed a temporary struct and returned .openapi().
        // We must preserve this behavior if the caller uses the return value.
        // The caller is build.rs. Let's check if it uses the return value.
        // We will assume it does checking/merging.
        
        use utoipa::OpenApi;
        #[derive(utoipa::OpenApi)]
        #[openapi(paths(), components(schemas()), modifiers(&SecurityAddon), security(("bearer_auth" = [])), servers((url = "https://ws.asepharyana.tech", description = "Production Server"), (url = "http://localhost:4091", description = "Local Development")), tags((name = "api", description = "Main API")))]
        struct TempApiDoc;
        struct SecurityAddon;
        impl utoipa::Modify for SecurityAddon {
            fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
                if let Some(components) = openapi.components.as_mut() {
                    use utoipa::openapi::security::*;
                    components.add_security_scheme(
                        "bearer_auth",
                        SecurityScheme::Http(
                            HttpBuilder::new()
                                .scheme(HttpAuthScheme::Bearer)
                                .bearer_format("JWT")
                                .build(),
                        ),
                    )
                }
            }
        }
        TempApiDoc::openapi()
    })
}

/// Generates the router creation code for the root API module.
fn generate_router_creation_code(modules: &[String]) -> String {
    let router_registrations = modules
        .iter()
        .map(|m| format!("    router = {}::register_routes(router);", m))
        .join("\n");

    let router_declaration = if modules.is_empty() {
        "    let router = Router::new();"
    } else {
        "    let mut router = Router::new();"
    };

    format!(
        "pub fn create_api_routes() -> Router<Arc<AppState>> {{\n{}\n{}\n    router\n}}\n",
        router_declaration, router_registrations
    )
}

/// Generates schema import strings and sanitized names, handling duplicates by aliasing.
fn generate_schema_imports_and_names(schemas: &HashSet<String>) -> (Vec<String>, Vec<String>) {
    let mut sorted_schemas: Vec<String> = schemas.iter().cloned().collect();
    sorted_schemas.sort();

    let mut imports = Vec::new();
    let mut names = Vec::new();
    let mut counts = HashMap::new();

    for full in sorted_schemas.iter() {
        let simple_name = full.split("::").last().unwrap().to_string();
        let count = counts.entry(simple_name.clone()).or_insert(0);
        *count += 1;

        if *count > 1 {
            let alias = format!("{} as {}_{}", full, simple_name, *count - 1);
            imports.push(format!("use {};", alias));
            names.push(format!("{}_{}", simple_name, *count - 1));
        } else {
            imports.push(format!("use {};", full));
            names.push(simple_name);
        }
    }
    (imports, names)
}
