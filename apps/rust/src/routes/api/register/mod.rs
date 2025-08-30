use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::MySqlPool;
use rust_lib::models::{user::RegisterRequest, user::User};
use crate::routes::ChatState; // Updated path to ChatState
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    user_id: String,
    email: String,
    name: String,
    exp: usize,
}

pub fn create_routes() -> Router<Arc<ChatState>> {
    Router::new()
        .route("/", post(register_handler))
}

pub async fn register_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<RegisterRequest>,
) -> Response {
    let pool = &state.pool;
    let jwt_secret = state.jwt_secret.as_bytes();

    // Check for missing fields
    if payload.name.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Missing required fields" })),
        )
            .into_response();
    }

    // Check if user with email already exists
    let existing_user = sqlx::query_as::<_, User>("SELECT id, name, email, password, role FROM User WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(pool.as_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({ "message": "User with this email already exists" })),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
        _ => {} // No existing user, continue
    }

    // Hash password
    let hashed_password = match hash(&payload.password, 10) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password hashing error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response();
        }
    };

    // Insert new user
    let new_user_id = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query(
        "INSERT INTO User (id, name, email, password, role) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&new_user_id)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&hashed_password)
    .bind("member") // Default role
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => {
            // Generate JWT token
            let expiration = Utc::now() + Duration::hours(1);
            let claims = Claims {
                user_id: new_user_id.clone(),
                email: payload.email.clone(),
                name: payload.name.clone(),
                exp: expiration.timestamp() as usize,
            };
            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret)) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("JWT encoding error: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "message": "Internal server error" })),
                    )
                        .into_response();
                }
            };

            let mut response = (
                StatusCode::CREATED,
                Json(json!({
                    "message": "User registered successfully",
                    "user": {
                        "id": new_user_id,
                        "name": payload.name,
                        "email": payload.email,
                    }
                })),
            )
                .into_response();

            // Set cookie
            response.headers_mut().insert(
                "Set-Cookie",
                format!("token={}; HttpOnly; Path=/; SameSite=Lax; Max-Age={}", token, Duration::hours(1).num_seconds())
                    .parse()
                    .unwrap(),
            );

            response
        }
        Err(e) => {
            eprintln!("User insertion error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            )
                .into_response()
        }
    }
}
