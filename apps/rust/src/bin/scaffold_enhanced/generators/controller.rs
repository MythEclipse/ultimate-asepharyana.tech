//! API controller generator with CRUD operations

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn generate_controller(name: &str, crud: bool, model: Option<&str>) -> Result<()> {
    let api_dir = Path::new("src/routes/api").join(name);
    fs::create_dir_all(&api_dir)?;

    let model_name = model.unwrap_or_else(|| {
        // Singularize the resource name
        let singular = name.trim_end_matches('s');
        &singular[..1].to_uppercase() + &singular[1..]
    });

    if crud {
        generate_crud_routes(&api_dir, name, model_name)?;
    } else {
        generate_basic_controller(&api_dir, name, model_name)?;
    }

    Ok(())
}

fn generate_crud_routes(api_dir: &Path, resource: &str, model: &str) -> Result<()> {
    // List all
    let index_content = generate_list_handler(resource, model);
    fs::write(api_dir.join("index.rs"), index_content)?;

    // Get by ID
    let show_content = generate_show_handler(resource, model);
    fs::write(api_dir.join("[id].rs"), show_content)?;

    // Create
    let create_content = generate_create_handler(resource, model);
    fs::write(api_dir.join("create.rs"), create_content)?;

    // Update & Delete in [id] subdirectory
    fs::create_dir_all(api_dir.join("[id]"))?;

    let update_content = generate_update_handler(resource, model);
    fs::write(api_dir.join("[id]/update.rs"), update_content)?;

    let delete_content = generate_delete_handler(resource, model);
    fs::write(api_dir.join("[id]/delete.rs"), delete_content)?;

    Ok(())
}

fn generate_list_handler(resource: &str, model: &str) -> String {
    format!(
        r#"//! List all {}

use axum::{{Extension, Json, response::IntoResponse, Router, routing::get}};
use sea_orm::{{DatabaseConnection, EntityTrait}};
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{}::{{Entity as {}, Model}};

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/{}";

pub async fn list(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {{
    match {}.find().all(&db).await {{
        Ok(items) => Json(items).into_response(),
        Err(e) => {{
            eprintln!("Error listing {}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to list {}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router.route(ENDPOINT_PATH, get(list))
}}
"#,
        resource,
        model.to_lowercase(),
        model,
        resource,
        model,
        resource,
        resource
    )
}

fn generate_show_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Get {} by ID

use axum::{{Extension, Json, extract::Path, response::IntoResponse, Router, routing::get}};
use sea_orm::{{DatabaseConnection, EntityTrait}};
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{}::{{Entity as {}, Model}};

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/{}/:id";

pub async fn show(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {{
    match {}.find_by_id(id).one(&db).await {{
        Ok(Some(item)) => Json(item).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "{} not found").into_response(),
        Err(e) => {{
            eprintln!("Error getting {}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to get {}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router.route(ENDPOINT_PATH, get(show))
}}
"#,
        singular,
        model.to_lowercase(),
        model,
        resource,
        model,
        singular,
        singular,
        singular
    )
}

fn generate_create_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Create new {}

use axum::{{Extension, Json, response::IntoResponse, Router, routing::post}};
use sea_orm::{{ActiveModelTrait, DatabaseConnection, Set}};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{}::{{ActiveModel, Model}};

pub const ENDPOINT_METHOD: &str = "post";
pub const ENDPOINT_PATH: &str = "/{}";

#[derive(Deserialize)]
pub struct Create{}Dto {{
    pub name: String,
    // Add your fields
}}

pub async fn create(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<Create{}Dto>,
) -> impl IntoResponse {{
    let new_item = ActiveModel {{
        name: Set(data.name),
        ..Default::default()
    }};
    
    match new_item.insert(&db).await {{
        Ok(item) => (axum::http::StatusCode::CREATED, Json(item)).into_response(),
        Err(e) => {{
            eprintln!("Error creating {}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create {}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router.route(ENDPOINT_PATH, post(create))
}}
"#,
        singular,
        model.to_lowercase(),
        resource,
        model,
        model,
        singular,
        singular
    )
}

fn generate_update_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Update {}

use axum::{{Extension, Json, extract::Path, response::IntoResponse, Router, routing::put}};
use sea_orm::{{ActiveModelTrait, DatabaseConnection, EntityTrait, Set}};
use serde::Deserialize;
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{}::{{ActiveModel, Entity as {}, Model}};

pub const ENDPOINT_METHOD: &str = "put";
pub const ENDPOINT_PATH: &str = "/{}/:id";

#[derive(Deserialize)]
pub struct Update{}Dto {{
    pub name: Option<String>,
    // Add your fields
}}

pub async fn update(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<Update{}Dto>,
) -> impl IntoResponse {{
    let item = match {}.find_by_id(id).one(&db).await {{
        Ok(Some(item)) => item,
        Ok(None) => return (axum::http::StatusCode::NOT_FOUND, "{} not found").into_response(),
        Err(e) => {{
            eprintln!("Error finding {}: {{}}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to find {}").into_response();
        }}
    }};
    
    let mut active_model: ActiveModel = item.into();
    if let Some(name) = data.name {{
        active_model.name = Set(name);
    }}
    
    match active_model.update(&db).await {{
        Ok(updated) => Json(updated).into_response(),
        Err(e) => {{
            eprintln!("Error updating {}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to update {}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router.route(ENDPOINT_PATH, put(update))
}}
"#,
        singular,
        model.to_lowercase(),
        model,
        resource,
        model,
        model,
        model,
        singular,
        singular,
        singular,
        singular,
        singular
    )
}

fn generate_delete_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Delete {}

use axum::{{Extension, extract::Path, response::IntoResponse, Router, routing::delete}};
use sea_orm::{{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel}};
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{}::{{Entity as {}}};

pub const ENDPOINT_METHOD: &str = "delete";
pub const ENDPOINT_PATH: &str = "/{}/:id";

pub async fn destroy(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {{
    let item = match {}.find_by_id(id).one(&db).await {{
        Ok(Some(item)) => item,
        Ok(None) => return (axum::http::StatusCode::NOT_FOUND, "{} not found").into_response(),
        Err(e) => {{
            eprintln!("Error finding {}: {{}}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to find {}").into_response();
        }}
    }};
    
    match item.into_active_model().delete(&db).await {{
        Ok(_) => axum::http::StatusCode::NO_CONTENT.into_response(),
        Err(e) => {{
            eprintln!("Error deleting {}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete {}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router.route(ENDPOINT_PATH, delete(destroy))
}}
"#,
        singular,
        model.to_lowercase(),
        model,
        resource,
        model,
        singular,
        singular,
        singular,
        singular,
        singular
    )
}

fn generate_basic_controller(api_dir: &Path, resource: &str, model: &str) -> String {
    let content = format!(
        r#"//! {} controller

use axum::{{Router, routing::get}};
use std::sync::Arc;
use crate::routes::AppState;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/{}";

pub async fn index() -> &'static str {{
    "{} endpoint"
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router.route(ENDPOINT_PATH, get(index))
}}
"#,
        resource, resource, resource
    );

    fs::write(api_dir.join("index.rs"), content).unwrap();
    Ok(())
}
