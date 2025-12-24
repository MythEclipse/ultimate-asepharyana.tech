# RustExpress Framework

A high-performance, production-ready Rust framework built on Axum.

## ✨ Complete Feature Set

### Core Infrastructure

| Feature          | Module     | Description          |
| ---------------- | ---------- | -------------------- |
| Type-Safe Config | `config`   | Fail-fast validation |
| Auto-Routing     | `routes`   | File-based routing   |
| OpenAPI/Swagger  | `utoipa`   | Auto-generated docs  |
| SeaORM           | `entities` | Type-safe DB         |

### Request Handling

| Feature             | Module          | Description           |
| ------------------- | --------------- | --------------------- |
| ValidatedJson/Query | `extractors`    | Auto-validation (422) |
| Rate Limiting       | `ratelimit`     | 1000 req/sec          |
| Request ID          | `observability` | Per-request tracking  |
| Auth Middleware     | `middleware`    | JWT + Redis           |

### Observability

| Feature            | Module                     | Description         |
| ------------------ | -------------------------- | ------------------- |
| Prometheus Metrics | `observability/metrics`    | `/metrics` endpoint |
| Request Tracing    | `observability/request_id` | X-Request-ID header |
| Health Checks      | `health`                   | Liveness/readiness  |

### Services

| Feature         | Module            | Description       |
| --------------- | ----------------- | ----------------- |
| DI Container    | `di`              | Runtime injection |
| Event Bus       | `events`          | Pub/sub pattern   |
| Circuit Breaker | `circuit_breaker` | Fault tolerance   |
| Browser Pool    | `browser`         | Pooled tabs       |

### Background Processing

| Feature   | Module        | Description      |
| --------- | ------------- | ---------------- |
| Job Queue | `jobs`        | Redis-backed     |
| Scheduler | `scheduler`   | Cron tasks       |
| Worker    | `jobs/worker` | Async processing |

### Developer Experience

| Feature           | Module       | Description     |
| ----------------- | ------------ | --------------- |
| `rex` CLI         | `bin/rex`    | Code generation |
| TestApp           | `testing`    | In-memory tests |
| TypeScript Gen    | `typescript` | TS types        |
| Seeder            | `seeder`     | DB seeding      |
| API Versioning    | `versioning` | v1/v2 routing   |
| Graceful Shutdown | `graceful`   | Signal handling |

## 🚀 Quick Start

```bash
cargo build                     # Build
cargo run                       # Run server
cargo run --bin rex make:api users --full  # Generate API
cargo test                      # Run tests
```

## 📖 Usage Examples

### Request ID

```rust
use axum::Extension;
use rust::observability::RequestId;

async fn handler(Extension(req_id): Extension<RequestId>) {
    println!("Request: {}", req_id);
}
```

### Metrics

```rust
use rust::observability::{setup_metrics, MetricsHandler};

setup_metrics()?;
router.route("/metrics", get(MetricsHandler::handle))
```

### Graceful Shutdown

```rust
use rust::graceful::shutdown_signal;

axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;
```

### API Versioning

```rust
use rust::versioning::VersionedApi;

let app = VersionedApi::new()
    .v1(users_v1_routes())
    .v2(users_v2_routes())
    .build();
```

## License

MIT License
