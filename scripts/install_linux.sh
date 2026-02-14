#!/bin/bash
# ==================================================================================
# AUTOMATIC INSTALLER FOR LINUX (Ubuntu/Debian/Raspberry Pi)
# ==================================================================================
# This script sets up the POS Hardware Service as a systemd service.
# Usage: sudo ./install_linux.sh
# ==================================================================================

set -e

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./install_linux.sh)"
  exit 1
fi

echo "Installing POS Hardware Service..."

# 1. Determine Paths
SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
PROJECT_ROOT=$(dirname "$SCRIPT_DIR")
EXE_PATH="$PROJECT_ROOT/target/release/pos_hardware_service"
CONFIG_PATH="$PROJECT_ROOT/config.toml"

INSTALL_DIR="/opt/pos_hardware_service"
SERVICE_NAME="pos_hardware.service"

if [ ! -f "$EXE_PATH" ]; then
    echo "Error: Binary not found at $EXE_PATH"
    echo "Please run 'cargo build --release' first."
    exit 1
fi

# 2. Create Installation Directory
echo "Creating $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp "$EXE_PATH" "$INSTALL_DIR/pos_hardware_service"
cp "$CONFIG_PATH" "$INSTALL_DIR/config.toml"
# Create logs directory
mkdir -p "$INSTALL_DIR/logs"

# 3. Create Systemd Service File
echo "Creating systemd unit..."
cat > /etc/systemd/system/$SERVICE_NAME <<EOF
[Unit]
Description=POS Hardware Service
After=network.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/pos_hardware_service
Restart=always
RestartSec=5
# Ensure logs are flushed
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

# 4. Enable and Start
echo "Enabling service..."
systemctl daemon-reload
systemctl enable $SERVICE_NAME
systemctl restart $SERVICE_NAME

echo "==========================================================="
echo "SUCCESS! Service is running."
echo "Check status command: systemctl status $SERVICE_NAME"
echo "View logs command:    tail -f $INSTALL_DIR/logs/pos_service.log.current"
echo "==========================================================="
