# PwGen Architecture

This document provides an overview of PwGen's system architecture, design decisions, and component interactions.

## 🏗️ High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   pwgen-gui     │    │   pwgen-cli     │    │ pwgen-extension │
│   (Desktop UI)  │    │ (Command Line)  │    │  (Browser Ext)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                         ┌─────────────────┐
                         │   pwgen-core    │
                         │ (Core Library)  │
                         └─────────────────┘
                                 │
                    ┌────────────┼────────────┐
                    │            │            │
              ┌──────────┐ ┌──────────┐ ┌──────────┐
              │Encryption│ │ Storage  │ │Generator │
              │ Module   │ │ Module   │ │ Module   │
              └──────────┘ └──────────┘ └──────────┘
                                 │
                         ┌─────────────────┐
                         │  SQLite Database│
                         │  (Encrypted)    │
                         └─────────────────┘
```

## 📦 Component Overview

### pwgen-core
The foundational library containing all core functionality:

- **Cryptography**: AES-256-GCM encryption, Argon2 key derivation
- **Storage**: SQLite database operations with encryption
- **Models**: Data structures and business logic
- **Generators**: Password and key generation algorithms
- **Secrets Management**: Advanced secret types (API keys, SSH keys, etc.)
- **Import/Export**: Browser and password manager integration

### pwgen-cli
Command-line interface providing:

- **Vault Management**: Create, unlock, lock operations
- **Entry Operations**: Add, search, update, delete passwords
- **Secret Management**: Handle API keys, SSH keys, documents
- **Batch Operations**: Import/export, backup/restore
- **Automation**: Scriptable interface for CI/CD

### pwgen-gui
Desktop GUI application featuring:

- **Native Performance**: Built with egui for fast rendering
- **Responsive Design**: Adaptive to different screen sizes
- **Tabbed Interface**: Organized access to different features
- **Visual Feedback**: Progress indicators and status updates
- **Cross-Platform**: Consistent experience across OS platforms

### pwgen-extension (Planned)
Browser extension providing:

- **Auto-Fill**: Seamless password insertion
- **Auto-Save**: Detect and save new credentials
- **Local Integration**: Secure communication with local vault
- **Cross-Browser**: Support for Chrome, Firefox, Safari, Edge

## 🔒 Security Architecture

### Encryption Layer

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│                   Encryption Service                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Key Manager │  │ Cipher Core │  │  Memory Protection  │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                   Storage Service                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Transaction │  │  Integrity  │  │   Backup System     │  │
│  │   Manager   │  │   Checker   │  │                     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Database Layer                           │
│                   (SQLite + WAL)                            │
└─────────────────────────────────────────────────────────────┘
```

### Key Management

1. **Master Key Derivation**:
   ```
   Master Password → Argon2 → Derived Key → AES-256-GCM Key
   ```

2. **Salt Generation**: Cryptographically secure random salt per vault

3. **Key Storage**: Never stored persistently, derived on vault unlock

4. **Memory Protection**: Zeroized after use

### Data Flow Security

```
Plaintext Data → Validation → Encryption → Storage → Retrieval → Decryption → Memory Cleanup
     ↓              ↓            ↓           ↓           ↓            ↓            ↓
  Input Sanitization  ↓     AES-256-GCM    SQLite   Integrity   AES-256-GCM   Zeroization
                 Rate Limiting              WAL      Verification              
```

## 🗄️ Database Schema

### Core Tables

#### vault_metadata
```sql
CREATE TABLE vault_metadata (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    version TEXT NOT NULL,
    master_password_hash TEXT NOT NULL,
    salt BLOB NOT NULL
);
```

#### password_entries
```sql
CREATE TABLE password_entries (
    id TEXT PRIMARY KEY,
    site TEXT NOT NULL,
    username TEXT NOT NULL,
    encrypted_password BLOB NOT NULL,
    notes TEXT,
    tags TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_used TEXT,
    password_changed_at TEXT NOT NULL,
    favorite INTEGER NOT NULL DEFAULT 0
);
```

#### secrets
```sql
CREATE TABLE secrets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    secret_type TEXT NOT NULL,
    encrypted_data BLOB NOT NULL,
    metadata_json TEXT NOT NULL,
    tags TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_accessed TEXT,
    expires_at TEXT,
    favorite INTEGER NOT NULL DEFAULT 0
);
```

#### secret_audit_log
```sql
CREATE TABLE secret_audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    secret_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    action TEXT NOT NULL,
    user_name TEXT,
    details TEXT,
    FOREIGN KEY (secret_id) REFERENCES secrets (id) ON DELETE CASCADE
);
```

### Indexing Strategy

- **Performance Indexes**: Site, username, updated_at, favorite
- **Security Indexes**: No indexes on encrypted data
- **Search Optimization**: Full-text search on plaintext fields only

## 🚀 Performance Considerations

### Database Optimization

- **WAL Mode**: Write-Ahead Logging for better concurrency
- **Connection Pooling**: Efficient database connection management
- **Prepared Statements**: SQL injection prevention and performance
- **Batch Operations**: Minimize transaction overhead

### Memory Management

- **Lazy Loading**: Load data only when needed
- **Streaming**: Handle large datasets efficiently
- **Cleanup**: Aggressive cleanup of sensitive data
- **Caching**: Smart caching of non-sensitive metadata

### UI Responsiveness

- **Async Operations**: Non-blocking database operations
- **Progressive Loading**: Show data as it becomes available
- **Virtual Scrolling**: Handle large entry lists efficiently
- **Debounced Search**: Optimize search performance

## 🔧 Design Patterns

### Error Handling

```rust
// Comprehensive error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Invalid master password")]
    InvalidMasterPassword,
}

// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
```

### Secure Data Handling

```rust
// Automatic cleanup
#[derive(Zeroize)]
pub struct SecretData {
    sensitive_field: String,
}

impl Drop for SecretData {
    fn drop(&mut self) {
        self.zeroize();
    }
}
```

### Async Architecture

```rust
// Async-first design
pub struct Storage {
    pool: sqlx::SqlitePool,
    master_key: MasterKey,
}

impl Storage {
    pub async fn add_entry(&self, entry: &Entry) -> Result<()> {
        // Non-blocking database operations
    }
}
```

## 🏛️ Architectural Principles

### Security by Design
- **Zero-Knowledge**: No plaintext sensitive data stored
- **Defense in Depth**: Multiple security layers
- **Fail Secure**: Errors default to secure state
- **Minimal Privilege**: Components access only needed data

### Performance by Design
- **Local-First**: No network dependencies for core functionality
- **Efficient Algorithms**: O(log n) search and retrieval
- **Resource Management**: Memory and file handle cleanup
- **Scalability**: Handle thousands of entries efficiently

### Maintainability by Design
- **Modular Architecture**: Clear separation of concerns
- **Type Safety**: Rust's type system prevents many bugs
- **Comprehensive Testing**: Unit, integration, and end-to-end tests
- **Documentation**: Extensive code and API documentation

## 🔄 Data Flow Diagrams

### Password Entry Creation

```
User Input → Validation → Encryption → Database → Confirmation
    ↓            ↓            ↓           ↓           ↓
Site/User    Input Sanit.  AES-256-GCM  SQLite     UI Update
Password     Length Check  Key Derivat.  Transaction Success Msg
Notes/Tags   Type Valid.   Secure Mem.   Integrity   Log Entry
```

### Vault Unlock Process

```
Master Password → Key Derivation → Database Unlock → Load Metadata → UI Unlock
      ↓               ↓                ↓               ↓              ↓
   User Input     Argon2           Connection Pool   Verify Schema   Show Entries
   Validation     100k+ Iterations  Authorization    Load Indexes    Enable Features
   Rate Limit     Salt Lookup      Transaction       Cache Metadata  Clear Errors
```

### Search and Retrieval

```
Search Query → Index Lookup → Database Query → Decryption → UI Display
     ↓             ↓              ↓             ↓            ↓
  Sanitization  B-Tree Search   SQL Prepared   AES Decrypt  Result List
  Validation    Filter Index    Statement      Memory Mgmt  Pagination
  Rate Limit    Sort Options    Fetch Results  Zeroize      Highlighting
```

## 🧪 Testing Architecture

### Test Pyramid

```
                    ┌────────────┐
                    │    E2E     │  ← Full workflow tests
                    │   Tests    │
                ┌───┴────────────┴───┐
                │  Integration Tests │  ← Component interaction
                │                    │
        ┌───────┴────────────────────┴───────┐
        │           Unit Tests               │  ← Individual functions
        │                                    │
        └────────────────────────────────────┘
```

### Security Testing

- **Cryptographic Verification**: Test encryption/decryption cycles
- **Key Derivation Testing**: Verify Argon2 implementation
- **Memory Safety**: Test for leaks and cleanup
- **Input Validation**: Fuzzing and boundary testing
- **Authentication**: Master password verification tests

### Performance Testing

- **Load Testing**: Large dataset performance
- **Memory Profiling**: Memory usage optimization
- **Startup Time**: Application launch performance
- **Search Performance**: Query optimization verification

## 📈 Scalability Considerations

### Current Limitations

- **Single User**: Designed for individual use
- **Local Storage**: No cloud synchronization
- **Memory Constraints**: Large datasets may impact performance

### Future Scalability

- **Multi-User Support**: Team and organization features
- **Cloud Integration**: End-to-end encrypted sync
- **Mobile Support**: iOS and Android applications
- **API Server**: REST API for integrations

## 🔮 Future Architecture Evolution

### Planned Enhancements

1. **Microservice Architecture**: Split into focused services
2. **Plugin System**: Extensible functionality
3. **Cloud Backend**: Optional encrypted cloud storage
4. **Real-time Sync**: Multi-device synchronization
5. **Mobile Architecture**: Native mobile applications

### Technology Roadmap

- **WebAssembly**: Browser-based core functionality
- **Hardware Security**: HSM and TPM integration
- **Advanced Crypto**: Post-quantum cryptography preparation
- **Zero-Knowledge Sync**: Cloud sync without server access to data

---

This architecture document is living documentation that evolves with the project. For the most current information, see the source code and recent commits.