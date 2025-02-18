# Hapus database SQLite jika ada
Remove-Item -Path "./apps/Express/database.sqlite" -ErrorAction SilentlyContinue
Remove-Item -Path "./apps/express/database.sqlite" -ErrorAction SilentlyContinue
Write-Host "Database reset"