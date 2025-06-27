#!/bin/bash

echo "🔐 PwGen - Personal Password Manager Demo"
echo "========================================"
echo
echo "This system provides:"
echo "✅ AES-256-GCM encryption with Argon2 key derivation"
echo "✅ Secure password generation (8-128 characters)"
echo "✅ SQLite database with encrypted storage"
echo "✅ Full CLI interface"
echo "✅ Cross-platform Tauri GUI with modern web UI"
echo "✅ Real clipboard integration"
echo "✅ Search and filtering capabilities"
echo
echo "📋 Available interfaces:"
echo
echo "1. CLI Interface:"
echo "   cargo run -p pwgen-cli -- --help"
echo "   cargo run -p pwgen-cli -- generate --length 20"
echo "   cargo run -p pwgen-cli -- init"
echo
echo "2. GUI Application:"
echo "   cargo run -p pwgen-gui"
echo
echo "🧪 Quick CLI test:"
echo "Generating secure passwords..."
echo

# Test password generation
echo "Standard password (16 chars):"
cargo run -p pwgen-cli -- generate --length 16

echo
echo "Numbers only (12 chars):"
cargo run -p pwgen-cli -- generate --length 12 --no-uppercase --no-lowercase --no-symbols

echo
echo "Passphrase (4 words):"
cargo run -p pwgen-cli -- generate --passphrase --words 4

echo
echo "Shell-escaped password:"
cargo run -p pwgen-cli -- generate --length 16 --escape

echo
echo "🚀 System Ready!"
echo
echo "To test the full system:"
echo "1. Run 'cargo run -p pwgen-gui' for the GUI"
echo "2. Initialize a new vault with a strong master password"
echo "3. Add some password entries"
echo "4. Test search and clipboard functionality"
echo
echo "The vault will be stored at: ~/.local/share/pwgen/vault.db"