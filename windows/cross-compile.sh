#!/bin/bash
# Cross-compilation setup for Windows targets from Linux
# Uses mingw-w64 toolchain

set -e

echo "Setting up Windows cross-compilation environment..."

# Check if mingw-w64 is installed
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Installing mingw-w64 cross-compilation toolchain..."
    sudo apt update
    sudo apt install -y mingw-w64 gcc-mingw-w64-x86-64
fi

# Add Rust target if not already added
if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
    echo "Adding x86_64-pc-windows-gnu target..."
    rustup target add x86_64-pc-windows-gnu
fi

# Set up Cargo configuration for Windows cross-compilation
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << 'EOF'

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[target.x86_64-pc-windows-msvc]
# Note: MSVC target requires additional setup with xwin or similar tools
# For now, use the GNU target for cross-compilation from Linux
EOF

# Build for Windows
echo "Building for Windows (x86_64-pc-windows-gnu)..."
cargo build --profile min-size --target x86_64-pc-windows-gnu --features "clipboard,document-compression"

# Create a symbolic link for MSVC-style paths (for installer scripts)
mkdir -p target/x86_64-pc-windows-msvc/min-size
ln -sf ../../x86_64-pc-windows-gnu/min-size/pwgen-gui.exe target/x86_64-pc-windows-msvc/min-size/
ln -sf ../../x86_64-pc-windows-gnu/min-size/pwgen-cli.exe target/x86_64-pc-windows-msvc/min-size/

echo "Windows build completed!"
echo "Binaries location: target/x86_64-pc-windows-gnu/min-size/"
echo ""
echo "Next steps:"
echo "1. Install NSIS on a Windows machine or Wine:"
echo "   sudo apt install nsis"
echo "2. Run: makensis windows/pwgen.nsi"
echo "3. For MSI, install WiX Toolset and run:"
echo "   candle.exe windows/pwgen.wxs"
echo "   light.exe pwgen.wixobj"
echo ""
echo "Note: Some installer features may require Windows or Wine for proper generation."