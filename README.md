# 🚀 Ultimate Asepharyana Cloud

A modern, high-performance distributed services architecture powering the personal portfolio and various interactive features of **Asep Haryana Saputra**.

This project has evolved from a monorepo into a **decoupled services architecture**, leveraging Rust, TypeScript, and Docker for maximum scalability and reliability.

---

## 🏗️ System Architecture

The ecosystem consists of several specialized services communicating over a shared Docker network and managed via **Docker Compose** and **Coolify**.

### 🛠 Core Services

| Service                         | Technology                  | Description                                                                     |
| :------------------------------ | :-------------------------- | :------------------------------------------------------------------------------ |
| **[Rust API](apps/rust)**       | Rust (Axum, SeaORM)         | High-performance core backend for scraping, media proxy, and heavy computation. |
| **[Elysia API](apps/elysia)**   | TypeScript (Elysia.js, Bun) | Real-time features, interactive quizzes (WebSocket), and session handling.      |
| **[SolidJS Web](apps/solidjs)** | SolidJS, TailwindCSS        | The main client-side application for the portfolio and media viewer.            |
| **[Leptos Web](apps/leptos)**   | Rust (Leptos, WASM)         | experimental high-performance frontend components compiled to WebAssembly.      |
| **[Visuals](apps/visuals)**     | Rust (Bevy/WGPU)            | Specialized WebGL/WebGPU rendering and interactive visual experiments.          |

### 🛰 Infrastructure sidecars

- **MySQL 8**: Primary relational data storage.
- **Redis**: High-speed caching, session storage, and request coalescing.
- **Minio**: S3-compatible object storage for media assets.
- **Browserless**: Dedicated headless Chrome cluster for robust web scraping.

---

## ⚡ Deployment & CI/CD

We use a **GitOps-driven deployment strategy** with automated tagging and manifest updates.

### ⚓ Tagging Strategy (SHA-based)

Unlike the traditional `latest` tag approach, our pipeline generates unique tags based on **GitHub Commit SHA** (`sha-<short-sha>`).

1. **Build**: GitHub Actions builds individual services only if code within their directory changes.
2. **Manifest Update**: Upon a successful build, the CI automatically updates `docker-compose.yml` with the new specific SHA tag.
3. **Trigger**: Coolify detects the manifest change and performs a precise rolling update to the target environment.

### 🚀 Local Development

While each service can be run independently (via `cargo run` or `bun dev`), the entire stack is best initialized via Docker:

```bash
# Clone the repository
git clone https://github.com/MythEclipse/ultimate-asepharyana.cloud.git
cd ultimate-asepharyana.cloud

# Start the entire ecosystem
docker compose up -d
```

---

## 🧪 Advanced Features

### 🖼️ Intelligent Image Cache (Rust API)

The Rust service provides a sophisticated image proxy and caching mechanism:

- **Resilient Uploads**: Multi-provider failover for CDN uploads (Picser, Leapcell, Vercel).
- **Audit & Repair**: Automated auditing via `POST /api/proxy/image-cache/audit` that verifies CDN link accessibility and re-uploads broken images.
- **Background Caching**: Non-blocking "Lazy Mode" for instantaneous user response while ensuring eventual consistency in the CDN.

### ⚙️ Scaffold System

Generate new Rust endpoints with professional-grade boilerplate:

```bash
cd apps/rust
cargo run --bin scaffold -- my-category/new-feature
```

---

## 📜 Documentation

- **Rust API Reference**: `https://rust.asepharyana.tech/docs`
- **Elysia API Reference**: `https://elysia.asepharyana.tech/docs`

---

**Author:** [Asep Haryana Saputra](https://asepharyana.cloud)  
**License:** MIT
