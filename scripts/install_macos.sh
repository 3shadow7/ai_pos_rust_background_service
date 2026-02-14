#!/bin/bash
# ==================================================================================
# AUTOMATIC INSTALLER FOR macOS
# ==================================================================================
set -e

echo "Installing POS Hardware Service for macOS..."

# 1. Paths
INSTALL_DIR="/usr/local/bin"
SERVICE_NAME="com.pos.hardware"
PLIST_PATH="$HOME/Library/LaunchAgents/$SERVICE_NAME.plist"
SCRIPT_DIR="$(dirname "$0")"
EXE_SOURCE="$SCRIPT_DIR/pos_hardware_service"
CONFIG_SOURCE="$SCRIPT_DIR/config.toml"

# 2. Check source
if [ ! -f "$EXE_SOURCE" ]; then
    echo "Error: Could not find pos_hardware_service in current directory."
    exit 1
fi

# 3. Install Binary
echo "Copying binary to $INSTALL_DIR..."
sudo mkdir -p "$INSTALL_DIR"
sudo cp "$EXE_SOURCE" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/pos_hardware_service"

# 4. Install Config (MacOS doesn't have a standard /etc/ folder for user apps usually, but let's use ~/.pos_hardware)
CONFIG_DIR="$HOME/.pos_hardware"
mkdir -p "$CONFIG_DIR"
cp "$CONFIG_SOURCE" "$CONFIG_DIR/config.toml"

echo "Configuration installed to $CONFIG_DIR/config.toml"

# 5. Create Launch Agent Plist
echo "Creating Launch Agent..."
mkdir -p "$HOME/Library/LaunchAgents"

cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>$SERVICE_NAME</string>
    <key>ProgramArguments</key>
    <array>
        <string>$INSTALL_DIR/pos_hardware_service</string>
    </array>
    <key>EnvironmentVariables</key>
    <dict>
        <key>POS_CONFIG_DIR</key>
        <string>$CONFIG_DIR</string>
    </dict>
    <key>WorkingDirectory</key>
    <string>$CONFIG_DIR</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$CONFIG_DIR/pos_service.log</string>
    <key>StandardErrorPath</key>
    <string>$CONFIG_DIR/pos_service.err</string>
</dict>
</plist>
EOF

# 6. Load Service
echo "Loading Service..."
launchctl unload "$PLIST_PATH" 2>/dev/null || true
launchctl load "$PLIST_PATH"

echo "=========================================="
echo "SUCCESS! Service installed and running."
echo "Logs are located in $CONFIG_DIR"
echo "=========================================="
