#!/usr/bin/env bash
# Builds a universal (arm64 + x86_64) macOS .app bundle for rkg-inspector.
set -euo pipefail

cd "$(dirname "$0")/.."

APP_NAME="RKG Inspector"
BIN_NAME="rkg-inspector"
BUNDLE_ID="io.github.skylarc2006.rkg-inspector"
VERSION="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"(.*)".*/\1/')"

OUT_DIR="target/macos"
APP_DIR="$OUT_DIR/$APP_NAME.app"

echo "==> Building for aarch64-apple-darwin"
cargo build --release --target aarch64-apple-darwin

echo "==> Building for x86_64-apple-darwin"
cargo build --release --target x86_64-apple-darwin

echo "==> Creating universal binary"
mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources"
lipo -create -output "$APP_DIR/Contents/MacOS/$BIN_NAME" \
    "target/aarch64-apple-darwin/release/$BIN_NAME" \
    "target/x86_64-apple-darwin/release/$BIN_NAME"
chmod +x "$APP_DIR/Contents/MacOS/$BIN_NAME"

echo "==> Copying icon"
cp images/icon.icns "$APP_DIR/Contents/Resources/icon.icns"

echo "==> Writing Info.plist"
cat > "$APP_DIR/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleExecutable</key>
    <string>$BIN_NAME</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
PLIST

echo "==> Done: $APP_DIR"
lipo -info "$APP_DIR/Contents/MacOS/$BIN_NAME"
