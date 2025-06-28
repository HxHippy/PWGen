#!/bin/bash
# Cross-compilation setup for macOS targets from Linux
# Requires osxcross toolchain

set -e

echo "Setting up macOS cross-compilation environment..."

# Check if osxcross is installed
if ! command -v x86_64-apple-darwin21-clang &> /dev/null; then
    echo "Error: osxcross toolchain not found!"
    echo "Please install osxcross first:"
    echo "  git clone https://github.com/tpoechtrager/osxcross"
    echo "  cd osxcross"
    echo "  # Follow osxcross installation instructions"
    exit 1
fi

# Set up environment for cross-compilation
export CC=x86_64-apple-darwin21-clang
export CXX=x86_64-apple-darwin21-clang++
export AR=x86_64-apple-darwin21-ar
export RANLIB=x86_64-apple-darwin21-ranlib

# Add Rust target if not already added
if ! rustup target list --installed | grep -q "x86_64-apple-darwin"; then
    echo "Adding x86_64-apple-darwin target..."
    rustup target add x86_64-apple-darwin
fi

# Build for macOS
echo "Building for macOS (x86_64-apple-darwin)..."
cargo build --profile min-size --target x86_64-apple-darwin --features "clipboard,document-compression"

echo "macOS build completed!"
echo "Binaries location: target/x86_64-apple-darwin/min-size/"
echo ""
echo "Next steps:"
echo "1. Run ./macos/build-pkg.sh to create PKG installer"
echo "2. Run ./macos/build-dmg.sh to create DMG installer"
echo ""
echo "Note: DMG creation requires macOS. Consider using GitHub Actions for automated builds."