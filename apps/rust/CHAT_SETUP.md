# Setup Chat Backend - Quick Start

## ✅ Sudah Dikonfigurasi

### Database
- **MySQL localhost** di `mysql://root:@localhost:3306/sosmed`
- Auto-migration saat startup aplikasi
- Auto-seed dengan 3 chat rooms default:
  - General
  - Tech Talk  
  - Random

### Fitur Auto-Setup
Ketika aplikasi dijalankan, otomatis akan:
1. ✅ Connect ke database MySQL
2. ✅ Run migrations (create tables)
3. ✅ Seed data default (jika belum ada)
4. ✅ Start server di port 4091

## 🚀 Cara Deploy

### 1. Reset Database (Optional - untuk fresh install)
```powershell
.\reset-db.ps1
```

### 2. Jalankan Aplikasi
```powershell
cargo run --bin rust
```

Server akan start di `http://localhost:4091`

### 3. Endpoints

#### REST API
- `POST /api/chat/rooms` - Create room baru
- `GET /api/chat/rooms` - List semua rooms
- `POST /api/chat/rooms/:id/messages` - Kirim message
- `GET /api/chat/rooms/:id/messages` - Get messages (dengan pagination)

#### WebSocket
- `ws://localhost:4091/ws/chat` - Real-time chat connection

#### Documentation
- `http://localhost:4091/docs` - Swagger UI

## 📁 File Penting

- `migrations/` - SQL migration files (auto-run)
  - `20251031162909_create_users_table.sql`
  - `20251031180000_create_chat_tables.sql`
  - `20251031190000_seed_chat_data.sql`
- `src/seed.rs` - Auto-seed logic (check if empty, then seed)
- `reset-db.ps1` - Script untuk reset database
- `.env` - Database connection config

## 🔧 Konfigurasi

File `.env`:
```env
DATABASE_URL="mysql://root:@localhost:3306/sosmed"
JWT_SECRET="273742"
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
```

## 📝 Notes

- Database `sosmed` akan otomatis dibuat jika belum ada (via migrations)
- Seed data hanya dijalankan jika table `chat_rooms` kosong
- Tidak perlu manual migration atau seed - semua otomatis!
- Untuk production, ubah DATABASE_URL ke server MySQL production

## 🧪 Testing

```bash
# Check database contents
mysql -u root sosmed -e "SELECT * FROM chat_rooms;"

# Test API
curl http://localhost:4091/api/chat/rooms

# View Swagger docs
open http://localhost:4091/docs
```
