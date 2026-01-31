# GEMINI.md - Codebase & Structure Explanation

This document provides an overview of the codebase structure, specifically focusing on the Rust application (`apps/rust`) within the monorepo.

## 🌍 Monorepo Context

This project is a **monorepo** managed with **Turborepo** and **Bun**.

- **`apps/`**: Contains the main applications.
  - **`rust/`**: The robust backend server built with Rust & Axum. (Focus of this doc)
  - **`elysia/`**: Likely a lightweight or edge-optimized service using ElysiaJS (Bun).
  - **`solidjs/`**: The frontend application built with SolidJS.
- **`packages/`**: Shared libraries and services.
  - **`services/`**: Shared logic/services (currently contains code shared across apps).

---

## 🦀 `apps/rust` - Rust Backend Server

This is a production-ready Web API framework built on **Axum**. It includes a wide range of "batteries-included" features like Database ORM, caching, scheduling, scraping, and more.

### 🛠 Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) (0.8.8) - High-performance web framework.
- **Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime.
- **Database ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) - Async & dynamic ORM for Rust (MySQL via `sqlx`).
- **Caching**: `deadpool-redis` & `redis` - Redis connection pooling and operations.
- **GraphQL**: `async-graphql` - For GraphQL API endpoints.
- **OpenAPI/Swagger**: `utoipa` - Auto-generated API documentation.
- **Background Jobs**: `tokio-cron-scheduler`.
- **Scraping**: `scraper` (HTML parsing) & `chromiumoxide` (Headless Chrome).

### 📂 Folder Structure (`apps/rust/src`)

The source code is organized by feature and technical concern (Clean Architecture / Vertical Slice hybrid).

| Directory         | Description                                                                                                        |
| :---------------- | :----------------------------------------------------------------------------------------------------------------- |
| **`bin/`**        | Binary entry points (e.g., `scaffold`, `rex` CLI tools).                                                           |
| **`config/`**     | Configuration loading (referenced in code, usually maps to `.env`).                                                |
| **`entities/`**   | **SeaORM Entities**. Represents database tables. Auto-generated or manually defined models.                        |
| **`routes/`**     | **API Route Handlers**. Organized hierarchies (e.g., `api/anime`, `api/v1`). Defining endpoints (GET, POST, etc.). |
| **`models/`**     | Data Transfer Objects (DTOs), request/response structs, and internal domain models.                                |
| **`services/`**   | Business logic layer. Handlers (in `routes/`) calls Services, which call Repositories/Entities.                    |
| **`scraping/`**   | Logic for web scraping (parsers, extractors). Specific modules for different sources (e.g., anime sites).          |
| **`features/`**   | Feature-specific logic that might not fit strictly into "services" or "routes".                                    |
| **`helpers/`**    | Utility functions, shared formatting, and small helper traits.                                                     |
| **`middleware/`** | Axum middlewares (Auth, Logging, CORS, etc.).                                                                      |
| **`di/`**         | Dependency Injection setup (Application State).                                                                    |
| **`events/`**     | Event-driven architecture components (Event emitters/listeners).                                                   |
| **`jobs/`**       | Background jobs and cron tasks.                                                                                    |
| **`graphql/`**    | GraphQL schemas, resolvers, and mutations.                                                                         |
| **`ws/`**         | WebSocket handlers and logic.                                                                                      |
| **`seeder/`**     | Database seeding logic.                                                                                            |
| **`testing/`**    | Test helpers and shared test usage code.                                                                           |

### 🔑 Key Concepts & Flows

1.  **Request Flow**:
    `Request` -> `Middleware` -> `Router` -> `Handler (in routes/)` -> `Service` -> `Entity/DB`.
2.  **Application State (`di`)**:
    The generic `AppState` struct (or similar) is passed to handlers via `Extension` or `State`, containing DB connections, Redis pools, and config.
3.  **Scraping Flow**:
    Routes like `api/anime2/*` trigger scrapers. These likely use `reqwest` or `chromiumoxide` to fetch pages, `scraper` to parse HTML, and return JSON responses. Caching (Redis) is often applied here to avoid re-fetching.

### 📜 Common Commands

- **Run Server**: `cargo run` (or `cargo watch -x run` for hot reload).
- **Run CLI**: `cargo run --bin rex` (Scaffolding tool).
- **Test**: `cargo test`.
- **Build**: `cargo build --release`.

---

This structure allows for scalable development, separating concerns while keeping related code (like a specific route's logic) relatively close.
