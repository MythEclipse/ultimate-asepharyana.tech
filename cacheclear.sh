#!/bin/bash
echo "Memulai pembersihan Docker..."
docker system prune -a -f
docker volume prune -f

if command -v bun &> /dev/null; then
  echo "Membersihkan cache Bun..."
  bun install --offline
else
  echo "Bun tidak terpasang, lewati..."
fi

echo "Pembersihan selesai!"