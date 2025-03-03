#!/bin/bash

# Check if the user provided a key name
if [ -z "$1" ]; then
    echo "Usage: $0 <key_name>"
    exit 1
fi

KEY_NAME=$1

# Generate SSH key
ssh-keygen -t rsa -b 4096 -C "your_email@example.com" -f ~/.ssh/$KEY_NAME -N ""

# Display the public key
echo "Your public key is:"
cat ~/.ssh/$KEY_NAME.pub

# Instructions to add the key to the VPS
echo "To add the key to your VPS, use the following command:"
echo "ssh-copy-id -i ~/.ssh/$KEY_NAME.pub root@217.15.165.147"