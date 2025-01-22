# Ultimate-Asepharyana.cloud

Selamat datang di proyek Ultimate-Asepharyana.cloud! Repositori ini adalah portofolio pribadi yang menampilkan pengaturan monorepo menggunakan Turborepo. Jika Anda mengalami masalah atau memiliki saran untuk perbaikan, silakan kirim pull request.

## Menggunakan proyek ini

Jalankan perintah berikut:

```bash
git clone https://github.com/MythEclipse/ultimate-asepharyana.cloud.git
```

## Apa yang ada di dalamnya?

Turborepo ini mencakup paket/aplikasi berikut:

### Aplikasi dan Paket

- `@asepharyana/web`: aplikasi [Next.js](https://github.com/MythEclipse/asepharyana.cloud)
- `@asepharyana/api`: aplikasi [Express](https://github.com/MythEclipse/API)
- `@asepharyana/eslint-config`: konfigurasi `eslint` (termasuk `eslint-config-next` dan `eslint-config-prettier`)
- `@asepharyana/database`: pembungkus ORM [Prisma](https://prisma.io/) untuk mengelola & mengakses database Anda
- `@asepharyana/typescript-config`: `tsconfig.json` yang digunakan di seluruh monorepo

Setiap paket/aplikasi 100% menggunakan [TypeScript](https://www.typescriptlang.org/).

### Utilitas

Turborepo ini memiliki beberapa alat tambahan yang sudah diatur untuk Anda:

- [TypeScript](https://www.typescriptlang.org/) untuk pemeriksaan tipe statis
- [ESLint](https://eslint.org/) untuk linting kode
- [Prettier](https://prettier.io) untuk pemformatan kode
- [Prisma](https://prisma.io/) untuk ORM database
- [Docker Compose](https://docs.docker.com/compose/) untuk database lokal

### Database

Kami menggunakan [Prisma](https://prisma.io/) untuk mengelola & mengakses database kami. Oleh karena itu, Anda memerlukan database untuk proyek ini, baik secara lokal atau di-host di cloud.

Untuk mempermudah proses ini, kami menawarkan file [`docker-compose.yml`](https://docs.docker.com/compose/) untuk menerapkan server MySQL secara lokal dengan database baru bernama `turborepo` (Untuk mengubah ini, perbarui variabel lingkungan `MYSQL_DATABASE` dalam file `docker-compose.yml`):

```bash
cd my-turborepo
docker-compose up -d
```

Setelah diterapkan, Anda perlu menyalin file `.env.example` ke `.env` agar Prisma memiliki variabel lingkungan `DATABASE_URL` untuk diakses.

```bash
cp .env.example .env
```

Jika Anda menambahkan nama database khusus atau menggunakan database berbasis cloud, Anda perlu memperbarui `DATABASE_URL` dalam `.env` Anda sesuai kebutuhan.

Setelah diterapkan & berjalan, Anda perlu membuat & menerapkan migrasi ke database Anda untuk menambahkan tabel yang diperlukan. Ini dapat dilakukan menggunakan [Prisma Migrate](https://www.prisma.io/migrate):

```bash
npx prisma migrate dev
```

Jika Anda perlu mendorong migrasi yang ada ke database, Anda dapat menggunakan perintah Prisma db push atau Prisma migrate deploy:

```bash
pnpm run db:push

# ATAU

pnpm run db:migrate:deploy
```

Ada sedikit perbedaan antara kedua perintah & [Prisma menawarkan penjelasan tentang perintah mana yang terbaik untuk digunakan](https://www.prisma.io/docs/concepts/components/prisma-migrate/db-push#choosing-db-push-or-prisma-migrate).

Langkah tambahan opsional adalah menambahkan beberapa data awal atau data palsu ke database Anda menggunakan [fungsi seeding Prisma](https://www.prisma.io/docs/guides/database/seed-database).

Untuk melakukan ini, periksa skrip seed yang terletak di `packages/database/src/seed.ts` & tambahkan atau perbarui pengguna yang ingin Anda tambahkan ke database.

Setelah diedit, jalankan perintah berikut untuk memberi tahu Prisma untuk menjalankan skrip seed yang ditentukan dalam konfigurasi Prisma:

```bash
pnpm run db:seed
```

Untuk informasi lebih lanjut tentang migrasi, seeding & lainnya, kami merekomendasikan membaca [Dokumentasi Prisma](https://www.prisma.io/docs/).

### Build

Untuk membangun semua aplikasi dan paket, jalankan perintah berikut:

```bash
pnpm run build
```

### Develop

Untuk mengembangkan semua aplikasi dan paket, jalankan perintah berikut:

```bash
pnpm run dev
```

## Tautan Berguna

Pelajari lebih lanjut tentang kekuatan Turborepo:

- [Tugas](https://turbo.build/repo/docs/core-concepts/monorepos/running-tasks)
- [Caching](https://turbo.build/repo/docs/core-concepts/caching)
- [Remote Caching](https://turbo.build/repo/docs/core-concepts/remote-caching)
- [Filtering](https://turbo.build/repo/docs/core-concepts/monorepos/filtering)
- [Opsi Konfigurasi](https://turbo.build/repo/docs/reference/configuration)
- [Penggunaan CLI](https://turbo.build/repo/docs/reference/command-line-reference)
