//! API controller generator with CRUD operations

use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate_controller(name: &str, crud: bool, model: Option<&str>) -> Result<()> {
    let api_dir = Path::new("src/routes/api").join(name);
    fs::create_dir_all(&api_dir)?;

    let model_name = model.map(|s| s.to_string()).unwrap_or_else(|| {
        // Singularize the resource name
        let singular = name.trim_end_matches('s');
        format!("{}{}", &singular[..1].to_uppercase(), &singular[1..])
    });

    if crud {
        generate_crud_routes(&api_dir, name, &model_name)?;
    } else {
        generate_basic_controller(&api_dir, name);
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
        r#"//! List all {resource}

use axum::{{Extension, Json, response::IntoResponse, Router}};
use sea_orm::{{DatabaseConnection, EntityTrait}};
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{model_low}::{{Entity as {model}, Model}};

#[utoipa::path(
    get,
    path = "/{resource}",
    tag = "{resource}",
    responses(
        (status = 200, description = "List all {resource}", body = [Model]),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn list(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {{
    match {model}.find().all(&db).await {{
        Ok(items) => Json(items).into_response(),
        Err(e) => {{
            eprintln!("Error listing {resource}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to list {resource}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router
}}
"#,
        resource = resource,
        model_low = model.to_lowercase(),
        model = model
    )
}

fn generate_show_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Get {singular} by ID

use axum::{{Extension, Json, extract::Path, response::IntoResponse, Router}};
use sea_orm::{{DatabaseConnection, EntityTrait}};
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{model_low}::{{Entity as {model}, Model}};

#[utoipa::path(
    get,
    path = "/{resource}/{{id}}",
    params(
        ("id" = i32, Path, description = "{singular} ID")
    ),
    tag = "{resource}",
    responses(
        (status = 200, description = "Get {singular} details", body = Model),
        (status = 404, description = "{singular} not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn show(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {{
    match {model}.find_by_id(id).one(&db).await {{
        Ok(Some(item)) => Json(item).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "{singular} not found").into_response(),
        Err(e) => {{
            eprintln!("Error getting {singular}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to get {singular}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router
}}
"#,
        singular = singular,
        model_low = model.to_lowercase(),
        model = model,
        resource = resource
    )
}

fn generate_create_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Create new {singular}

use axum::{{Extension, Json, response::IntoResponse, Router}};
use sea_orm::{{ActiveModelTrait, DatabaseConnection, Set}};
use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{model_low}::{{ActiveModel, Model}};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Create{model}Dto {{
    pub name: String,
    // Add your fields
}}

#[utoipa::path(
    post,
    path = "/{resource}",
    tag = "{resource}",
    request_body = Create{model}Dto,
    responses(
        (status = 201, description = "{singular} created successfully", body = Model),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn create(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<Create{model}Dto>,
) -> impl IntoResponse {{
    let new_item = ActiveModel {{
        name: Set(data.name),
        ..Default::default()
    }};
    
    match new_item.insert(&db).await {{
        Ok(item) => (axum::http::StatusCode::CREATED, Json(item)).into_response(),
        Err(e) => {{
            eprintln!("Error creating {singular}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create {singular}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router
}}
"#,
        singular = singular,
        model_low = model.to_lowercase(),
        model = model,
        resource = resource
    )
}

fn generate_update_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Update {singular}

use axum::{{Extension, Json, extract::Path, response::IntoResponse, Router}};
use sea_orm::{{ActiveModelTrait, DatabaseConnection, EntityTrait, Set}};
use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{model_low}::{{ActiveModel, Entity as {model}, Model}};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Update{model}Dto {{
    pub name: Option<String>,
    // Add your fields
}}

#[utoipa::path(
    put,
    path = "/{resource}/{{id}}",
    params(
        ("id" = i32, Path, description = "{singular} ID")
    ),
    tag = "{resource}",
    request_body = Update{model}Dto,
    responses(
        (status = 200, description = "{singular} updated successfully", body = Model),
        (status = 404, description = "{singular} not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn update(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<Update{model}Dto>,
) -> impl IntoResponse {{
    let item = match {model}.find_by_id(id).one(&db).await {{
        Ok(Some(item)) => item,
        Ok(None) => return (axum::http::StatusCode::NOT_FOUND, "{singular} not found").into_response(),
        Err(e) => {{
            eprintln!("Error finding {singular}: {{}}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to find {singular}").into_response();
        }}
    }};
    
    let mut active_model: ActiveModel = item.into();
    if let Some(name) = data.name {{
        active_model.name = Set(name);
    }}
    
    match active_model.update(&db).await {{
        Ok(updated) => Json(updated).into_response(),
        Err(e) => {{
            eprintln!("Error updating {singular}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to update {singular}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router
}}
"#,
        singular = singular,
        model_low = model.to_lowercase(),
        model = model,
        resource = resource
    )
}

fn generate_delete_handler(resource: &str, model: &str) -> String {
    let singular = resource.trim_end_matches('s');
    format!(
        r#"//! Delete {singular}

use axum::{{Extension, extract::Path, response::IntoResponse, Router}};
use sea_orm::{{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel}};
use std::sync::Arc;
use crate::routes::AppState;
use crate::entities::{model_low}::{{Entity as {model}}};

#[utoipa::path(
    delete,
    path = "/{resource}/{{id}}",
    params(
        ("id" = i32, Path, description = "{singular} ID")
    ),
    tag = "{resource}",
    responses(
        (status = 204, description = "{singular} deleted successfully"),
        (status = 404, description = "{singular} not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn destroy(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {{
    let item = match {model}.find_by_id(id).one(&db).await {{
        Ok(Some(item)) => item,
        Ok(None) => return (axum::http::StatusCode::NOT_FOUND, "{singular} not found").into_response(),
        Err(e) => {{
            eprintln!("Error finding {singular}: {{}}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to find {singular}").into_response();
        }}
    }};
    
    match item.into_active_model().delete(&db).await {{
        Ok(_) => axum::http::StatusCode::NO_CONTENT.into_response(),
        Err(e) => {{
            eprintln!("Error deleting {singular}: {{}}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete {singular}").into_response()
        }}
    }}
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router
}}
"#,
        singular = singular,
        model_low = model.to_lowercase(),
        model = model,
        resource = resource
    )
}

fn generate_basic_controller(api_dir: &Path, resource: &str) {
    let content = format!(
        r#"//! {resource} controller

use axum::Router;
use std::sync::Arc;
use crate::routes::AppState;

#[utoipa::path(
    get,
    path = "/{resource}",
    tag = "{resource}",
    responses(
        (status = 200, description = "{resource} endpoint", body = String)
    )
)]
pub async fn index() -> &'static str {{
    "{resource} endpoint"
}}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{
    router
}}
"#,
        resource = resource
    );

    let _ = fs::write(api_dir.join("index.rs"), content);
}
