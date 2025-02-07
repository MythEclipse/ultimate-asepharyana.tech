#!/bin/bash

# Minta input username dan email dengan default
read -p "Masukkan username [default: asepharyana]: " GIT_USERNAME
GIT_USERNAME=${GIT_USERNAME:-asepharyana}

read -p "Masukkan email [default: asepharyana@example.com]: " GIT_EMAIL
GIT_EMAIL=${GIT_EMAIL:-asepharyana@example.com}

# Konfigurasi Git secara global
git config --global user.name "$GIT_USERNAME"
git config --global user.email "$GIT_EMAIL"

# Verifikasi konfigurasi
echo "Git configuration updated:"
git config --global --list
