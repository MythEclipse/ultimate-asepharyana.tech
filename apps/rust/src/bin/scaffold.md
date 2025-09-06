# Panduan Lengkap Scaffold

Panduan lengkap untuk membuat handler API menggunakan scaffolder di proyek ini.
File yang dihasilkan: src/routes/api/<path>.rs

Prasyarat:
- Pastikan env: PORT dan JWT_SECRET diset (lihat `.env.example` jika ada).
- Scaffold dapat dijalankan dari workspace root atau dari direktori `apps/rust/`.
- Setelah membuat file kosong, jalankan `cargo build` supaya [`apps/rust/build.rs`](apps/rust/build.rs:1) menghasilkan template handler.

Ringkasan tipe endpoint:
- Static: path tetap, contoh: /api/products/list
- Dynamic: path dinamis dengan parameter, contoh: /api/products/detail/product_id → /api/products/detail/{id}
- Params (query): query string, contoh: /api/products/search?q=sepatu&sort=price_desc

Pengenalan Dynamic Routes:
Sistem secara otomatis mendeteksi segmen dinamis berdasarkan keberadaan `//! DYNAMIC_ROUTE` di awal file handler, dan nama filenya (misalnya, `id.rs` untuk parameter `id`).
Contoh: `products/detail/id` akan menghasilkan `products/detail/id.rs` dengan `//! DYNAMIC_ROUTE` di dalamnya.

1) Endpoint Static (tanpa parameter)
------------------------------------------------------------
- Perintah scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- products/list

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/products/list.rs`](apps/rust/src/routes/api/products/list.rs:1)

Contoh handler (Rust):

```rust
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse {
    pub message: String,
}

pub async fn list() -> impl IntoResponse {
    Json(ListResponse { message: "All products".to_string() })
}
```

Tes endpoint:

curl http://127.0.0.1:3000/api/products/list

2) Endpoint Dynamic / Path Parameter
------------------------------------------------------------
- Untuk path dinamis, gunakan nama parameter langsung tanpa kurung siku:

  cargo run --bin scaffold -- products/detail/product_id

- Sistem akan otomatis mendeteksi pola dinamis dan mengkonversi:
  - `product_id` → parameter path `id` dengan route `/products/detail/{id}`
  - `user_slug` → parameter path `slug` dengan route `/users/profile/{slug}`
  - `order_uuid` → parameter path `uuid` dengan route `/orders/detail/{uuid}`

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/products/detail/product_id.rs`](apps/rust/src/routes/api/products/detail/product_id.rs:1)

Contoh handler untuk endpoint dinamis (sesuai template yang dihasilkan):

```rust
use axum::{response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use serde_json;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/products/detail/product_id";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the products/detail/product_id endpoint.";
pub const ENDPOINT_TAG: &str = "products/detail/product_id";
pub const OPERATION_ID: &str = "products/detail/product_id";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

/// Response structure for the ProductId endpoint.
/// Replace `serde_json::Value` with your actual data types and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,
}

pub async fn product_id(Path(id): Path<String>) -> impl IntoResponse {
    Json(DetailResponse {
        message: format!("Hello from product_id with parameters: id: {id}"),
        data: serde_json::json!({"id": "id"}),
    })
}

/// Handles GET requests for the products/detail/product_id endpoint.
pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(product_id))
}
```

Contoh handler dengan UUID:

```rust
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;

pub async fn product_id(Path(id): Path<String>) -> impl IntoResponse {
    let parsed = Uuid::parse_str(&id).ok();
    Json(ProductResponse { message: format!("Parsed UUID: {:?}", parsed) })
}
```

Tes:

curl http://127.0.0.1:3000/api/products/detail/sepatu-lari-123

3) Endpoint Query Params
------------------------------------------------------------
- Scaffold path tanpa query (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- products/search

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/products/search.rs`](apps/rust/src/routes/api/products/search.rs:1)

Contoh handler menggunakan Query:

```rust
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub sort: Option<String>,
    pub page: Option<u32>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub message: String,
}

pub async fn search(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let sort = params.sort.unwrap_or_else(|| "relevance".to_string());
    let page = params.page.unwrap_or(1);
    Json(SearchResponse { message: format!("Searching '{}' sort={} page={}", params.q, sort, page) })
}
```

Tes:

curl "http://127.0.0.1:3000/api/products/search?q=sepatu&sort=price_desc&page=2"

4) Kombinasi: Slug + Query Params
------------------------------------------------------------
- Contoh path: products/product_id/reviews
- Scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- products/product_id/reviews

- Build:

  cargo build

Handler contoh:

```rust
use axum::{extract::{Path, Query}, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReviewQuery {
    pub limit: Option<u32>,
}

#[derive(Serialize)]
pub struct ReviewResponse {
    pub message: String,
}

pub async fn reviews(Path(product_id): Path<String>, Query(q): Query<ReviewQuery>) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(10);
    Json(ReviewResponse { message: format!("Reviews for {} limit={}", product_id, limit) })
}
```

5) Petunjuk Operasional & Best Practices
------------------------------------------------------------
- State aplikasi dinamai `AppState`. Untuk mengaksesnya, gunakan `Arc<AppState>` dan parameter `State` di handler.
- Jangan edit `mod.rs` di `src/routes/api` karena dihasilkan oleh [`apps/rust/build.rs`](apps/rust/build.rs:1).
- Setelah membuat route baru: jalankan `cargo build` untuk menghasilkan template, lalu edit file handler.
- Jalankan server: `cargo run` (dijalankan dari `apps/rust`).
- Tes health: `curl http://127.0.0.1:3000/api/health`

6) Contoh lengkap alur
------------------------------------------------------------
1. buat file kosong (dapat dijalankan dari workspace root atau apps/rust/):

   cargo run --bin scaffold -- products/detail/product_id
2. build:

   cargo build
3. edit `src/routes/api/products/detail/product_id.rs`
4. run:

   cargo run

Catatan tambahan:
- Jika handler diinginkan menerima JSON body (POST/PUT), gunakan `axum::Json<T>` dan `serde::Deserialize`.
- Jika butuh validasi, gunakan crate seperti `validator` atau lakukan pemeriksaan manual.
- Jika ingin mengubah template generator, modifikasi [`apps/rust/build.rs`](apps/rust/build.rs:1).

## Troubleshooting: Native FFmpeg dependency (Windows)

Jika `cargo build` atau `cargo run --bin scaffold` gagal dengan pesan tentang `libavutil` / `ffmpeg-sys-next`, lakukan salah satu opsi berikut:

1) (Direkomendasikan) Pasang MSYS2 dan paket FFmpeg
- Unduh dan pasang MSYS2: https://www.msys2.org/  
- Buka *MSYS2 MinGW 64-bit* shell, lalu jalankan:
```powershell
# update system
pacman -Syu

# setelah restart shell, pasang pkg-config dan ffmpeg dev
pacman -S mingw-w64-x86_64-pkg-config mingw-w64-x86_64-ffmpeg
```
- Tambahkan ke PATH dan set PKG_CONFIG_PATH (PowerShell):
```powershell
setx PATH "$env:PATH;C:\msys64\mingw64\bin"
setx PKG_CONFIG_PATH "C:\msys64\mingw64\lib\pkgconfig"
```
- Tutup dan buka kembali terminal VSCode lalu jalankan `cargo build` kembali.

2) Pasang library dev FFmpeg secara manual
- Pasang paket dev yang menyediakan file `libavutil.pc`.
- Set environment variable PKG_CONFIG_PATH ke direktori yang berisi `libavutil.pc`.
- Contoh (PowerShell):
```powershell
setx PKG_CONFIG_PATH "C:\path\to\ffmpeg\lib\pkgconfig"
```
Restart terminal lalu jalankan `cargo build`.

3) Jika Anda tidak memerlukan fitur ffmpeg di lingkungan pengembangan, pertimbangkan menonaktifkan/menjadikan dependensi `ffmpeg-next` opsional di `Cargo.toml` sementara (tidak direkomendasikan untuk produksi).

Dokumentasi scaffold telah diperbarui di [`apps/rust/src/bin/scaffold.md`](apps/rust/src/bin/scaffold.md:1). Setelah memperbaiki dependensi native, ulangi:
```powershell
# Dari workspace root atau apps/rust/
cargo run --bin scaffold -- test/helloworld
cargo build
```
