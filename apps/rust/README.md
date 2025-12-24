# RustExpress Framework

A high-performance, developer-friendly Rust framework built on Axum with batteries included.

## ✨ Features

### Core Infrastructure

| Feature          | Module     | Description                 |
| ---------------- | ---------- | --------------------------- |
| Type-Safe Config | `config`   | Fail-fast config validation |
| Auto-Routing     | `routes`   | File-based routing          |
| OpenAPI/Swagger  | `utoipa`   | Auto-generated docs         |
| SeaORM           | `entities` | Type-safe DB                |

### Request Handling

| Feature         | Module       | Description              |
| --------------- | ------------ | ------------------------ |
| ValidatedJson   | `extractors` | Auto-validation with 422 |
| ValidatedQuery  | `extractors` | Query param validation   |
| Rate Limiting   | `ratelimit`  | 1000 req/sec (governor)  |
| Auth Middleware | `middleware` | JWT + Redis blacklist    |

### Services

| Feature         | Module            | Description                  |
| --------------- | ----------------- | ---------------------------- |
| DI Container    | `di`              | Runtime dependency injection |
| Event Bus       | `events`          | Pub/sub between modules      |
| Circuit Breaker | `circuit_breaker` | Protect external calls       |
| Browser Pool    | `browser`         | Single browser, pooled tabs  |

### Background Processing

| Feature   | Module        | Description          |
| --------- | ------------- | -------------------- |
| Job Queue | `jobs`        | Redis-backed jobs    |
| Scheduler | `scheduler`   | Cron-based tasks     |
| Worker    | `jobs/worker` | Async job processing |

### Developer Experience

| Feature        | Module       | Description               |
| -------------- | ------------ | ------------------------- |
| `rex` CLI      | `bin/rex`    | Code generation           |
| TestApp        | `testing`    | In-memory testing         |
| TypeScript Gen | `typescript` | TS type generation        |
| Seeder         | `seeder`     | Database seeding          |
| Health Checks  | `health`     | Liveness/readiness probes |

## 🚀 Quick Start

```bash
cargo build                     # Build
cargo run                       # Run server
cargo run --bin rex make:api users --full  # Generate API
cargo test                      # Run tests
```

## 📁 Project Structure

```
src/
├── browser/        # Browser tab pooling
├── circuit_breaker/# External service protection
├── config.rs       # Type-safe configuration
├── di/             # Dependency injection
├── events/         # Event bus (pub/sub)
├── extractors/     # ValidatedJson, ValidatedQuery
├── health/         # Health check endpoints
├── jobs/           # Background job system
├── middleware/     # Auth, rate limiting
├── ratelimit.rs    # Rate limiter (1000/sec)
├── scheduler/      # Cron-based tasks
├── seeder/         # Database seeding
├── testing/        # TestApp utilities
├── typescript/     # TypeScript generation
└── main.rs
```

## 🔧 Configuration

```env
DATABASE_URL=mysql://user:pass@localhost:3306/db
JWT_SECRET=your_secret
REDIS_URL=redis://localhost:6379
APP_LOG_LEVEL=info
```

## 📖 Usage Examples

### Rate Limiting

```rust
use axum::middleware::from_fn;

router.layer(from_fn(rate_limit_middleware))
```

### Event Bus

```rust
let bus = EventBus::new();
bus.publish(UserRegistered { user_id: "123".into(), .. }).await;
```

### Circuit Breaker

```rust
let breaker = CircuitBreaker::new("api", CircuitBreakerConfig::default());
let result = breaker.call(|| client.get(url)).await;
```

### Scheduler

```rust
let scheduler = Scheduler::new().await?;
scheduler.add_job("cleanup", "0 * * * * *", || async { ... }).await?;
scheduler.start().await?;
```

## License

MIT License
