# Rust Application Best Practices

## Arsitektur

Aplikasi ini menggunakan arsitektur modular dengan clean separation of concerns:

```
src/
├── core/           # Core framework (config, error, jwt, rate limiting)
├── infra/          # Infrastructure (redis, http_client, proxy)
├── features/       # Business features
├── auth/           # Authentication & authorization
├── entities/       # Database entities (SeaORM)
├── models/         # Data models
├── routes/         # API routes
├── middleware/     # Middleware components
└── helpers/        # Utility functions
```

## Code Quality Standards

### 1. Error Handling

✅ **DO:**
```rust
pub async fn fetch_data() -> Result<Data, AppError> {
    let response = http_client
        .get(url)
        .send()
        .await?;  // Use ? for error propagation
    
    let data = response
        .json()
        .await
        .map_err(|e| AppError::Json(e))?;
    
    Ok(data)
}
```

❌ **DON'T:**
```rust
pub async fn fetch_data() -> Data {
    let response = http_client
        .get(url)
        .send()
        .await
        .unwrap();  // ❌ Never use unwrap() in production code
    
    response.json().await.expect("Failed")  // ❌ Never use expect()
}
```

### 2. Logging

✅ **DO:**
```rust
use tracing::{info, warn, error, debug};

pub async fn process_request() -> Result<(), AppError> {
    debug!("Starting request processing");
    
    let result = risky_operation().await;
    
    match result {
        Ok(data) => {
            info!("Successfully processed request");
            Ok(())
        }
        Err(e) => {
            error!("Failed to process request: {}", e);
            Err(e.into())
        }
    }
}
```

### 3. Database Queries

✅ **DO:**
```rust
use sea_orm::*;

// Fetch only needed columns
let users = User::find()
    .select_only()
    .column(user::Column::Id)
    .column(user::Column::Email)
    .filter(user::Column::Active.eq(true))
    .all(db)
    .await?;

// Use transactions for multiple operations
let txn = db.begin().await?;
// ... operations
txn.commit().await?;
```

### 4. Async Best Practices

✅ **DO:**
```rust
// Spawn CPU-intensive tasks
let result = tokio::task::spawn_blocking(|| {
    expensive_computation()
}).await?;

// Use semaphores for rate limiting
let semaphore = Arc::new(Semaphore::new(5));
let permit = semaphore.acquire().await?;
// ... do work
drop(permit);
```

### 5. Type Safety

✅ **DO:**
```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    #[serde(default)]
    pub email: String,
    
    #[serde(default)]
    pub name: String,
}

// Use newtype pattern for type safety
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(Uuid);
```

### 6. Testing

✅ **DO:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_user() {
        let user = create_user("test@example.com").await;
        assert!(user.is_ok());
        
        let user = user.unwrap();
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let result = create_user("duplicate@example.com").await;
        assert!(result.is_err());
    }
}
```

## Performance Optimization

### 1. Connection Pooling

```rust
// Database connection pool (already configured)
let mut opt = sea_orm::ConnectOptions::new(CONFIG.database_url.clone());
opt.max_connections(50)      // Max 50 connections
    .min_connections(5)       // Keep 5 warm connections
    .connect_timeout(Duration::from_secs(10))
    .idle_timeout(Duration::from_secs(10));
```

### 2. Redis Caching

```rust
use crate::helpers::cache::Cache;

// Cache expensive computations
let cache_key = format!("user:{}:profile", user_id);
let cached: Option<UserProfile> = Cache::get(&cache_key).await?;

if let Some(profile) = cached {
    return Ok(profile);
}

let profile = fetch_profile_from_db(user_id).await?;
Cache::set(&cache_key, &profile, 3600).await?;  // Cache for 1 hour

Ok(profile)
```

### 3. Batch Operations

```rust
// Batch insert
let users = vec![user1, user2, user3];
User::insert_many(users)
    .exec(db)
    .await?;

// Batch fetch with IN query
let ids = vec![1, 2, 3, 4, 5];
let users = User::find()
    .filter(user::Column::Id.is_in(ids))
    .all(db)
    .await?;
```

### 4. Streaming Large Responses

```rust
use axum::response::StreamBody;
use tokio_util::io::ReaderStream;

pub async fn download_large_file() -> impl IntoResponse {
    let file = tokio::fs::File::open("large_file.bin").await?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/octet-stream")],
        body,
    )
}
```

## Security Best Practices

### 1. Input Validation

```rust
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

pub async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<JsonResponse<User>, ErrorResponse> {
    // payload is already validated
    Ok(JsonResponse::ok(user))
}
```

### 2. Rate Limiting

```rust
// Already implemented in middleware
// Configure in routes with RateLimit extractor
use crate::middleware::rate_limit::RateLimit;

pub async fn protected_endpoint(
    RateLimit(()): RateLimit,
    // ... other extractors
) -> impl IntoResponse {
    // Rate limiting is already applied
}
```

### 3. Authentication

```rust
use crate::middleware::auth::Claims;

pub async fn protected_route(
    claims: Claims,  // Automatic JWT validation
) -> Result<JsonResponse<Data>, ErrorResponse> {
    let user_id = claims.sub;
    // ... use authenticated user_id
}
```

## API Documentation

### OpenAPI/Swagger

```rust
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn get_user(
    Path(id): Path<Uuid>,
) -> Result<JsonResponse<UserResponse>, ErrorResponse> {
    // ... implementation
}
```

## Monitoring & Observability

### 1. Structured Logging

```rust
use tracing::{info, instrument};

#[instrument(skip(db))]
pub async fn create_user(
    db: &DatabaseConnection,
    email: &str,
) -> Result<User, AppError> {
    info!(email = %email, "Creating new user");
    // ... implementation
}
```

### 2. Metrics

```rust
use metrics::{counter, histogram};

counter!("api.requests.total", "endpoint" => "/api/users").increment(1);
histogram!("api.request.duration", "endpoint" => "/api/users").record(duration);
```

### 3. Health Checks

```rust
// Health check endpoint (already implemented)
// GET /health
// Returns: {"status": "healthy", "database": "ok", "redis": "ok"}
```

## Deployment

### Environment Variables

Required environment variables:
```bash
DATABASE_URL=mysql://user:pass@localhost:3306/db
JWT_SECRET=your-secret-key-here
REDIS_URL=redis://localhost:6379
RUST_LOG=info
```

### Build for Production

```bash
# Build release binary
cargo build --release

# Run with production config
RUST_ENV=production ./target/release/rustexpress
```

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rustexpress /usr/local/bin/
CMD ["rustexpress"]
```

## Troubleshooting

### Common Issues

1. **Database Connection Errors**
   - Check DATABASE_URL format
   - Verify database is accessible
   - Check connection pool settings

2. **Redis Connection Errors**
   - Verify REDIS_URL is correct
   - Check Redis server is running
   - Review connection timeout settings

3. **High Memory Usage**
   - Review connection pool sizes
   - Check for memory leaks with valgrind
   - Monitor with tokio-console

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Enable tokio tracing
RUSTFLAGS="--cfg tokio_unstable" cargo run
```

## Resources

- [Axum Documentation](https://docs.rs/axum)
- [SeaORM Guide](https://www.sea-ql.org/SeaORM)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Tracing Documentation](https://docs.rs/tracing)
