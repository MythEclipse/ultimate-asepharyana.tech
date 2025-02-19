#!/bin/bash
echo "Memulai pembersihan Docker..."
docker system prune -a -f
docker volume prune -f

if command -v pnpm &> /dev/null; then
  echo "Membersihkan cache PNPM..."
  pnpm store prune
else
  echo "PNPM tidak terpasang, lewati..."
fi

if command -v yarn &> /dev/null; then
  echo "Membersihkan cache Yarn..."
  yarn cache clean
else
  echo "Yarn tidak terpasang, lewati..."
fi

if command -v npm &> /dev/null; then
  echo "Membersihkan cache NPM..."
  npm cache clean --force
else
  echo "NPM tidak terpasang, lewati..."
fi

echo "Pembersihan selesai!"
