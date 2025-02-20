# Jalankan script reset SQLite
powershell -ExecutionPolicy Bypass -File sqlitereset.ps1

# Ambil perubahan terbaru dari repository Git
git fetch origin
git pull origin main

# Instal dependensi menggunakan pnpm
pnpm install

# Jalankan migrasi database jika diperlukan
pnpm run generate
# pnpm run db:push
# pnpm run db:migrate:deploy

# Build proyek Next.js untuk produksi
pnpm run buildnc

# Cek apakah build berhasil
if ($?) {
    # Restart proses PM2 untuk "express" dan "nextjs" dengan environment yang diperbarui
    pm2 restart express --update-env
    pm2 restart nextjs --update-env

    # Jalankan script commit
    powershell -ExecutionPolicy Bypass -File commit.ps1
} else {
    Write-Host "Build failed. Skipping commit."
}
