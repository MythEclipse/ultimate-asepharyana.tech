# 🚀 Ultimate Asepharyana Cloud

My personal portfolio monorepo – a full-stack web application showcasing various projects, APIs, and interactive features.

[![Turborepo](https://img.shields.io/badge/Turborepo-EF4444?style=for-the-badge&logo=turborepo&logoColor=white)](https://turbo.build/)
[![Bun](https://img.shields.io/badge/Bun-%23000000.svg?style=for-the-badge&logo=bun&logoColor=white)](https://bun.sh/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![SolidJS](https://img.shields.io/badge/SolidJS-2c4f7c?style=for-the-badge&logo=solid&logoColor=white)](https://www.solidjs.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

## 📦 Tech Stack

| Layer                    | Technology                       |
| ------------------------ | -------------------------------- |
| **Frontend**             | SolidJS, TailwindCSS, Kobalte UI |
| **Backend (TypeScript)** | Elysia.js (Bun runtime)          |
| **Backend (Rust)**       | Axum, SeaORM, Utoipa (OpenAPI)   |
| **Database**             | MySQL, Redis                     |
| **Build System**         | Turborepo + Bun Workspaces       |
| **Documentation**        | Swagger UI (auto-generated)      |

## 🏗️ Project Structure

```
ultimate-asepharyana.cloud/
├── apps/
│   ├── solidjs/          # Frontend - SolidJS with SSR
│   ├── elysia/           # Backend - Elysia.js (real-time, quiz, auth)
│   └── rust/             # Backend - Rust Axum (scraping APIs)
├── packages/
│   └── services/         # Shared services (Drizzle ORM, utilities)
└── turbo.json            # Turborepo configuration
```

## 🚀 Getting Started

### Prerequisites

- [Bun](https://bun.sh/) >= 1.3.5
- [Rust](https://rustup.rs/) (latest stable)
- MySQL & Redis instances

### Installation

```bash
# Clone the repository
git clone https://github.com/MythEclipse/ultimate-asepharyana.cloud.git
cd ultimate-asepharyana.cloud

# Install dependencies
bun install

# Copy environment variables
cp .env.example .env
# Edit .env with your configuration
```

### Development

```bash
# Run all apps in development mode
bun run dev

# Run specific apps
bun run elysia:dev      # Elysia backend
bun run rust:dev        # Rust backend

# Build all apps
bun run build
```

## 🔗 API Endpoints

### Rust API (Port 4091)

| Endpoint        | Description                |
| --------------- | -------------------------- |
| `/api/anime/*`  | Anime scraping & streaming |
| `/api/anime2/*` | Alternative anime source   |
| `/api/komik/*`  | Manga/comic scraping       |
| `/api/proxy/*`  | Media proxy service        |
| `/api/auth/*`   | Authentication endpoints   |
| `/docs`         | Swagger UI documentation   |

### Elysia API (Port 4092)

| Endpoint      | Description                     |
| ------------- | ------------------------------- |
| `/api/quiz/*` | Real-time quiz game (WebSocket) |
| `/api/auth/*` | User authentication             |
| `/swagger`    | API documentation               |

### SolidJS Frontend (Port 4090)

| Route    | Description              |
| -------- | ------------------------ |
| `/`      | Landing page / Portfolio |
| `/anime` | Anime streaming viewer   |
| `/komik` | Manga/comic reader       |

## 🛠️ Available Scripts

| Script               | Description                   |
| -------------------- | ----------------------------- |
| `bun run dev`        | Run all apps in dev mode      |
| `bun run build`      | Build all apps for production |
| `bun run lint`       | Lint all packages             |
| `bun run format`     | Format code with Prettier     |
| `bun run rust:build` | Build Rust backend (release)  |
| `bun run copyenv`    | Copy root .env to all apps    |

## 🔧 Rust Scaffold System

Generate new API endpoints quickly using the scaffold CLI:

```bash
# Create a new endpoint
cargo run --bin scaffold -- products/list

# Build to generate handler template
cargo build

# Edit the generated handler
# src/routes/api/products/list.rs
```

See [Scaffold Documentation](apps/rust/src/bin/scaffold.md) for more details.

## 📄 License

This project is licensed under the MIT License.

---

**Author:** [Asep Haryana Saputra](https://asepharyana.cloud)
