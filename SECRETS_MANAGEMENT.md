# 🔐 PwGen Secrets Management System

## 🎯 Overview

The PwGen password manager has been expanded into a comprehensive secrets management system that can securely store and manage various types of sensitive information beyond just passwords.

## 🔑 Supported Secret Types

### 1. **Passwords** 
- Traditional username/password combinations
- URLs and notes support
- Enhanced password generation

### 2. **SSH Keys**
- Private and public key pairs
- Support for RSA, Ed25519, ECDSA, DSA key types
- Optional passphrase protection
- Key metadata (comments, fingerprints)

### 3. **API Keys & Tokens**
- API keys with optional secrets
- Endpoint URLs and scope definitions
- Rate limit information
- Expiration tracking

### 4. **Secure Documents**
- Encrypted file storage
- Multiple content types supported
- Integrity verification with checksums

### 5. **Configuration Files**
- Environment variables and settings
- Support for JSON, YAML, TOML, XML formats
- Template-based configurations

### 6. **Secure Notes**
- Plain text, Markdown, HTML, Rich Text
- Encrypted storage for sensitive information

### 7. **Digital Certificates**
- X.509, SSL, Code Signing certificates
- Private keys and CA chains
- Certificate metadata

### 8. **Database Connections**
- Connection strings for all major databases
- SSL configuration support
- Credential management

### 9. **Cloud Credentials**
- AWS, GCP, Azure, and other cloud providers
- Access keys and regions
- Custom configuration support

### 10. **Custom Secret Types**
- Flexible schema-based secrets
- User-defined field structures

## 🛠️ CLI Commands

### Core Secret Management
```bash
# Add different types of secrets
pwgen-cli add-secret --name "MySecret" --secret-type password
pwgen-cli add-secret --name "ServerKey" --secret-type ssh-key
pwgen-cli add-secret --name "GitHubToken" --secret-type api-key
pwgen-cli add-secret --name "DevNotes" --secret-type note

# List and search secrets
pwgen-cli list-secrets
pwgen-cli list-secrets --secret-type api-key
pwgen-cli list-secrets --query "github"
pwgen-cli list-secrets --environment "production"
pwgen-cli list-secrets --favorites

# Get secret details
pwgen-cli get-secret "MySecret"
pwgen-cli get-secret "MySecret" --show

# Update secrets
pwgen-cli update-secret "MySecret" --description "Updated description"
pwgen-cli update-secret "MySecret" --tags dev,important

# Delete secrets
pwgen-cli delete-secret "MySecret"
pwgen-cli delete-secret "MySecret" --force
```

### Templates and Management
```bash
# List available templates
pwgen-cli list-templates

# Check expiring secrets
pwgen-cli expiring-secrets --within-days 30

# View statistics
pwgen-cli secrets-stats
```

## 🏗️ Architecture

### Core Components

1. **`secrets.rs`** - Core data models and types
   - `SecretType` enum for different secret categories
   - `SecretData` enum for type-specific data structures
   - `SecretMetadata` for compliance and audit information
   - Memory-safe operations with automatic cleanup

2. **`secrets_storage.rs`** - Storage layer
   - SQLite-based encrypted storage
   - Advanced search and filtering
   - Audit logging for compliance
   - Statistics and reporting

3. **`SecretManager`** - Business logic
   - Encryption/decryption operations
   - Expiration management
   - Template system

### Security Features

- **Multi-layer Encryption**: Secrets are encrypted within the already-encrypted vault
- **Memory Safety**: Automatic zeroization of sensitive data
- **Audit Logging**: Complete audit trail for compliance
- **Access Tracking**: Last accessed timestamps
- **Expiration Management**: Optional expiration dates with alerts

### Templates System

Pre-built templates for common services:
- **AWS Credentials** - Access keys with validation patterns
- **Database Connections** - Connection strings with SSL config
- **SSH Keys** - Key pairs with metadata
- **API Keys** - Tokens with scopes and endpoints

## 📊 Database Schema

### Secrets Table
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

### Audit Log Table
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

## 🔍 Search and Filtering

The system supports advanced filtering by:
- **Name/Description** - Full-text search
- **Secret Type** - Filter by category
- **Tags** - Multiple tag support
- **Environment** - Development, staging, production
- **Project** - Organize by project
- **Expiration** - Find expiring secrets
- **Favorites** - Quick access to important secrets

## 🎨 Usage Examples

### Adding an AWS Secret
```bash
pwgen-cli add-secret \
  --name "AWS Production" \
  --secret-type cloud \
  --description "Production AWS credentials" \
  --tags aws,production
```

### Adding an SSH Key
```bash
pwgen-cli add-secret \
  --name "Production Server" \
  --secret-type ssh-key \
  --description "Main production server access"
```

### Finding Expiring Secrets
```bash
pwgen-cli expiring-secrets --within-days 7
```

## 🚀 Future Enhancements

The foundation is now in place for additional features:

1. **File Attachments** - Store documents and certificates as files
2. **Team Sharing** - Share secrets between team members
3. **Browser Integration** - Auto-fill from stored secrets
4. **Multi-vault Support** - Separate vaults for different contexts
5. **Import/Export** - Migrate from other secret managers
6. **Advanced Templates** - Custom templates for specific services

## 🔒 Security Considerations

- All secrets are encrypted using AES-256-GCM
- Master password never stored, only derived keys
- Memory is automatically cleared after use
- Audit trails for compliance requirements
- Optional expiration dates for key rotation
- Separate backup encryption for additional security

## ✅ Integration Status

- ✅ **CLI Interface** - Complete with all commands
- ✅ **Core Storage** - SQLite with encryption
- ✅ **Memory Safety** - Automatic cleanup
- ✅ **Audit Logging** - Full compliance support
- ✅ **Template System** - Pre-built templates
- ✅ **Search & Filter** - Advanced query capabilities
- 🚧 **GUI Integration** - Pending implementation
- 🚧 **Browser Extension** - Future enhancement

The secrets management system transforms PwGen from a simple password manager into a comprehensive enterprise-grade secrets management solution while maintaining the same security standards and ease of use.