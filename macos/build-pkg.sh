#!/bin/bash
# PwGen macOS PKG Installer Build Script
# Builds a standard macOS installer package

set -e

PRODUCT_NAME="PwGen"
PRODUCT_VERSION="1.2.0"
BUNDLE_ID="dev.pwgenrust.pwgen"
INSTALL_LOCATION="/Applications/PwGen"
BUILD_DIR="target/x86_64-apple-darwin/min-size"
PKG_DIR="macos/pkg-build"
SCRIPTS_DIR="$PKG_DIR/scripts"
PAYLOAD_DIR="$PKG_DIR/payload"

echo "Building PwGen macOS PKG installer..."

# Clean and create build directories
rm -rf "$PKG_DIR"
mkdir -p "$PAYLOAD_DIR$INSTALL_LOCATION"
mkdir -p "$SCRIPTS_DIR"

# Copy application files
echo "Copying application files..."
cp "$BUILD_DIR/pwgen-gui" "$PAYLOAD_DIR$INSTALL_LOCATION/"
cp "$BUILD_DIR/pwgen-cli" "$PAYLOAD_DIR$INSTALL_LOCATION/"

# Copy documentation
cp README.md "$PAYLOAD_DIR$INSTALL_LOCATION/"
cp LICENSE "$PAYLOAD_DIR$INSTALL_LOCATION/"
cp CHANGELOG.md "$PAYLOAD_DIR$INSTALL_LOCATION/"

# Copy icon
cp assets/PWGenLogo.png "$PAYLOAD_DIR$INSTALL_LOCATION/"

# Create CLI symlinks script
cat > "$SCRIPTS_DIR/postinstall" << 'EOF'
#!/bin/bash
# Create symlinks for CLI access
ln -sf /Applications/PwGen/pwgen-cli /usr/local/bin/pwgen-cli
ln -sf /Applications/PwGen/pwgen-cli /usr/local/bin/pwgen

# Set executable permissions
chmod +x /Applications/PwGen/pwgen-gui
chmod +x /Applications/PwGen/pwgen-cli

# Create application data directory
mkdir -p ~/Library/Application\ Support/PwGen

echo "PwGen installation completed successfully!"
echo "GUI: /Applications/PwGen/pwgen-gui"
echo "CLI: pwgen-cli (added to PATH)"
EOF

chmod +x "$SCRIPTS_DIR/postinstall"

# Create preinstall script to remove old versions
cat > "$SCRIPTS_DIR/preinstall" << 'EOF'
#!/bin/bash
# Remove old installation if it exists
if [ -d "/Applications/PwGen" ]; then
    echo "Removing previous PwGen installation..."
    rm -rf "/Applications/PwGen"
fi

# Remove old symlinks
rm -f /usr/local/bin/pwgen-cli
rm -f /usr/local/bin/pwgen

exit 0
EOF

chmod +x "$SCRIPTS_DIR/preinstall"

# Build the package
echo "Building PKG installer..."
pkgbuild \
    --root "$PAYLOAD_DIR" \
    --scripts "$SCRIPTS_DIR" \
    --identifier "$BUNDLE_ID" \
    --version "$PRODUCT_VERSION" \
    --install-location "/" \
    "pwgen-${PRODUCT_VERSION}-macos-x64.pkg"

# Create distribution XML for productbuild (optional, for more control)
cat > "$PKG_DIR/distribution.xml" << EOF
<?xml version="1.0" encoding="utf-8"?>
<installer-gui-script minSpecVersion="2">
    <title>$PRODUCT_NAME $PRODUCT_VERSION</title>
    <organization>dev.pwgenrust</organization>
    <domains enable_anywhere="true"/>
    <options customize="never" require-scripts="true" rootVolumeOnly="true"/>
    
    <welcome file="welcome.html" mime-type="text/html"/>
    <readme file="readme.html" mime-type="text/html"/>
    <license file="license.html" mime-type="text/html"/>
    
    <pkg-ref id="$BUNDLE_ID"/>
    
    <choices-outline>
        <line choice="default">
            <line choice="$BUNDLE_ID"/>
        </line>
    </choices-outline>
    
    <choice id="default"/>
    <choice id="$BUNDLE_ID" visible="false">
        <pkg-ref id="$BUNDLE_ID"/>
    </choice>
    
    <pkg-ref id="$BUNDLE_ID" version="$PRODUCT_VERSION" onConclusion="none">pwgen-${PRODUCT_VERSION}-macos-x64.pkg</pkg-ref>
</installer-gui-script>
EOF

echo "PKG installer created: pwgen-${PRODUCT_VERSION}-macos-x64.pkg"
echo "Size: $(du -h pwgen-${PRODUCT_VERSION}-macos-x64.pkg | cut -f1)"