---
sidebar_position: 1
---

# Developer Architecture

Technical deep-dive into PwGen-rust's architecture for contributors and developers.

## Overview

PwGen-rust is built as a modular, secure password management system leveraging Rust's memory safety guarantees and modern cryptographic standards. This document provides detailed technical information for developers contributing to or building upon PwGen-rust.

## Project Structure

```
pwgen/
├── pwgen-core/          # Core library with business logic
│   ├── src/
│   │   ├── crypto/      # Encryption/decryption modules
│   │   ├── storage/     # Database and file operations
│   │   ├── models/      # Data structures
│   │   └── lib.rs       # Library entry point
│   └── Cargo.toml
├── pwgen-cli/           # Command-line interface
│   ├── src/
│   │   ├── commands/    # CLI command implementations
│   │   └── main.rs      # CLI entry point
│   └── Cargo.toml
├── pwgen-gui/           # Graphical user interface
│   ├── src/
│   │   ├── ui/          # UI components
│   │   └── main.rs      # GUI entry point
│   └── Cargo.toml
└── pwgen-server/        # API server (in development)
    ├── src/
    └── Cargo.toml
```

## Core Components

### pwgen-core

The heart of PwGen-rust, providing:

#### Cryptography Module (`crypto/`)
```rust
// Key components
pub struct CryptoEngine {
    // AES-256-GCM for encryption
    cipher: Aes256Gcm,
    // Argon2 for key derivation
    kdf: Argon2,
}

// Main operations
impl CryptoEngine {
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>>;
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
    pub fn derive_key(&self, password: &str, salt: &[u8]) -> Result<Key>;
}
```

#### Storage Module (`storage/`)
```rust
// Database abstraction
pub trait StorageBackend {
    fn create_vault(&self, path: &Path) -> Result<Vault>;
    fn open_vault(&self, path: &Path, key: &Key) -> Result<Vault>;
    fn save_entry(&self, entry: &Entry) -> Result<()>;
    fn get_entry(&self, id: &Uuid) -> Result<Entry>;
}

// SQLite implementation
pub struct SqliteStorage {
    connection: Connection,
    crypto: CryptoEngine,
}
```

#### Models (`models/`)
```rust
// Core data structures
#[derive(Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: Uuid,
    pub site: String,
    pub username: String,
    pub password: SecureString,  // Zeroizes on drop
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub id: Uuid,
    pub name: String,
    pub secret_type: SecretType,
    pub data: SecureBytes,       // Zeroizes on drop
    pub metadata: HashMap<String, String>,
}
```

### pwgen-cli

Command-line interface built with `clap`:

```rust
// Command structure
#[derive(Parser)]
#[command(name = "pwgen-cli")]
#[command(about = "Secure password manager CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Password(PasswordCommand),
    Secret(SecretCommand),
    Generate(GenerateCommand),
    Vault(VaultCommand),
}
```

### pwgen-gui

Desktop GUI built with `egui`:

```rust
// Main application state
pub struct PwGenApp {
    vault: Option<Vault>,
    ui_state: UiState,
    crypto: CryptoEngine,
}

impl eframe::App for PwGenApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // UI rendering logic
    }
}
```

## Security Architecture

For detailed security architecture, see [Security Architecture](../security/architecture).

Key security features:
- Zero-knowledge architecture
- Memory-safe Rust implementation
- Authenticated encryption (AES-256-GCM)
- Memory-hard key derivation (Argon2)
- Automatic memory zeroization

## Database Schema

### SQLite Schema

```sql
-- Vault metadata
CREATE TABLE vault_metadata (
    id INTEGER PRIMARY KEY,
    version INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    salt BLOB NOT NULL,
    nonce BLOB NOT NULL,
    settings BLOB  -- Encrypted JSON
);

-- Password entries
CREATE TABLE passwords (
    id TEXT PRIMARY KEY,
    encrypted_data BLOB NOT NULL,
    nonce BLOB NOT NULL,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
);

-- Secrets
CREATE TABLE secrets (
    id TEXT PRIMARY KEY,
    secret_type TEXT NOT NULL,
    encrypted_data BLOB NOT NULL,
    nonce BLOB NOT NULL,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
);

-- Audit log
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TIMESTAMP NOT NULL,
    action TEXT NOT NULL,
    entity_type TEXT,
    entity_id TEXT,
    details TEXT  -- JSON
);
```

## Build System

### Dependencies

Key dependencies and their purposes:

```toml
[dependencies]
# Cryptography
aes-gcm = "0.10"          # AES-256-GCM encryption
argon2 = "0.5"            # Password hashing
rand = "0.8"              # Secure random generation
zeroize = "1.7"           # Memory clearing

# Storage
sqlcipher = "0.5"         # Encrypted SQLite
serde = "1.0"             # Serialization
uuid = "1.6"              # Unique identifiers

# UI (GUI only)
egui = "0.24"             # Immediate mode GUI
eframe = "0.24"           # egui framework

# CLI only
clap = "4.4"              # Command line parsing
```

### Build Profiles

```toml
# Development build
[profile.dev]
opt-level = 0
debug = true

# Release build
[profile.release]
opt-level = 3
lto = true
strip = true

# Security-focused build
[profile.secure]
inherits = "release"
panic = "abort"
overflow-checks = true
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let engine = CryptoEngine::new();
        let plaintext = b"secret data";
        
        let encrypted = engine.encrypt(plaintext).unwrap();
        let decrypted = engine.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
}
```

### Integration Tests

```rust
// tests/vault_operations.rs
#[test]
fn test_vault_creation_and_access() {
    let temp_dir = tempdir().unwrap();
    let vault_path = temp_dir.path().join("test.vault");
    
    // Create vault
    let vault = Vault::create(&vault_path, "test_password").unwrap();
    
    // Add entry
    let entry = PasswordEntry::new("github.com", "user@example.com");
    vault.add_password(entry).unwrap();
    
    // Verify
    let entries = vault.list_passwords().unwrap();
    assert_eq!(entries.len(), 1);
}
```

## Performance Considerations

### Memory Management

- Use `zeroize` for sensitive data
- Avoid unnecessary allocations
- Use references where possible
- Clear clipboard after timeout

### Concurrency

- Vault operations are single-threaded
- GUI runs on main thread
- Background tasks use `tokio` runtime
- Database connections are pooled

### Optimization

- Lazy loading for large vaults
- Indexed database queries
- Cached decryption for session
- Efficient search algorithms

## Contributing Guidelines

### Code Style

Follow Rust standard style:
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

### Pull Request Process

1. Fork the repository
2. Create feature branch
3. Write tests for new features
4. Ensure all tests pass
5. Update documentation
6. Submit pull request

### Security Considerations

When contributing:
- Never log sensitive data
- Always zeroize secrets
- Use constant-time comparisons
- Validate all inputs
- Follow OWASP guidelines

## API Design (Future)

The planned API server will provide:

```rust
// RESTful endpoints
POST   /api/v1/auth/login
POST   /api/v1/auth/logout
GET    /api/v1/passwords
POST   /api/v1/passwords
PUT    /api/v1/passwords/:id
DELETE /api/v1/passwords/:id

// WebSocket for real-time updates
WS     /api/v1/sync
```

## Platform-Specific Code

### Linux
```rust
#[cfg(target_os = "linux")]
fn get_config_dir() -> PathBuf {
    // XDG Base Directory compliance
    xdg::BaseDirectories::new()
        .unwrap()
        .get_config_home()
        .join("pwgen")
}
```

### macOS
```rust
#[cfg(target_os = "macos")]
fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("dev.pwgenrust.pwgen")
}
```

### Windows
```rust
#[cfg(target_os = "windows")]
fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("PwGen")
}
```

## Debugging

### Debug Builds

```bash
# Build with debug symbols
cargo build

# Run with logging
RUST_LOG=debug cargo run

# Use debugger
rust-gdb target/debug/pwgen-cli
```

### Profiling

```bash
# CPU profiling
cargo build --release
perf record --call-graph=dwarf target/release/pwgen-cli
perf report

# Memory profiling
valgrind --tool=massif target/release/pwgen-cli
```

## Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Guidelines](https://owasp.org/www-project-application-security-verification-standard/)

### Dependencies Documentation
- [aes-gcm](https://docs.rs/aes-gcm/)
- [argon2](https://docs.rs/argon2/)
- [egui](https://docs.rs/egui/)
- [clap](https://docs.rs/clap/)

### Community
- [GitHub Discussions](https://github.com/HxHippy/PWGen/discussions)
- [Issue Tracker](https://github.com/HxHippy/PWGen/issues)
- [Twitter Updates](https://x.com/HxHippy)