---
sidebar_position: 2
---

# Secrets Management

Beyond passwords - manage API keys, SSH keys, certificates, and secure documents.

## Overview

PwGen-rust's secrets management system extends beyond simple password storage to handle various types of sensitive information that modern developers and organizations need to secure:

- **API Keys**: Service tokens, access keys, webhooks
- **SSH Keys**: Private keys, public keys, certificates
- **Documents**: Encrypted file storage, secure notes
- **Environment Variables**: Configuration secrets, connection strings
- **Certificates**: SSL/TLS certificates, signing keys

## Secret Types

### API Keys

Store and manage service tokens, API keys, and access credentials.

**Structure:**
- **Name**: Descriptive identifier
- **Provider**: Service name (AWS, Google, GitHub, etc.)
- **Key Type**: API key, access token, webhook secret
- **Key Value**: The actual secret (encrypted)
- **Expiration**: Optional expiration date
- **Notes**: Usage instructions, scopes, permissions
- **Tags**: Organization labels

**Adding API Keys (GUI):**
1. Go to **🗝️ Secrets** tab
2. Click **➕ Add Secret**
3. Select **API Key** type
4. Fill in the form:
   - **Name**: `GitHub Personal Access Token`
   - **Provider**: `GitHub`
   - **Key Type**: `Personal Access Token`
   - **Key Value**: `ghp_xxxxxxxxxxxxxxxxxxxx`
   - **Expiration**: Set reminder date
   - **Notes**: Scopes and permissions
   - **Tags**: `dev`, `github`, `personal`

**Adding API Keys (CLI):**
```bash
# Interactive API key entry
pwgen-cli secret add api-key --name "GitHub PAT" --provider "GitHub"

# Complete API key entry
pwgen-cli secret add api-key \
  --name "GitHub Personal Access Token" \
  --provider "GitHub" \
  --key-type "Personal Access Token" \
  --key-value "ghp_xxxxxxxxxxxxxxxxxxxx" \
  --expiration "2024-12-31" \
  --notes "Repo access for CI/CD" \
  --tags "dev,github,ci"
```

### SSH Keys

Securely store SSH private keys, public keys, and certificates.

**Structure:**
- **Name**: Key identifier
- **Key Type**: RSA, ED25519, ECDSA
- **Private Key**: Encrypted private key content
- **Public Key**: Public key content
- **Passphrase**: Key passphrase (if any)
- **Certificate**: SSH certificate (if applicable)
- **Hosts**: Associated hostnames
- **Tags**: Organization labels

**Adding SSH Keys (GUI):**
1. Go to **🗝️ Secrets** tab
2. Click **➕ Add Secret**
3. Select **SSH Key** type
4. Fill in the form:
   - **Name**: `Production Server Key`
   - **Key Type**: `ED25519`
   - **Private Key**: Paste private key content
   - **Public Key**: Paste public key content
   - **Hosts**: `prod.example.com`
   - **Tags**: `server`, `production`

**Adding SSH Keys (CLI):**
```bash
# Interactive SSH key entry
pwgen-cli secret add ssh-key --name "Production Server"

# From file
pwgen-cli secret add ssh-key \
  --name "Production Server Key" \
  --private-key-file ~/.ssh/prod_ed25519 \
  --public-key-file ~/.ssh/prod_ed25519.pub \
  --hosts "prod.example.com,backup.example.com" \
  --tags "server,production"

# Generate new SSH key pair
pwgen-cli secret generate ssh-key \
  --name "New Server Key" \
  --key-type "ED25519" \
  --hosts "new-server.example.com"
```

### Secure Documents

Store encrypted documents, notes, and file attachments.

**Structure:**
- **Title**: Document identifier
- **Content Type**: Text, binary, file attachment
- **Content**: Encrypted document content
- **File Name**: Original filename (for attachments)
- **File Type**: MIME type or file extension
- **Size**: File size information
- **Tags**: Organization labels

**Adding Documents (GUI):**
1. Go to **🗝️ Secrets** tab
2. Click **➕ Add Secret**
3. Select **Document** type
4. Choose content type:
   - **Text**: For secure notes, configs
   - **File**: Upload file attachment
5. Fill in details and save

**Adding Documents (CLI):**
```bash
# Text document
pwgen-cli secret add document \
  --title "Server Configuration" \
  --content "Database connection details..." \
  --tags "server,config"

# File attachment
pwgen-cli secret add document \
  --title "SSL Certificate" \
  --file-path ./certificate.pem \
  --tags "ssl,certificate"

# Secure note
pwgen-cli secret add document \
  --title "Recovery Codes" \
  --content-type "text" \
  --content "Recovery codes: 123456, 789012..." \
  --tags "recovery,2fa"
```

### Environment Variables

Store configuration secrets, connection strings, and environment variables.

**Structure:**
- **Name**: Variable name
- **Value**: Variable value (encrypted)
- **Environment**: Development, staging, production
- **Service**: Associated service or application
- **Description**: Usage description
- **Tags**: Organization labels

**Adding Environment Variables (GUI):**
1. Go to **🗝️ Secrets** tab
2. Click **➕ Add Secret**
3. Select **Environment Variable** type
4. Fill in the form:
   - **Name**: `DATABASE_URL`
   - **Value**: `postgres://user:pass@host:port/db`
   - **Environment**: `Production`
   - **Service**: `Web Application`
   - **Description**: Main database connection

**Adding Environment Variables (CLI):**
```bash
# Single environment variable
pwgen-cli secret add env-var \
  --name "DATABASE_URL" \
  --value "postgres://user:pass@host:port/db" \
  --environment "production" \
  --service "web-app" \
  --tags "database,production"

# Batch import from .env file
pwgen-cli secret import env-file \
  --file .env.production \
  --environment "production" \
  --service "web-app"
```

## Managing Secrets

### Viewing Secrets

**GUI Navigation:**
- Go to **🗝️ Secrets** tab
- Browse by type or use filters
- Click on any secret to view details
- Copy functionality for each secret type

**CLI Listing:**
```bash
# List all secrets
pwgen-cli secret list

# Filter by type
pwgen-cli secret list --type "api-key"
pwgen-cli secret list --type "ssh-key"

# Filter by tags
pwgen-cli secret list --tag "production"

# Search secrets
pwgen-cli secret search "github"
```

### Copying Secret Data

**Type-Specific Copy Operations:**

**API Keys:**
- Copy key value to clipboard
- Copy formatted curl command
- Copy environment variable format

**SSH Keys:**
- Copy private key content
- Copy public key content
- Copy SSH config entry

**Documents:**
- Copy document content
- Export file attachment
- Copy formatted text

**Environment Variables:**
- Copy variable value
- Copy as export statement
- Copy formatted for shell

**CLI Copy Operations:**
```bash
# Copy secret value
pwgen-cli secret copy --name "GitHub PAT"

# Copy in specific format
pwgen-cli secret copy --name "Database URL" --format "export"
pwgen-cli secret copy --name "SSH Key" --format "ssh-config"

# Export to file
pwgen-cli secret export --name "SSL Certificate" --output-file cert.pem
```

### Updating Secrets

**GUI Updates:**
1. Select the secret from the list
2. Click **✏️ Edit** button
3. Modify fields as needed
4. Click **Save** to apply changes

**CLI Updates:**
```bash
# Update API key expiration
pwgen-cli secret update --name "GitHub PAT" --expiration "2025-12-31"

# Update SSH key hosts
pwgen-cli secret update --name "Server Key" --hosts "new-host.example.com"

# Rotate API key
pwgen-cli secret update --name "AWS Access Key" --key-value "new-key-value"
```

### Deleting Secrets

**Secure Deletion:**
- GUI: Select secret and click **🗑️ Delete** button
- CLI: `pwgen-cli secret delete --name "Secret Name"`
- Confirmation required for deletion
- Secure memory clearing after deletion

## Secret Templates

### Pre-configured Templates

PwGen includes templates for common services:

**Cloud Providers:**
- AWS (Access Key, Secret Key, Session Token)
- Google Cloud (Service Account, API Key)
- Azure (Client ID, Client Secret, Tenant ID)

**Development Services:**
- GitHub (Personal Access Token, SSH Key)
- Docker (Registry credentials)
- Kubernetes (Service Account, Config)

**Using Templates (GUI):**
1. Click **➕ Add Secret**
2. Select **From Template**
3. Choose service template
4. Fill in template-specific fields

**Using Templates (CLI):**
```bash
# List available templates
pwgen-cli template list

# Create from template
pwgen-cli secret add template --template "aws-iam" \
  --name "Production AWS" \
  --access-key "AKIA..." \
  --secret-key "..." \
  --tags "aws,production"
```

### Custom Templates

**Creating Custom Templates:**
```bash
# Create template for your service
pwgen-cli template create \
  --name "custom-api" \
  --fields "endpoint,api-key,webhook-secret" \
  --description "Custom API service template"

# Use custom template
pwgen-cli secret add template --template "custom-api" \
  --name "My API Service" \
  --endpoint "https://api.example.com" \
  --api-key "..." \
  --webhook-secret "..."
```

## Security Features

### Encryption

All secrets are encrypted using AES-256-GCM:
- Individual encryption for each secret
- Unique encryption keys derived from master password
- No plaintext storage anywhere on disk
- Secure memory handling during processing

### Access Control

**Personal Vaults:**
- Master password protection
- Auto-lock after inactivity
- Secure clipboard handling
- Memory protection

**Team Features (Future):**
- Role-based access control
- Secret sharing permissions
- Audit logging
- Access revocation

### Expiration Management

**Automatic Reminders:**
- Set expiration dates for API keys
- Certificate expiration tracking
- Automated renewal reminders
- Integration with notification systems

**CLI Expiration Management:**
```bash
# List expiring secrets
pwgen-cli secret list --expiring-soon

# Set expiration reminder
pwgen-cli secret update --name "API Key" --expiration "2024-12-31"

# Bulk expiration check
pwgen-cli secret analyze expiration
```

## Integration and Automation

### Command Line Integration

**Environment Setup:**
```bash
# Export secrets as environment variables
eval $(pwgen-cli secret export --tag "development" --format "env")

# Load secrets into shell session
source <(pwgen-cli secret export --service "web-app" --format "shell")
```

**CI/CD Integration:**
```bash
# Get secret for CI pipeline
API_KEY=$(pwgen-cli secret get --name "Deploy Key" --format "raw")

# Inject into application config
pwgen-cli secret export --tag "production" --format "json" > app-secrets.json
```

### Application Integration

**Configuration Management:**
```bash
# Generate application config
pwgen-cli secret export --service "web-app" --format "yaml" > config.yaml

# Docker environment file
pwgen-cli secret export --tag "docker" --format "env" > .env.docker
```

## Best Practices

### Organization

1. **Consistent Naming**: Use clear, descriptive names for all secrets
2. **Tagging Strategy**: Develop consistent tagging for environments and services
3. **Regular Reviews**: Periodically review and clean up unused secrets
4. **Documentation**: Use notes fields to document secret usage and permissions

### Security

1. **Principle of Least Privilege**: Only store secrets that are actually needed
2. **Regular Rotation**: Rotate API keys and certificates regularly
3. **Expiration Dates**: Set expiration reminders for all time-sensitive secrets
4. **Access Logging**: Monitor access patterns for suspicious activity

### Operational

1. **Backup Strategy**: Include secrets in regular backup procedures
2. **Recovery Planning**: Document secret recovery procedures
3. **Team Coordination**: Establish clear procedures for shared secrets
4. **Monitoring**: Set up alerts for expiring certificates and keys

## Advanced Features

### Secret Sharing (Future)

**Team Collaboration:**
- Secure secret sharing between team members
- Role-based access control
- Audit trail for all secret access
- Revocation and access management

### API Integration (Future)

**Programmatic Access:**
- REST API for secret management
- SDK for popular programming languages
- Webhook integration for secret updates
- Third-party service integration

### Compliance Features (Future)

**Enterprise Security:**
- Compliance reporting (SOC 2, ISO 27001)
- Audit logging and monitoring
- Data residency controls
- Integration with enterprise identity systems

## Troubleshooting

### Common Issues

**Import Failures:**
- Verify file format and encoding
- Check for special characters in secret names
- Ensure proper permissions for file access

**Performance Issues:**
- Large secrets (documents) may load slowly
- Use filtering to narrow secret lists
- Consider archiving old secrets

**Copy Operations:**
- Check clipboard permissions
- Verify secret format compatibility
- Try alternative copy formats

### Recovery Scenarios

**Lost API Keys:**
- Check secret history for previous versions
- Review notes for recovery information
- Contact service provider for key regeneration

**Corrupted Files:**
- Restore from recent backups
- Verify backup integrity before restoration
- Contact support for database recovery assistance

## Next Steps

- [Password Generator](generator) - Advanced password generation
- [Import & Export](import-export) - Data migration and backup
- [Security Architecture](../security/architecture) - Understanding encryption