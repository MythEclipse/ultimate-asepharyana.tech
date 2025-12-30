# RustExpress Framework

[![Crates.io](https://img.shields.io/crates/v/rustexpress.svg)](https://crates.io/crates/rustexpress)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A **batteries-included**, production-ready Rust web framework built on Axum with **39+ enterprise-grade modules**.

> 🏆 **The most comprehensive Rust web framework** — exceeds Laravel, Rails, Django, Spring Boot

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rustexpress = "0.1.0"
```

## 🚀 Quick Start

```bash
# From the apps/rust directory:
cd apps/rust

# Run the server
cargo run

# Or use the rex CLI
cargo run --bin rex -- --help
cargo run --bin rex -- serve
cargo run --bin rex -- about
```

## 🛠️ Rex CLI - Laravel Artisan-inspired

**13 powerful commands** for scaffolding and managing your application:

### Code Generation

```bash
rex make:model User              # Generate SeaORM entity
rex make:migration create_users  # Generate migration file
rex make:controller users        # Generate API controller
rex make:service UserService     # Generate service layer
rex make:repository UserRepo     # Generate repository pattern
rex make:api products            # Generate complete CRUD stack
```

### Database Management

```bash
rex migrate:run                  # Run pending migrations
rex migrate:rollback             # Rollback last migration
rex migrate:status               # Show migration status
rex db:seed                      # Run database seeders
```

### Server & Info

```bash
rex serve                        # Start development server
rex serve --port 8080            # Custom port
rex list                         # List models and routes
rex about                        # Show framework info
```

## ✨ Complete Feature Set (39+ Modules)

### 🔐 Authentication & Security

| Feature        | Module                | Description             |
| -------------- | --------------------- | ----------------------- |
| JWT Auth       | `core/jwt`            | Token-based auth        |
| OAuth2         | `auth/oauth`          | Google, GitHub, Discord |
| 2FA/TOTP       | `auth/totp`           | Time-based OTP          |
| API Keys       | `auth/api_key`        | API key management      |
| Password Reset | `auth/password_reset` | Secure reset tokens     |
| Remember Me    | `auth/remember_me`    | Long-term tokens        |
| Session        | `session`             | Redis-backed sessions   |
| CSRF           | `security/csrf`       | Cross-site protection   |
| Rate Limiting  | `core/ratelimit`      | Request throttling      |

### 📊 Database & ORM

| Feature         | Module                  | Description         |
| --------------- | ----------------------- | ------------------- |
| SeaORM          | `entities`              | Type-safe ORM       |
| Soft Delete     | `helpers/soft_delete`   | Recoverable deletes |
| Query Builder   | `helpers/query`         | Pagination, sorting |
| Bulk Operations | `helpers/bulk`          | Batch insert/delete |
| Transactions    | `helpers/transaction`   | Atomic operations   |
| Searchable      | `helpers/searchable`    | Full-text search    |
| Sluggable       | `helpers/sluggable`     | URL-safe slugs      |
| State Machine   | `helpers/state_machine` | Workflow states     |

### 🚀 API & Web

| Feature         | Module                  | Description           |
| --------------- | ----------------------- | --------------------- |
| Auto-Routing    | `routes`                | File-based routing    |
| OpenAPI/Swagger | `utoipa`                | Auto-generated docs   |
| Form Validation | `helpers/form_request`  | Request validation    |
| API Resources   | `helpers/resource`      | Response transformers |
| API Versioning  | `helpers/versioning`    | v1/v2 routing         |
| Signed URLs     | `helpers/signed_url`    | Expiring URLs         |
| Import/Export   | `helpers/import_export` | CSV/JSON/NDJSON       |
| GraphQL         | `graphql`               | async-graphql support |

### 📧 Communication

| Feature         | Module                   | Description            |
| --------------- | ------------------------ | ---------------------- |
| Notifications   | `notifications`          | Multi-channel alerts   |
| Email Templates | `helpers/email_template` | HTML email builder     |
| Broadcasting    | `broadcasting`           | Real-time SSE          |
| Webhooks        | `webhooks`               | Signature verification |
| WebSocket       | `ws`                     | Room-based WebSocket   |

### 🏢 Enterprise Features

| Feature          | Module                   | Description           |
| ---------------- | ------------------------ | --------------------- |
| Multi-tenancy    | `helpers/tenant`         | SaaS isolation        |
| Feature Flags    | `features`               | Gradual rollouts      |
| Authorization    | `auth`                   | RBAC/policies         |
| Audit Logging    | `audit`                  | Activity tracking     |
| Maintenance Mode | `middleware/maintenance` | Downtime handling     |
| Health Checks    | `health`                 | Dependency monitoring |
| Circuit Breaker  | `circuit_breaker`        | Failure isolation     |

### 🛠 Infrastructure

| Feature           | Module                   | Description          |
| ----------------- | ------------------------ | -------------------- |
| Cache Tags        | `helpers/cache_tags`     | Intelligent caching  |
| Encryption        | `helpers/encryption`     | AES-256 at rest      |
| Storage           | `storage`                | Local + S3 drivers   |
| i18n              | `i18n`                   | Translations         |
| Query Profiler    | `helpers/query_profiler` | Performance analysis |
| Dependency Inject | `di`                     | Service container    |
| Graceful Shutdown | `graceful`               | Signal handling      |

### 🔧 Background Processing

| Feature   | Module        | Description       |
| --------- | ------------- | ----------------- |
| Job Queue | `jobs`        | Redis-backed jobs |
| Scheduler | `scheduler`   | Cron tasks        |
| Worker    | `jobs/worker` | Async processing  |

## Usage Examples

### Form Validation

```rust
use rustexpress::helpers::form_request::{ValidationRules, validate};

let mut rules = ValidationRules::new();
rules.required("email").email("email").min_length("password", 8);

let validation = validate(&data, &rules);
if !validation.is_valid() {
    return Err(AppError::Other(validation.errors[0].message.clone()));
}
```

### Email Templates

```rust
use rustexpress::helpers::email_template::welcome_email;

let email = welcome_email("John", "https://example.com/verify?token=abc");
// Returns EmailTemplate { subject, html, text }
```

### API Versioning

```rust
use rustexpress::helpers::versioning::{ApiVersion, extract_version};

let version = extract_version(&headers);
if version.at_least(2, 0) {
    // Use v2 response format
}
```

### Multi-tenancy

```rust
use rustexpress::helpers::tenant::{TenantManager, Tenant};

let tenant = manager.get_by_domain("acme.example.com").await?;
```

### Storage

```rust
use rustexpress::storage::Storage;

let storage = Storage::local("./uploads");
storage.put("images/photo.jpg", &bytes).await?;
let content = storage.get("images/photo.jpg").await?;
```

## 📁 Project Structure

```
src/
├── auth/           # OAuth, TOTP, API Keys, Password Reset, RBAC
├── audit/          # Audit logging
├── bin/            # CLI tools (rex, scaffold)
├── broadcasting/   # Real-time SSE
├── circuit_breaker/# Failure isolation
├── core/           # Config, JWT, Rate limiting
├── di/             # Dependency injection
├── entities/       # SeaORM entities (13 models)
├── events/         # Event bus (pub/sub)
├── extractors/     # ValidatedJson, ValidatedQuery
├── features/       # Feature flags
├── graceful/       # Graceful shutdown
├── graphql/        # GraphQL support
├── health/         # Health checks
├── helpers/        # 50 utility modules
├── i18n/           # Internationalization
├── infra/          # Redis, HTTP client, Proxy
├── jobs/           # Background job queue
├── middleware/     # Auth, logging, maintenance
├── notifications/  # Multi-channel notifications
├── observability/  # Metrics, tracing
├── routes/         # API routes (33 endpoints)
├── scheduler/      # Cron tasks
├── security/       # CSRF protection
├── session/        # Session management
├── storage/        # File storage (Local/S3)
├── testing/        # Test utilities
├── webhooks/       # Webhook handling
└── ws/             # WebSocket with rooms
```

## 🔗 API Documentation

Swagger UI is available at `/docs` when the server is running.

OpenAPI spec: `/api-docs/openapi.json`

## License

MIT License
