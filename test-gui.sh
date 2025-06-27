#!/bin/bash
# Test script to run the GUI and capture any errors

echo "Removing any existing vault files..."
rm -f ~/.local/share/pwgen/vault.db
rm -f ~/.local/share/pwgen/secrets_vault.db

echo "Starting GUI..."
cargo run -p pwgen-gui