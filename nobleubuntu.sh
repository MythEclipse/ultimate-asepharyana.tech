#!/bin/bash

# Buat cadangan file sources.list
sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak

# Mengubah repositori dari focal ke noble
sudo sed -i 's/focal/noble/g' /etc/apt/sources.list

# Ganti dengan mirror baru menggunakan format deb822
sudo bash -c 'cat <<EOF > /etc/apt/sources.list.d/ubuntu.sources
Types: deb
URIs: http://mirror.poliwangi.ac.id/ubuntu/
Suites: noble
Components: main restricted universe multiverse
EOF

cat <<EOF > /etc/apt/sources.list.d/ubuntu-updates.sources
Types: deb
URIs: http://mirror.poliwangi.ac.id/ubuntu/
Suites: noble-updates
Components: main restricted universe multiverse
EOF

cat <<EOF > /etc/apt/sources.list.d/ubuntu-security.sources
Types: deb
URIs: http://mirror.poliwangi.ac.id/ubuntu/
Suites: noble-security
Components: main restricted universe multiverse
EOF

cat <<EOF > /etc/apt/sources.list.d/ubuntu-backports.sources
Types: deb
URIs: http://mirror.poliwangi.ac.id/ubuntu/
Suites: noble-backports
Components: main restricted universe multiverse
EOF

cat <<EOF > /etc/apt/sources.list.d/ubuntu-proposed.sources
Types: deb
URIs: http://mirror.poliwangi.ac.id/ubuntu/
Suites: noble-proposed
Components: main restricted universe multiverse
EOF'

# Memperbarui daftar paket
sudo apt update

# Melakukan upgrade
sudo apt upgrade -y

# Melakukan dist-upgrade
sudo apt dist-upgrade -y

# Menghapus paket yang tidak diperlukan
sudo apt autoremove -y

echo "Pembaruan sistem dari Focal ke Noble telah selesai dan repositori telah diganti."