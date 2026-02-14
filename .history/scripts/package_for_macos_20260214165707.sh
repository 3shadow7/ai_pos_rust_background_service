#!/bin/bash
set -e

echo "=========================================="
echo "   POS SERVICE: BUILD & PACKAGE (macOS)"
echo "=========================================="

# 1. Build
echo "1. Building Release Version..."
cargo build --release

# 2. Prepare Dist Folder
DIST_DIR="$(dirname "$0")/../dist_macos"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# 3. Copy Files
echo "2. Packaging files..."
TARGET_DIR="$(dirname "$0")/../target/release"
PROJECT_ROOT="$(dirname "$0")/.."

cp "$TARGET_DIR/pos_hardware_service" "$DIST_DIR/"
cp "$PROJECT_ROOT/config.toml" "$DIST_DIR/"
cp "$PROJECT_ROOT/scripts/install_macos.sh" "$DIST_DIR/install.sh"
chmod +x "$DIST_DIR/install.sh"
chmod +x "$DIST_DIR/pos_hardware_service"

echo "3. Creating Zip..."
cd "$(dirname "$0")/.."
zip -r pos_hardware_macos.zip dist_macos

echo "=========================================="
echo "DONE! Package created at:"
echo "$(pwd)/pos_hardware_macos.zip"
echo "=========================================="
