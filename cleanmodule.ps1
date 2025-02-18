# Menghapus semua direktori node_modules di direktori saat ini dan subdirektori
Write-Host "Menghapus semua direktori node_modules dan file lock di $(Get-Location) dan subdirektorinya..."

# Menghapus direktori node_modules
Get-ChildItem -Recurse -Directory -Filter "node_modules" | Remove-Item -Recurse -Force

# Menghapus file package-lock.json, yarn.lock, dan pnpm-lock.yaml
Get-ChildItem -Recurse -File -Filter "package-lock.json" | Remove-Item -Force
Get-ChildItem -Recurse -File -Filter "yarn.lock" | Remove-Item -Force
Get-ChildItem -Recurse -File -Filter "pnpm-lock.yaml" | Remove-Item -Force

Write-Host "Penghapusan node_modules dan file lock selesai."
