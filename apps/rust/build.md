## Cara Kerjanya 🚀
Alur kerja Anda sekarang menjadi sangat sederhana dan cepat:

Buat File Baru: Ingin endpoint baru? Cukup buat file .rs di lokasi yang sesuai.

Untuk /api/products/list: Buat file src/routes/api/products/list.rs.

Untuk /api/products: Buat file src/routes/api/products/index.rs.

Untuk /api/products/:id: Buat file src/routes/api/products/[id].rs.

Biarkan File Kosong: Simpan file yang baru Anda buat dalam keadaan kosong.

Jalankan cargo build: Cukup kompilasi proyek Anda seperti biasa.

Skrip build.rs akan mendeteksi file baru yang kosong.

Ia akan secara otomatis mengisinya dengan template handler lengkap, termasuk metadata, struct response, fungsi async, dan fungsi register_routes.

Semua file mod.rs dari direktori tersebut hingga ke src/routes/api/mod.rs akan dibuat atau diperbarui untuk menyertakan rute baru Anda.

Dokumentasi ApiDoc Utoipa juga akan diperbarui.

Isi Logika & Metadata: Sekarang, buka file yang sudah terisi otomatis (misalnya list.rs). Anda hanya perlu:

Mengubah nilai const di bagian atas (seperti ENDPOINT_DESCRIPTION).

Menyesuaikan struct Response.

Menulis logika bisnis Anda di dalam fungsi async.
