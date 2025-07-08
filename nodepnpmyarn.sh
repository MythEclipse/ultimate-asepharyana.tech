#!/bin/bash

# Set NVM_DIR ke lokasi yang benar
export NVM_DIR="/usr/local/share/nvm"
echo "NVM_DIR set to $NVM_DIR"

# Load NVM
if [ -s "$NVM_DIR/nvm.sh" ]; then
  echo "Loading NVM..."
  . "$NVM_DIR/nvm.sh"
  
  # Instal Node.js versi terbaru
  echo "Installing Node.js..."
  nvm install node

  # Install Bun globally
  echo "Installing Bun globally..."
  npm install -g bun

  # Install dependencies using Bun
  echo "Installing dependencies with Bun..."
  bun install
else
  echo "nvm.sh not found in $NVM_DIR"
fi