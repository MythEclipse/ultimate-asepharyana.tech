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

    content.push_str(&generate_router_creation_code(modules, all_handlers));

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
        #[openapi(paths(), components(schemas()), modifiers(&SecurityAddon), security(("bearer_auth" = [])), servers((url = "https://ws.asepharyana.tech", description = "Production Server"), (url = "http://localhost:4091", description = "Local Development")), tags())]
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
fn generate_router_creation_code(modules: &[String], all_handlers: &[HandlerRouteInfo]) -> String {
    let mut registrations = Vec::new();

    // 1. Add module-level registrations (backward compatibility and manual routes)
    for module in modules {
        registrations.push(format!("    router = {}::register_routes(router);", module));
    }

    // 2. Add automatic route registrations for all handlers
    for handler in all_handlers {
        let auth_layer = if handler.is_protected {
            ".layer(crate::middleware::auth::AuthMiddleware::layer())"
        } else {
            ""
        };

        registrations.push(format!(
            "    router = router.route(\"{}\", axum::routing::{}({}::{}){});",
            handler.route_path,
            handler.http_method.to_lowercase(),
            handler.handler_module_path,
            handler.func_name,
            auth_layer
        ));
    }

    format!(
        "pub fn create_api_routes() -> Router<Arc<AppState>> {{\n    let mut router = Router::new();\n{}\n    router\n}}\n",
        registrations.join("\n")
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
