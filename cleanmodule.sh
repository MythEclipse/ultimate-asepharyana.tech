#!/bin/bash

# Menghapus semua direktori node_modules di direktori saat ini dan subdirektori
echo "Menghapus semua direktori node_modules dan file lock di $(pwd) dan subdirektorinya..."

find . -name "node_modules" -type d -prune -exec rm -rf {} +
find . -name "package-lock.json" -type f -exec rm -f {} +
find . -name "yarn.lock" -type f -exec rm -f {} +
find . -name "pnpm-lock.yaml" -type f -exec rm -f {} +

echo "Penghapusan node_modules dan file lock selesai."
