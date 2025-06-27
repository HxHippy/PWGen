#!/bin/bash

# Test CLI functionality
echo "Testing pwgen CLI..."

# Test password generation
echo "1. Generating passwords:"
cargo run -p pwgen-cli -- generate --length 16
cargo run -p pwgen-cli -- generate --length 20 --no-symbols
cargo run -p pwgen-cli -- generate --passphrase --words 4

echo -e "\n2. Generating escaped password:"
cargo run -p pwgen-cli -- generate --length 16 --escape

echo -e "\nCLI test complete. To test the full system:"
echo "  1. Initialize vault: cargo run -p pwgen-cli -- init"
echo "  2. Add password: cargo run -p pwgen-cli -- add github.com myuser --generate"
echo "  3. Get password: cargo run -p pwgen-cli -- get github.com --show"
echo "  4. List entries: cargo run -p pwgen-cli -- list"