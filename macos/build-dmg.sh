#!/bin/bash
# PwGen macOS DMG Installer Build Script
# Creates a disk image with drag-and-drop installation

set -e

PRODUCT_NAME="PwGen"
PRODUCT_VERSION="1.2.0"
BUILD_DIR="target/x86_64-apple-darwin/min-size"
DMG_DIR="macos/dmg-build"
APP_DIR="$DMG_DIR/PwGen.app"
TEMP_DMG="pwgen-temp.dmg"
FINAL_DMG="pwgen-${PRODUCT_VERSION}-macos-x64.dmg"

echo "Building PwGen macOS DMG installer..."

# Clean and create build directories
rm -rf "$DMG_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Create Info.plist for the app bundle
cat > "$APP_DIR/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>pwgen-gui</string>
    <key>CFBundleIdentifier</key>
    <string>dev.pwgenrust.pwgen</string>
    <key>CFBundleName</key>
    <string>PwGen</string>
    <key>CFBundleDisplayName</key>
    <string>PwGen</string>
    <key>CFBundleVersion</key>
    <string>$PRODUCT_VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$PRODUCT_VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>PWGN</string>
    <key>CFBundleIconFile</key>
    <string>pwgen.icns</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>pwgen</string>
            </array>
            <key>CFBundleTypeName</key>
            <string>PwGen Vault</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>dev.pwgenrust.vault</string>
            </array>
        </dict>
    </array>
    <key>UTExportedTypeDeclarations</key>
    <array>
        <dict>
            <key>UTTypeIdentifier</key>
            <string>dev.pwgenrust.vault</string>
            <key>UTTypeDescription</key>
            <string>PwGen Vault File</string>
            <key>UTTypeConformsTo</key>
            <array>
                <string>public.data</string>
            </array>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key>
                <array>
                    <string>pwgen</string>
                </array>
            </dict>
        </dict>
    </array>
</dict>
</plist>
EOF

# Copy application files
echo "Copying application files..."
cp "$BUILD_DIR/pwgen-gui" "$APP_DIR/Contents/MacOS/"
cp "$BUILD_DIR/pwgen-cli" "$APP_DIR/Contents/MacOS/"

# Copy and convert icon (note: would need icns conversion in real build)
cp assets/PWGenLogo.png "$APP_DIR/Contents/Resources/pwgen.png"

# Copy documentation to DMG root
cp README.md "$DMG_DIR/"
cp LICENSE "$DMG_DIR/"

# Create CLI installer script
cat > "$DMG_DIR/Install CLI Tools.command" << 'EOF'
#!/bin/bash
# Install PwGen CLI tools to system PATH

echo "Installing PwGen CLI tools..."

# Check if /usr/local/bin exists
if [ ! -d "/usr/local/bin" ]; then
    echo "Creating /usr/local/bin directory..."
    sudo mkdir -p /usr/local/bin
fi

# Get the directory of this script (DMG mount point)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_PATH="$SCRIPT_DIR/PwGen.app/Contents/MacOS"

# Create symlinks
echo "Creating symlinks..."
sudo ln -sf "$APP_PATH/pwgen-cli" /usr/local/bin/pwgen-cli
sudo ln -sf "$APP_PATH/pwgen-cli" /usr/local/bin/pwgen

echo "CLI tools installed successfully!"
echo "You can now use 'pwgen-cli' or 'pwgen' from the terminal."
echo ""
echo "Press any key to continue..."
read -n 1
EOF

chmod +x "$DMG_DIR/Install CLI Tools.command"

# Create Applications symlink for drag-and-drop
ln -s /Applications "$DMG_DIR/Applications"

# Create background image script (placeholder)
mkdir -p "$DMG_DIR/.background"
# Note: In a real build, you'd copy a custom background image here
# cp assets/dmg-background.png "$DMG_DIR/.background/"

# Calculate required size for DMG
SIZE=$(du -sk "$DMG_DIR" | cut -f1)
SIZE=$((SIZE + 10000))  # Add 10MB buffer

echo "Creating temporary DMG..."
hdiutil create -srcfolder "$DMG_DIR" -volname "$PRODUCT_NAME" -fs HFS+ \
    -fsargs "-c c=64,a=16,e=16" -format UDRW -size ${SIZE}k "$TEMP_DMG"

echo "Mounting temporary DMG..."
DEVICE=$(hdiutil attach -readwrite -noverify -noautoopen "$TEMP_DMG" | \
    egrep '^/dev/' | sed 1q | awk '{print $1}')

sleep 2

# Configure DMG appearance with AppleScript
osascript << EOF
tell application "Finder"
    tell disk "$PRODUCT_NAME"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {100, 100, 600, 400}
        set theViewOptions to the icon view options of container window
        set arrangement of theViewOptions to not arranged
        set icon size of theViewOptions to 128
        set background picture of theViewOptions to file ".background:dmg-background.png"
        
        -- Position icons
        set position of item "PwGen.app" of container window to {150, 200}
        set position of item "Applications" of container window to {350, 200}
        set position of item "Install CLI Tools.command" of container window to {250, 300}
        
        close
        open
        update without registering applications
        delay 2
    end tell
end tell
EOF

# Unmount temporary DMG
hdiutil detach "$DEVICE"

echo "Creating final compressed DMG..."
hdiutil convert "$TEMP_DMG" -format UDZO -imagekey zlib-level=9 -o "$FINAL_DMG"

# Clean up
rm -f "$TEMP_DMG"
rm -rf "$DMG_DIR"

echo "DMG installer created: $FINAL_DMG"
echo "Size: $(du -h $FINAL_DMG | cut -f1)"