# Ultimate Asepharyana Tech

Monorepo untuk ekosistem portfolio dan layanan pendukung milik Asep Haryana Saputra.
Arsitektur dipisah ke beberapa service agar frontend, API, dan visual engine bisa dikembangkan serta di-deploy secara independen.

## Services

| Service | Path | Default Local Port | Notes |
| :-- | :-- | :-- | :-- |
| Rust API | `apps/rust` | `4091` | API utama (Axum + SeaORM), scraping, image proxy/cache, metrics, OpenAPI docs |
| Elysia API | `apps/elysia` | `4092` | API realtime/auth/chat/quiz (Elysia + Bun + Drizzle + Redis + MinIO) |
| SolidJS Web | `apps/solidjs` | `4090` | Frontend SSR utama (SolidStart) |
| Next.js Web | `apps/nextjs` | `3000` | Frontend alternatif/eksperimen UI (Next.js App Router) |
| Leptos Web | `apps/leptos` | `3000` (Trunk) | Frontend WASM berbasis Leptos |
| Visuals | `apps/visuals` | `3001` (Trunk) | Eksperimen visual berbasis Bevy + WebGPU |

## Infrastructure

File compose berada di `infra/compose`:

- `shared.yml`: MySQL, Redis, MinIO
- `rust.yml`, `elysia.yml`, `solidjs.yml`, `nextjs.yml`, `leptos.yml`, `visuals.yml`: manifest deploy per service (image GHCR bertag SHA)

Dockerfile per service berada di folder `docker/`.

## Local Development

### 1) Jalankan dependency bersama

```bash
docker compose -f infra/compose/shared.yml up -d
```

### 2) Jalankan service yang dibutuhkan

```bash
# Rust API
cd apps/rust
cargo run

# Elysia API
cd apps/elysia
bun install
bun run dev

# SolidJS web
cd apps/solidjs
bun install
bun run dev

# Next.js web
cd apps/nextjs
npm install
npm run dev

# Leptos web (WASM)
cd apps/leptos
bun install
trunk serve
```

## API Docs and Monitoring

- Rust OpenAPI: `/docs`
- Rust metrics (Prometheus): `/metrics`
- Elysia Swagger: `/docs`
- Elysia AsyncAPI viewer: `/docs-ws`

## Deployment Notes

- Pipeline memakai image tag berbasis commit SHA (`sha-<short-sha>`), bukan `latest`.
- Setiap perubahan service akan menghasilkan image baru dan pembaruan manifest compose service terkait.

## License

MIT
