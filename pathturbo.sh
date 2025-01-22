#!/bin/bash

# Dapatkan path dari perintah 'which'
TURBO_PATH=$(which turbo)

# Ambil direktori dari path
DIR_PATH=$(dirname "$TURBO_PATH")

# Cek apakah path valid
if [ -d "$DIR_PATH" ]; then
    # Tambahkan path ke ~/.bashrc atau /etc/profile
    # echo "export PATH=\"$DIR_PATH:\$PATH\"" >> ~/.bashrc
    # echo "Path untuk turbo ditambahkan ke ~/.bashrc"

    # Untuk semua pengguna, gunakan /etc/profile
    sudo sh -c "echo 'export PATH=\"$DIR_PATH:\$PATH\"' >> /etc/profile"
    echo "Path untuk turbo ditambahkan ke /etc/profile"
else
    echo "Turbo tidak ditemukan di sistem."
fi
