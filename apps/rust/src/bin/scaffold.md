
-----

## Langkah 1: Jalankan Perintah `scaffold`

Buka terminal Anda di root proyek dan jalankan perintah berikut. `admins/detail/[admin_id]` adalah path rute yang kita inginkan.

```powershell
cargo run --bin scaffold -- admins/detail/[admin_id]
```

Anda akan melihat output yang mengonfirmasi bahwa file kosong telah dibuat:

```
✅ Empty file created successfully at: "src/routes/api/admins/detail/[admin_id].rs"
   Run `cargo build` to auto-populate the file with the handler template.
```

-----

## Langkah 2: Jalankan `build` untuk Mengisi Konten

Sekarang, jalankan proses build. `build.rs` akan mendeteksi file baru tersebut dan mengisinya dengan template.

```powershell
cargo build
```

Setelah build selesai, file `src/routes/api/admins/detail/[admin_id].rs` yang tadinya kosong sekarang akan berisi:

```rust
//! Handler for the admin_id endpoint.
#![allow(dead_code)]

use axum::{response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const ENDPOINT_METHOD: &str = "get";
pub const ENDPOINT_PATH: &str = "/admins/detail/{admin_id}";
pub const ENDPOINT_DESCRIPTION: &str = "Description for the admin_id endpoint";
pub const ENDPOINT_TAG: &str = "detail";
pub const SUCCESS_RESPONSE_BODY: &str = "Json<AdminIdResponse>";

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AdminIdResponse {
    pub message: String,
}

pub async fn admin_id() -> impl IntoResponse {
    Json(AdminIdResponse {
        message: "Hello from admin_id!".to_string(),
    })
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route(ENDPOINT_PATH, get(admin_id))
}
```

-----

## Langkah 3: Modifikasi Handler

File di atas adalah template. Sekarang kita bisa memodifikasinya untuk mengambil `admin_id` dari path.

Buka file `[admin_id].rs` dan ubah fungsi `admin_id()`:

```rust
// Tambahkan `extract::Path` ke `use axum`
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
// ... (sisa kode tidak berubah) ...

// Tambahkan Path(admin_id) sebagai parameter
pub async fn admin_id(Path(admin_id): Path<String>) -> impl IntoResponse {
    Json(AdminIdResponse {
        // Ubah pesan response untuk menggunakan parameter
        message: format!("Fetching details for admin: {}", admin_id),
    })
}
// ... (sisa kode tidak berubah) ...
```

-----

## Langkah 4: Jalankan dan Tes

Terakhir, jalankan server Anda dan tes endpoint baru tersebut.

1.  **Jalankan Server:**

    ```powershell
    cargo run
    ```

2.  **Tes dengan `curl`** (atau buka di browser):

    ```powershell
    curl http://127.0.0.1:3000/api/admins/detail/user-12345
    ```

**Hasil yang Diharapkan:**

```json
{
  "message": "Fetching details for admin: user-12345"
}
```
