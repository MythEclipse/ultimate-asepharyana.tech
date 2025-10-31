use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;

use super::db;
use super::models::CreateRoomRequest;
use crate::utils::error::AppError;
use crate::routes::AppState;

#[allow(dead_code)]
pub const ENDPOINT_METHOD: &str = "get,post";
#[allow(dead_code)]
pub const ENDPOINT_PATH: &str = "/api/chat/rooms";
#[allow(dead_code)]
pub const ENDPOINT_DESCRIPTION: &str = "Handles chat room operations";
#[allow(dead_code)]
pub const ENDPOINT_TAG: &str = "chat";
#[allow(dead_code)]
pub const OPERATION_ID: &str = "chat_rooms";

#[utoipa::path(
    post,
    path = "/api/chat/rooms",
    tag = "chat",
    operation_id = "create_chat_room",
    responses(
        (status = 200, description = "Room created successfully"),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn create_room_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Get user_id from JWT token
    let user_id = "user_123";

    let room = db::create_room(&state.pool, user_id, req).await?;

    Ok(Json(json!({
        "success": true,
        "room": room
    })))
}

#[utoipa::path(
    get,
    path = "/api/chat/rooms",
    tag = "chat",
    operation_id = "get_chat_rooms",
    responses(
        (status = 200, description = "List of chat rooms"),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn get_rooms_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let rooms = db::get_rooms(&state.pool).await?;

    Ok(Json(json!({
        "success": true,
        "rooms": rooms
    })))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route(ENDPOINT_PATH, post(create_room_handler))
        .route(ENDPOINT_PATH, get(get_rooms_handler))
}
