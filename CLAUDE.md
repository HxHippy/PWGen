# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pwgen is a personal password management system written in Rust, featuring:
- Full encryption using AES-256-GCM
- Secure password generation with customizable complexity
- SQLite-based encrypted storage
- CLI interface with Tauri GUI and planned browser extension support
- Cross-platform compatibility

## Architecture

The project uses a Cargo workspace with the following structure:
- `pwgen-core/`: Core library with encryption, storage, and password generation
- `pwgen-cli/`: Command-line interface
- `pwgen-gui/`: Tauri-based GUI application with modern web UI
- `pwgen-server/`: API server for browser extension (planned)

## Common Commands

### Build
```bash
# Build all workspace members
cargo build

# Build release version
cargo build --release

# Build specific package
cargo build -p pwgen-cli
```

### Run
```bash
# Run CLI
cargo run -p pwgen-cli -- [commands]

# Initialize a new vault
cargo run -p pwgen-cli -- init

# Generate a password
cargo run -p pwgen-cli -- generate --length 20

# Run GUI application
cargo run -p pwgen-gui

# Create backup
cargo run -p pwgen-cli -- backup --output my_backup.json

# Verify backup
cargo run -p pwgen-cli -- verify-backup my_backup.json

# Restore backup
cargo run -p pwgen-cli -- restore --backup-file my_backup.json
```

### Test
```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p pwgen-core

# Run tests with output
cargo test -- --nocapture
```

### Format and Lint
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings
```

## Key Implementation Details

### Encryption (pwgen-core/src/crypto.rs)
- Uses Argon2 for key derivation from master password
- AES-256-GCM for password encryption
- Automatic nonce generation and storage
- Zeroize for secure memory handling

### Storage (pwgen-core/src/storage.rs)
- SQLite database with encrypted password entries
- Async operations using sqlx
- Support for search, filtering, and tagging
- Entry IDs are SHA-256 hashes of site+username

### Password Generation (pwgen-core/src/generator.rs)
- Configurable character sets and length (8-128 chars)
- Minimum character requirements per type
- Shell-escaped password generation
- Passphrase generation with word lists

### CLI Commands
- `init`: Initialize new vault
- `add`: Add new password entry
- `get`: Retrieve password (with clipboard support on Linux)
- `list`: List all entries with filtering
- `update`: Update existing entry
- `delete`: Remove entry
- `generate`: Generate passwords without storing
- `backup`: Create encrypted backups (full or incremental)
- `restore`: Restore from encrypted backup files
- `verify-backup`: Verify backup integrity and metadata
- `import/export`: Planned for browser password import

### GUI Features
- Modern dark theme interface with responsive design
- Vault initialization and unlock with master password
- Interactive password generator with real-time preview
- Entry management (add, view, edit, delete)
- Search and filtering capabilities
- Cross-platform clipboard integration
- Secure state management with encrypted storage
- Backup creation and verification (restore via CLI)

### Backup & Restore System
- **Full Backups**: Complete vault encryption with separate backup password
- **Incremental Backups**: Only entries modified since specified date
- **Integrity Verification**: SHA-256 checksums and file size validation
- **Conflict Resolution**: Three strategies (merge, overwrite, skip)
- **Security**: Separate backup encryption, metadata protection
- **CLI Integration**: Complete command-line backup workflow
- **GUI Support**: Backup creation and verification through desktop app

## Security Considerations

- Master passwords are never stored in plaintext
- All passwords are encrypted at rest
- Memory is securely wiped using Zeroize
- Database queries use parameterized statements
- Clipboard integration clears after use (planned)