#!/bin/bash

# Menghapus semua direktori node_modules di direktori saat ini dan subdirektori
echo "Menghapus semua direktori node_modules di $(pwd) dan subdirektorinya..."

find . -name "node_modules" -type d -prune -exec rm -rf {} +

echo "Penghapusan node_modules selesai."
