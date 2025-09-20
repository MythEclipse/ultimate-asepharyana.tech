# Panduan Lengkap Scaffold

Panduan lengkap untuk membuat handler API menggunakan scaffolder di proyek ini.
File yang dihasilkan: src/routes/api/\<path\>.rs

Prasyarat:

- Pastikan env: PORT dan JWT_SECRET diset (lihat `.env.example` jika ada).
- Scaffold dapat dijalankan dari workspace root atau dari direktori `apps/rust/`.
- **Baru**: Scaffold sekarang menghasilkan template handler lengkap secara otomatis tanpa perlu `cargo build` untuk populasi template awal.

Ringkasan tipe endpoint:

- Static: path tetap, contoh: /api/products/list
- Index: path root untuk sebuah direktori, contoh: /api/anime (dari `index.rs` di `src/routes/api/anime/index.rs`)
- Dynamic: path dinamis dengan parameter, contoh: /api/products/detail/product_id → /api/products/detail/{id}
- Params (query): query string, contoh: /api/products/search?q=sepatu&sort=price_desc

Pengenalan Dynamic Routes dan Index Files:
Sistem secara otomatis mendeteksi segmen dinamis berdasarkan pola nama file (misalnya, `slug.rs` untuk parameter `{slug}`).
Contoh: `anime/detail/slug` akan menghasilkan `anime/detail/slug.rs`.
File `index.rs` akan secara otomatis di-handle sebagai route root untuk direktorinya.
Contoh: `src/routes/api/anime/index.rs` akan melayani `/api/anime`.

## 0. Flag --protected untuk Authentication

Scaffold sekarang mendukung flag `--protected` untuk menghasilkan handler dengan authentication JWT:

```bash
cargo run --bin scaffold -- anime/profile --protected
```

Handler yang dihasilkan akan menyertakan:

- Middleware authentication menggunakan `AuthMiddleware`
- Parameter `Extension(claims): Extension<Claims>` di function signature
- Security annotation untuk OpenAPI documentation
- Akses ke user claims (user_id, email, name) dari JWT token

### Mekanisme Authentication

Authentication menggunakan JWT (JSON Web Token) dengan:

- **Library**: `jsonwebtoken` crate
- **Algorithm**: HS256
- **Secret**: Diambil dari `CONFIG_MAP["JWT_SECRET"]`
- **Claims Structure**:

  ```rust
  pub struct Claims {
      pub user_id: String,
      pub email: String,
      pub name: String,
      pub exp: usize,  // expiration timestamp
  }
  ```

Handler protected akan memverifikasi token JWT secara otomatis melalui middleware sebelum mengeksekusi logic handler.

## 1. Endpoint Index (root directory route)

------------------------------------------------------------

- Perintah scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- anime/index

- **Baru**: Template lengkap langsung dihasilkan, tidak perlu `cargo build`
- Edit handler di [`src/routes/api/anime/index.rs`](apps/rust/src/routes/api/anime/index.rs:1)

Contoh handler (Rust) untuk `index.rs`:

```rust
use axum::{response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/api/anime"; // Ini akan otomatis disesuaikan
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime index endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_index";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<AnimeResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AnimeResponse {
    pub message: String,
    pub data: String,
}

pub async fn index(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(AnimeResponse { message: "Hello from anime index!".to_string(), data: "Anime data here".to_string() })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(index))
}
```

Tes endpoint:

`curl http://127.0.0.1:3000/api/anime`

## 2. Endpoint Static (tanpa parameter)

------------------------------------------------------------

- Perintah scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- products/list

- **Baru**: Template lengkap langsung dihasilkan
- Edit handler di [`src/routes/api/products/list.rs`](apps/rust/src/routes/api/products/list.rs:1)

Contoh handler (Rust):

------------------------------------------------------------

- Perintah scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- products/list

- **Baru**: Template lengkap langsung dihasilkan
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
    Json(ListResponse { message: "All products".to_string(), data: vec![], total: None })
}
```

Tes endpoint:

`curl http://127.0.0.1:3000/api/products/list`

## 3. Endpoint Dynamic / Path Parameter

------------------------------------------------------------

- Untuk path dinamis, gunakan nama file yang sesuai dengan parameter:

  cargo run --bin scaffold -- anime/detail/slug

- Sistem akan otomatis mendeteksi pola dinamis dan mengkonversi:
  - `slug` (dari `anime/detail/slug.rs`) → parameter path `{slug}` dengan route `/anime/detail/{slug}`
  - `id` (dari `products/detail/id.rs`) → parameter path `{id}` dengan route `/products/detail/{id}`
  - `key` (dari `posts/detail/key.rs`) → parameter path `{key}` dengan route `/posts/detail/{key}`

- **Baru**: Template lengkap langsung dihasilkan
- Edit handler di [`src/routes/api/anime/detail/slug.rs`](apps/rust/src/routes/api/anime/detail/slug.rs:1)

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
pub const ENDPOINT_PATH: &str = "/anime/detail/{slug}";
pub const ENDPOINT_DESCRIPTION: &str = "Handles GET requests for the anime/detail/{slug} endpoint.";
pub const ENDPOINT_TAG: &str = "anime";
pub const OPERATION_ID: &str = "anime_detail_slug";
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
        message: format!("Hello from slug with parameters: slug: {slug}"),
        data: serde_json::json!({"slug": slug}),
    })
}

/// Handles GET requests for the anime/detail/{slug} endpoint.
pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(slug))
}
```

Contoh handler dengan UUID:

```rust
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;

pub async fn slug(Path(slug): Path<String>) -> impl IntoResponse {
    let parsed = Uuid::parse_str(&slug).ok();
    Json(ProductResponse { message: format!("Parsed UUID: {:?}", parsed) })
}
```

Tes:

`curl http://127.0.0.1:3000/api/anime/detail/log-horizon`

## 4. Endpoint Query Params

------------------------------------------------------------

- Scaffold path tanpa query (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- products/search

- **Baru**: Template lengkap langsung dihasilkan
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
    pub page: Option<u34>,
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

`curl "http://127.0.0.1:3000/api/products/search?q=sepatu&sort=price_desc&page=2"`

## 5. Kombinasi: Slug + Query Params

------------------------------------------------------------

- Contoh path: anime/slug/reviews
- Scaffold (dapat dijalankan dari workspace root atau apps/rust/):

  cargo run --bin scaffold -- anime/slug/reviews

- **Baru**: Template lengkap langsung dihasilkan

Handler contoh:

```rust
use axum::{extract::{Path, Query}, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReviewQuery {
    pub limit: Option<u34>,
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

## 6. Endpoint dengan Authentication (--protected)

------------------------------------------------------------

- Untuk membuat endpoint yang memerlukan authentication:

  cargo run --bin scaffold -- user/profile --protected

- Handler akan menyertakan middleware authentication dan akses ke user claims

Contoh handler protected:

```rust
use axum::{response::IntoResponse, routing::get, Json, Router, Extension};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::utils::auth::Claims;

#[utoipa::path(
    get,
    path = "/api/user/profile",
    tag = "user",
    operation_id = "user_profile",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserProfileResponse),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("ApiKeyAuth" = [])
    )
)]
pub async fn profile(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    Json(UserProfileResponse {
        message: "User profile retrieved".to_string(),
        user_id: claims.user_id,
        email: claims.email,
        name: claims.name,
    })
}

pub fn register_routes(router: Router) -> Router {
    let router = router.layer(AuthMiddleware::layer());
    router.route("/api/user/profile", get(profile))
}
```

Untuk test endpoint protected, sertakan JWT token di header Authorization:

```bash
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://127.0.0.1:3000/api/user/profile
```

## 7. Petunjuk Operasional & Best Practices

------------------------------------------------------------

- State aplikasi dinamai `AppState`. Untuk mengaksesnya, gunakan `Arc<AppState>` dan parameter `State` di handler.
- Jangan edit `mod.rs` di `src/routes/api` karena dihasilkan oleh [`apps/rust/build.rs`](apps/rust/build.rs:1).
- **Baru**: Template handler lengkap dihasilkan langsung oleh scaffold, tidak perlu `cargo build` untuk template awal.
- Jalankan server: `cargo run` (dijalankan dari `apps/rust`).
- Tes health: `curl http://127.0.0.1:3000/api/health`
- Untuk endpoint protected, pastikan JWT_SECRET sudah diset dengan benar di environment.

## 8. Contoh lengkap alur

------------------------------------------------------------

1. buat file kosong (dapat dijalankan dari workspace root atau apps/rust/):

    cargo run --bin scaffold -- anime/detail/slug

2. **Baru**: Template lengkap sudah dihasilkan, langsung edit file
3. edit `src/routes/api/anime/detail/slug.rs`
4. run:

    cargo run

Catatan tambahan:

- Jika handler diinginkan menerima JSON body (POST/PUT), gunakan `axum::Json<T>` dan `serde::Deserialize`.
- Jika butuh validasi, gunakan crate seperti `validator` atau lakukan pemeriksaan manual.
- Jika ingin mengubah template generator, modifikasi [`apps/rust/build.rs`](apps/rust/build.rs:1).
- Untuk endpoint yang memerlukan authentication, gunakan flag `--protected` saat scaffold.

Dokumentasi scaffold telah diperbarui di [`apps/rust/src/bin/scaffold.md`](apps/rust/src/bin/scaffold.md:1). Scaffold sekarang menghasilkan template lengkap dengan dukungan authentication opsional.

```powershell
# Dari workspace root atau apps/rust/
cargo run --bin scaffold -- test/helloworld
# Template lengkap langsung tersedia untuk edit
