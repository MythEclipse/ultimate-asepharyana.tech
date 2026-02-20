#!/bin/bash

# Configuration
SERVICE_NAME="rustexpress"
SERVICE_FILE="apps/rust/rustexpress.service"
TARGET_DIR="$HOME/ultimate-asepharyana.cloud"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"


# Ensure XDG_RUNTIME_DIR is set
export XDG_RUNTIME_DIR="/run/user/$(id -u)"

echo "Setup Systemd (User Mode) for $SERVICE_NAME..."

# Check if service file exists
if [ ! -f "$TARGET_DIR/$SERVICE_FILE" ]; then
    echo "Error: Service file not found at $TARGET_DIR/$SERVICE_FILE"
    exit 1
fi

# Create user systemd directory
mkdir -p "$SYSTEMD_USER_DIR"

# Link or copy service file
echo "Installing service file to $SYSTEMD_USER_DIR/..."
cp "$TARGET_DIR/$SERVICE_FILE" "$SYSTEMD_USER_DIR/$SERVICE_NAME.service"

# Reload daemon
echo "Reloading systemd --user..."
systemctl --user daemon-reload

# Enable service
echo "Enabling service..."
systemctl --user enable $SERVICE_NAME

# Enable lingering (allows user service to run without active session)
echo "Enabling lingering for $USER (requires sudo purely for loginctl if not already set, or admin intervention)"
if loginctl show-user $USER | grep -q "Linger=no"; then
    echo "Attempting to enable linger..."
    # Often loginctl enable-linger can be run by the user for themselves on modern systems, 
    # OR it needs root. If this fails, user sees message.
    loginctl enable-linger $USER || echo "⚠️  Could not enable linger. You might need: sudo loginctl enable-linger $USER"
else
    echo "Linger already enabled."
fi

# Start/Restart service
echo "Starting service..."
systemctl --user restart $SERVICE_NAME

# Status
systemctl --user status $SERVICE_NAME --no-pager

echo "✅ Systemd (User Mode) setup complete!"
echo "You can now use: systemctl --user restart $SERVICE_NAME"
