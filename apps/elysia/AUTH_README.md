# ElysiaJS Authentication System

Sistem autentikasi lengkap yang kompatibel dengan backend Rust, menggunakan database MySQL yang sama.

## 🔐 Fitur Authentication

✅ **Register** - Pendaftaran user dengan validasi password kuat
✅ **Login** - Login dengan email/username + JWT token
✅ **Logout** - Token blacklisting dengan Redis
✅ **Me** - Get current user profile
✅ **Email Verification** - Verifikasi email otomatis
✅ **Forgot Password** - Request password reset
✅ **Reset Password** - Reset password dengan token
✅ **Refresh Token** - Refresh JWT access token
✅ **Remember Me** - Extended session (30 hari)

## 📋 Database Requirements

Sistem ini menggunakan database MySQL yang sama dengan aplikasi Rust Anda. Pastikan migrations sudah dijalankan:

```sql
-- Tables yang digunakan:
- users
- email_verification_tokens
- password_reset_tokens
- refresh_tokens
- login_history
```

## ⚙️ Configuration

Buat file `.env` dari `.env.example`:

```bash
# Database (sama dengan Rust app)
DATABASE_URL=mysql://username:password@localhost:3306/database_name

# JWT Secret (sama dengan Rust app)
JWT_SECRET=your-super-secret-jwt-key-minimum-32-characters-long

# Redis (untuk token blacklisting)
REDIS_URL=redis://localhost:6379

# Email Configuration
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=your-email@gmail.com
FROM_NAME=Your App Name
```

## 🚀 Getting Started

```bash
# Install dependencies
bun install

# Start development server
bun run dev

# Server akan berjalan di http://localhost:3002
```

## 📡 API Endpoints

### 1. Register
```bash
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "username",
  "password": "StrongPass123!",
  "full_name": "Full Name" # optional
}

Response:
{
  "success": true,
  "message": "User registered successfully...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "username",
    ...
  },
  "verification_token": "token-string"
}
```

### 2. Login
```bash
POST /api/auth/login
Content-Type: application/json

{
  "login": "user@example.com",  # email or username
  "password": "StrongPass123!",
  "remember_me": false # optional
}

Response:
{
  "user": { ... },
  "access_token": "jwt-token",
  "refresh_token": "refresh-token",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

### 3. Get Current User (Me)
```bash
GET /api/auth/me
Authorization: Bearer <access_token>

Response:
{
  "success": true,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "username",
    ...
  }
}
```

### 4. Logout
```bash
POST /api/auth/logout
Authorization: Bearer <access_token>

Response:
{
  "success": true,
  "message": "Logged out successfully"
}
```

### 5. Email Verification
```bash
GET /api/auth/verify?token=<verification_token>

Response:
{
  "success": true,
  "message": "Email verified successfully"
}
```

### 6. Forgot Password
```bash
POST /api/auth/forgot-password
Content-Type: application/json

{
  "email": "user@example.com"
}

Response:
{
  "success": true,
  "message": "If the email exists, a password reset link has been sent"
}
```

### 7. Reset Password
```bash
POST /api/auth/reset-password
Content-Type: application/json

{
  "token": "reset-token",
  "new_password": "NewStrongPass123!"
}

Response:
{
  "success": true,
  "message": "Password has been reset successfully"
}
```

### 8. Refresh Token
```bash
POST /api/auth/refresh-token
Content-Type: application/json

{
  "refresh_token": "refresh-token-string"
}

Response:
{
  "access_token": "new-jwt-token",
  "refresh_token": "new-refresh-token",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

## 🔒 Password Requirements

Password harus memenuhi syarat:
- Minimal 8 karakter
- Minimal 1 huruf besar
- Minimal 1 huruf kecil
- Minimal 1 angka
- Minimal 1 karakter special (!@#$%^&*, dll)

## 🧪 Testing dengan cURL

```bash
# Register
curl -X POST http://localhost:3002/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "Test123!@#",
    "full_name": "Test User"
  }'

# Login
curl -X POST http://localhost:3002/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "login": "test@example.com",
    "password": "Test123!@#"
  }'

# Get Profile (replace TOKEN dengan access_token dari login)
curl http://localhost:3002/api/auth/me \
  -H "Authorization: Bearer TOKEN"

# Logout
curl -X POST http://localhost:3002/api/auth/logout \
  -H "Authorization: Bearer TOKEN"
```

## 🔄 Compatibility dengan Rust App

Sistem ini 100% kompatibel dengan Rust backend:

1. **Database Schema** - Menggunakan tabel yang sama
2. **JWT Format** - Format token sama
3. **Password Hashing** - Menggunakan bcrypt yang sama
4. **API Response** - Format response identik
5. **Business Logic** - Validasi dan flow yang sama

Anda bisa menggunakan ElysiaJS sebagai alternatif atau berjalan bersamaan dengan Rust app!

## 📝 Development Mode

Saat `SMTP_PASSWORD` tidak di-set, email akan di-log ke console:

```
📧 [DEV MODE] Email would be sent:
To: user@example.com
Subject: Verify Your Email Address
Body: ...
```

## 🐳 Docker Support

```bash
# Build
docker build -t elysia-auth .

# Run
docker run -p 3002:3002 \
  -e DATABASE_URL=mysql://... \
  -e JWT_SECRET=... \
  -e REDIS_URL=... \
  elysia-auth
```

## 🛠️ Tech Stack

- **Elysia** - Web framework
- **Bun** - JavaScript runtime
- **MySQL2** - Database driver
- **bcryptjs** - Password hashing
- **@elysiajs/jwt** - JWT authentication
- **ioredis** - Redis client
- **nodemailer** - Email sending
- **uuid** - ID generation

## 📚 Architecture

```
src/
├── routes/
│   └── auth/
│       ├── register.ts      # User registration
│       ├── login.ts          # User login
│       ├── logout.ts         # User logout
│       ├── me.ts             # Get current user
│       ├── verify.ts         # Email verification
│       ├── forgot-password.ts # Request password reset
│       ├── reset-password.ts # Reset password
│       ├── refresh-token.ts  # Refresh JWT token
│       └── index.ts          # Auth routes aggregator
├── models/
│   └── user.ts               # User types & interfaces
├── utils/
│   ├── database.ts           # MySQL connection pool
│   ├── redis.ts              # Redis client
│   ├── jwt.ts                # JWT utilities
│   └── email.ts              # Email service
├── middleware/
│   └── index.ts              # Logger & CORS
├── config.ts                 # Configuration
└── index.ts                  # Main app entry
```

## ⚠️ Important Notes

1. **JWT_SECRET** harus sama dengan Rust app untuk kompatibilitas token
2. **DATABASE_URL** harus menunjuk ke database yang sama
3. **Redis** diperlukan untuk token blacklisting
4. **Migrations** harus sudah dijalankan dari Rust app

## 🚀 Production Deployment

1. Set environment variables yang benar
2. Gunakan SMTP provider production (SendGrid, AWS SES, dll)
3. Enable Redis persistence
4. Setup database backup
5. Use process manager (PM2):

```bash
pm2 start ecosystem.config.cjs
```

## 📞 Support

Jika ada pertanyaan atau issue, silakan buat issue di repository atau hubungi developer.

---

**Made with ❤️ using ElysiaJS & Bun**
