#!/bin/bash

# Update package list and install dependencies
sudo apt update
sudo apt install -y wget software-properties-common

# Add FFmpeg PPA
sudo apt update

# Install FFmpeg
sudo apt install -y ffmpeg curl git build-essential imagemagick libvips

# Verify FFmpeg installation
ffmpeg -version

# Install other necessary tools for compression
sudo apt install -y zip unzip 

echo "FFmpeg and necessary compression tools have been installed successfully."