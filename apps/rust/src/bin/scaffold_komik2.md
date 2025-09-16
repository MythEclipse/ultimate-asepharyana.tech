# Panduan Lengkap Scaffold Komik2 API

Panduan lengkap untuk membuat handler API menggunakan scaffolder di proyek ini untuk API komik2.
File yang dihasilkan: src/routes/api/komik2/\<path\>.rs

Prasyarat:

- Pastikan env: PORT dan JWT_SECRET diset (lihat `.env.example` jika ada).
- Scaffold dapat dijalankan dari workspace root atau dari direktori `apps/rust/`.
- Setelah membuat file kosong, jalankan `cargo build` supaya [`apps/rust/build.rs`](apps/rust/build.rs:1) menghasilkan template handler.

Ringkasan tipe endpoint:

- Static: path tetap, contoh: /api/komik2/manga/list
- Index: path root untuk sebuah direktori, contoh: /api/komik2/manga (dari `index.rs` di `src/routes/api/komik2/manga/index.rs`)
- Dynamic: path dinamis dengan parameter, contoh: /api/komik2/detail/comic_slug → /api/komik2/detail/{slug}
- Params (query): query string, contoh: /api/komik2/search?s=naruto

Pengenalan Dynamic Routes dan Index Files:
Sistem secara otomatis mendeteksi segmen dinamis berdasarkan pola nama file (misalnya, `slug.rs` untuk parameter `{slug}`).
Contoh: `komik2/detail/slug` akan menghasilkan `komik2/detail/slug.rs`.
File `index.rs` akan secara otomatis di-handle sebagai route root untuk direktorinya.
Contoh: `src/routes/api/komik2/manga/index.rs` akan melayani `/api/komik2/manga`.

Base URL: `https://komiku.org/`

## 1. Endpoint Index (root directory route)

------------------------------------------------------------

- Perintah scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- komik2/manga/index

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/komik2/manga/index.rs`](apps/rust/src/routes/api/komik2/manga/index.rs:1)

Contoh handler (Rust) untuk `index.rs`:

```rust
use axum::{response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/komik2/manga"; // Ini akan otomatis disesuaikan
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the komik2 manga index endpoint.";
pub const ENDPOINT_TAG: &str = "komik2";
pub const OPERATION_ID: &str = "komik2_manga_index";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<MangaResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MangaResponse {
    pub message: String,
    pub data: String,
}

pub async fn index(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(MangaResponse { message: "Hello from komik2 manga index!".to_string(), data: "Komik2 Manga list here".to_string() })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(index))
}
```

Tes endpoint:

`curl http://127.0.0.1:3000/api/komik2/manga`

## 2. Endpoint Static (tanpa parameter)

------------------------------------------------------------

- Perintah scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- komik2/manga/list

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/komik2/manga/list.rs`](apps/rust/src/routes/api/komik2/manga/list.rs:1)

Contoh handler (Rust):

```rust
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse {
    pub message: String,
    pub data: Vec<String>,
    pub total: Option<u32>,
}

pub async fn list() -> impl IntoResponse {
    Json(ListResponse { message: "All Komik2 manga list".to_string(), data: vec![], total: None })
}
```

Tes endpoint:

`curl http://127.0.0.1:3000/api/komik2/manga/list`

## 3. Endpoint Dynamic / Path Parameter

------------------------------------------------------------

- Untuk path dinamis, gunakan nama file yang sesuai dengan parameter:

  cargo run --bin scaffold -- komik2/detail/slug

- Sistem akan otomatis mendeteksi pola dinamis dan mengkonversi:
  - `slug` (dari `komik2/detail/slug.rs`) → parameter path `{slug}` dengan route `/komik2/detail/{slug}`

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/komik2/detail/slug.rs`](apps/rust/src/routes/api/komik2/detail/slug.rs:1)

Contoh handler untuk endpoint dinamis (sesuai template yang dihasilkan):

```rust
//! DYNAMIC_ROUTE
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use serde_json;
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/komik2/detail/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the komik2/detail/{slug} endpoint.";
pub const ENDPOINT_TAG: &str = "komik2";
pub const OPERATION_ID: &str = "komik2_detail_slug";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<DetailResponse>";

/// Response structure for the Detail endpoint.
/// Replace `serde_json::Value` with your actual data type and implement `utoipa::ToSchema` for complex types.
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DetailResponse {
    /// Success message
    pub message: String,
    /// Detailed data - replace with actual T where T implements ToSchema
    pub data: serde_json::Value,
}

pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
    Json(DetailResponse {
        message: format!("Hello from komik2 detail with slug: {slug}"),
        data: serde_json::json!({"slug": slug}),
    })
}

/// Handles GET requests for the komik2/detail/{slug} endpoint.
pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}
```

Tes:

`curl http://127.0.0.1:3000/api/komik2/detail/log-horizon`

## 4. Endpoint Query Params

------------------------------------------------------------

- Scaffold path tanpa query (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- komik2/search

- Jalankan build:

  cargo build

- Edit handler di [`src/routes/api/komik2/search.rs`](apps/rust/src/routes/api/komik2/search.rs:1)

Contoh handler menggunakan Query:

```rust
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct SearchParams {
    pub s: String, // search query
    pub page: Option<u32>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub message: String,
    pub results: Vec<String>,
    pub page: u32,
}

pub async fn search(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    Json(SearchResponse { message: format!("Searching komik2 for '{}' on page {}", params.s, page), results: vec![], page: page })
}
```

Tes:

`curl "http://127.0.0.1:3000/api/komik2/search?s=naruto&page=1"`

## 5. Kombinasi: Slug + Query Params

------------------------------------------------------------

- Contoh path: komik2/chapter/slug
- Scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- komik2/chapter/slug

- Build:

  cargo build

Handler contoh:

```rust
use axum::{extract::{Path, Query}, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChapterQuery {
    pub page: Option<u32>,
}

#[derive(Serialize)]
pub struct ChapterResponse {
    pub message: String,
    pub chapter_data: serde_json::Value,
}

pub async fn slug(Path(chapter_slug): Path<String>, Query(q): Query<ChapterQuery>) -> impl IntoResponse {
    let page = q.page.unwrap_or(1);
    Json(ChapterResponse { message: format!("Komik2 Chapter {} on page {}", chapter_slug, page), chapter_data: serde_json::json!({"chapter_slug": chapter_slug, "page": page}) })
}
```

## 6. Petunjuk Operasional & Best Practices

------------------------------------------------------------

- State aplikasi dinamai `AppState`. Untuk mengaksesnya, gunakan `Arc<AppState>` dan parameter `State` di handler.
- Jangan edit `mod.rs` di `src/routes/api` karena dihasilkan oleh [`apps/rust/build.rs`](apps/rust/build.rs:1).
- Setelah membuat route baru: jalankan `cargo build` untuk menghasilkan template, lalu edit file handler.
- Jalankan server: `cargo run` (dijalankan dari `apps/rust`).
- Tes health: `curl http://127.0.0.1:3000/api/health`

## 7. Contoh lengkap alur

------------------------------------------------------------

1. buat file kosong (dapat dijalankan dari workspace root atau apps/rust/):

   cargo run --bin scaffold -- komik2/detail/slug
2. build:

   cargo build
3. edit `src/routes/api/komik2/detail/slug.rs`
4. run:

   cargo run

Catatan tambahan:

- Jika handler diinginkan menerima JSON body (POST/PUT), gunakan `axum::Json<T>` dan `serde::Deserialize`.
- Jika butuh validasi, gunakan crate seperti `validator` atau lakukan pemeriksaan manual.
- Jika ingin mengubah template generator, modifikasi [`apps/rust/build.rs`](apps/rust/build.rs:1).

Dokumentasi scaffold telah diperbarui di [`apps/rust/src/bin/scaffold_komik2.md`](apps/rust/src/bin/scaffold_komik2.md:1). Setelah memperbaiki dependensi native, ulangi:

```powershell
# Dari workspace root atau apps/rust/
cargo run --bin scaffold -- komik2/test/helloworld
cargo build
