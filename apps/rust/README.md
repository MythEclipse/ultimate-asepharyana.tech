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

Or use the CLI to scaffold a new project:

```bash
cargo install rustexpress
rex new my-app
cd my-app
rex serve
```

## ✨ Complete Feature Set (39 Modules)

### 🔐 Authentication & Security

| Feature        | Module                | Description             |
| -------------- | --------------------- | ----------------------- |
| JWT Auth       | `utils/auth`          | Token-based auth        |
| OAuth2         | `auth/oauth`          | Google, GitHub, Discord |
| 2FA/TOTP       | `auth/totp`           | Time-based OTP          |
| API Keys       | `auth/api_key`        | API key management      |
| Password Reset | `auth/password_reset` | Secure reset tokens     |
| Remember Me    | `auth/remember_me`    | Long-term tokens        |
| Session        | `session`             | Redis-backed sessions   |
| CSRF           | `security/csrf`       | Cross-site protection   |
| Rate Limiting  | `ratelimit`           | Request throttling      |

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

### 📧 Communication

| Feature         | Module                   | Description            |
| --------------- | ------------------------ | ---------------------- |
| Notifications   | `notifications`          | Multi-channel alerts   |
| Email Templates | `helpers/email_template` | HTML email builder     |
| Broadcasting    | `broadcasting`           | Real-time SSE          |
| Webhooks        | `webhooks`               | Signature verification |

### 🏢 Enterprise Features

| Feature          | Module                   | Description           |
| ---------------- | ------------------------ | --------------------- |
| Multi-tenancy    | `helpers/tenant`         | SaaS isolation        |
| Feature Flags    | `features`               | Gradual rollouts      |
| Authorization    | `auth`                   | RBAC/policies         |
| Audit Logging    | `audit`                  | Activity tracking     |
| Maintenance Mode | `middleware/maintenance` | Downtime handling     |
| Health Checks    | `helpers/health_check`   | Dependency monitoring |

### 🛠 Infrastructure

| Feature        | Module                   | Description          |
| -------------- | ------------------------ | -------------------- |
| Cache Tags     | `helpers/cache_tags`     | Intelligent caching  |
| Encryption     | `helpers/encryption`     | AES-256 at rest      |
| Storage        | `storage`                | Local + S3 drivers   |
| i18n           | `i18n`                   | Translations         |
| Query Profiler | `helpers/query_profiler` | Performance analysis |
| Console/CLI    | `helpers/console`        | Command builder      |

### 🔧 Background Processing

| Feature   | Module        | Description       |
| --------- | ------------- | ----------------- |
| Job Queue | `jobs`        | Redis-backed jobs |
| Scheduler | `scheduler`   | Cron tasks        |
| Worker    | `jobs/worker` | Async processing  |

## 🚀 Quick Start

```bash
cargo build                     # Build
cargo run                       # Run server on :8080
cargo test                      # Run tests
```

## 📖 Usage Examples

### Form Validation

```rust
use crate::helpers::form_request::{ValidationRules, validate};

let mut rules = ValidationRules::new();
rules.required("email").email("email").min_length("password", 8);

let validation = validate(&data, &rules);
if !validation.is_valid() {
    return Err(AppError::Other(validation.errors[0].message.clone()));
}
```

### Email Templates

```rust
use crate::helpers::email_template::welcome_email;

let email = welcome_email("John", "https://example.com/verify?token=abc");
// Returns EmailTemplate { subject, html, text }
```

### API Versioning

```rust
use crate::helpers::versioning::{ApiVersion, extract_version};

let version = extract_version(&headers);
if version.at_least(2, 0) {
    // Use v2 response format
}
```

### Searchable

```rust
use crate::helpers::searchable::{calculate_score, SearchQuery};

let score = calculate_score("John Doe", "john", true); // fuzzy match
```

### Multi-tenancy

```rust
use crate::helpers::tenant::{TenantManager, Tenant};

let tenant = manager.get_by_domain("acme.example.com").await?;
```

## 📁 Project Structure

```
src/
├── auth/           # OAuth, TOTP, API Keys, Password Reset
├── audit/          # Audit logging
├── broadcasting/   # Real-time SSE
├── features/       # Feature flags
├── helpers/        # 20+ utility modules
├── i18n/           # Internationalization
├── middleware/     # Auth, logging, maintenance
├── notifications/  # Multi-channel notifications
├── routes/         # API routes
├── security/       # CSRF protection
├── session/        # Session management
├── storage/        # File storage (Local/S3)
└── webhooks/       # Webhook handling
```

## License

MIT License
