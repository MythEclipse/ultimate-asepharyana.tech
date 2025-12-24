# RustExpress Framework

A high-performance, developer-friendly Rust framework built on Axum with batteries included.

## ✨ Features

### Core Framework

- **Type-Safe Configuration** - Fail-fast config validation at startup
- **Auto-Routing** - File-based routing with automatic handler discovery
- **OpenAPI/Swagger** - Auto-generated API documentation
- **SeaORM** - Type-safe database operations with MySQL

### Request Handling

- **ValidatedJson** - Automatic request validation with 422 responses
- **ValidatedQuery** - Query parameter validation
- **Auth Middleware** - JWT authentication with Redis blacklist

### Dependency Injection

- **ServiceContainer** - Runtime DI with provider pattern
- **MiddlewareRegistry** - Named middleware groups

### Background Processing

- **Job Queue** - Redis-backed background jobs with retries
- **Worker** - Async job processing with configurable concurrency

### Browser Automation

- **BrowserPool** - Single browser with pooled tabs for scraping
- **PooledTab** - Navigate, click, type, evaluate JS, screenshot

### Developer Experience

- **`rex` CLI** - Code generation (models, controllers, APIs)
- **TestApp** - In-memory integration testing
- **Hot Reload** - Fast development iteration

## 🚀 Quick Start

```bash
# Install
cargo build

# Run dev server
cargo run

# Generate API resources
cargo run --bin rex make:api products --full

# Run tests
cargo test
```

## 📁 Project Structure

```
src/
├── browser/        # Browser tab pooling
├── config.rs       # Type-safe configuration
├── di/             # Dependency injection
├── entities/       # SeaORM models
├── extractors/     # ValidatedJson, ValidatedQuery
├── jobs/           # Background job system
├── middleware/     # Auth, rate limiting, registry
├── routes/         # API handlers (auto-discovered)
├── testing/        # TestApp utilities
└── main.rs         # Entry point
```

## 🔧 Configuration

```env
DATABASE_URL=mysql://user:password@localhost:3306/database
JWT_SECRET=your_secret_key
REDIS_URL=redis://localhost:6379
APP_LOG_LEVEL=info
APP_SERVER_PORT=4091
```

## 📖 Usage Examples

### Validated Requests

```rust
use rust::extractors::ValidatedJson;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

async fn create_user(ValidatedJson(data): ValidatedJson<CreateUser>) {
    // data is guaranteed valid
}
```

### Background Jobs

```rust
use rust::jobs::{Job, JobDispatcher};

#[derive(Serialize, Deserialize)]
struct SendEmail { user_id: String }

#[async_trait]
impl Job for SendEmail {
    const NAME: &'static str = "send_email";
    async fn handle(&self) -> anyhow::Result<()> { Ok(()) }
}

dispatcher.dispatch(SendEmail { user_id: "123".into() }).await?;
```

### Browser Scraping

```rust
use rust::browser::{BrowserPool, BrowserPoolConfig, get_browser_pool};

// Get tab from pool
let tab = get_browser_pool().unwrap().get_tab().await?;
tab.goto("https://example.com").await?;
let html = tab.content().await?;
// Tab auto-returns to pool when dropped
```

### Integration Testing

```rust
use rust::testing::TestApp;

#[tokio::test]
async fn test_health() {
    let app = TestApp::with_router(create_router());
    app.get("/health").await.assert_status(200);
}
```

## 🔨 CLI Commands

```bash
cargo run --bin rex make:model User          # Generate model
cargo run --bin rex make:migration create_users # Generate migration
cargo run --bin rex make:controller users --crud # Generate CRUD controller
cargo run --bin rex make:api products --full # Generate complete stack
```

## 📦 Key Dependencies

| Crate          | Purpose               |
| -------------- | --------------------- |
| axum           | Web framework         |
| sea-orm        | ORM                   |
| tokio          | Async runtime         |
| validator      | Request validation    |
| chromiumoxide  | Browser automation    |
| deadpool-redis | Redis connection pool |

## License

MIT License
