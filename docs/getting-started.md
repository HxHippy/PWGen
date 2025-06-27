# Getting Started with PwGen

Welcome to PwGen! This guide will help you get up and running with your new password and secrets manager.

## 📦 Installation

### Quick Install

Choose your platform:

#### Linux
```bash
curl -sSL https://raw.githubusercontent.com/your-username/pwgen/main/scripts/install.sh | bash
```

#### macOS
```bash
curl -sSL https://raw.githubusercontent.com/your-username/pwgen/main/scripts/install-macos.sh | bash
```

#### Windows (PowerShell as Administrator)
```powershell
irm https://raw.githubusercontent.com/your-username/pwgen/main/scripts/install.ps1 | iex
```

### From Source

If you prefer to build from source, see our [Installation Guide](../README.md#from-source).

## 🚀 First Launch

### Starting PwGen

After installation, launch PwGen:

```bash
# GUI Application
pwgen-gui

# CLI Application (for help)
pwgen-cli --help
```

### Creating Your First Vault

When you first launch PwGen, you'll be prompted to create a new vault:

1. **Master Password**: Choose a strong, memorable master password
   - Use at least 12 characters
   - Include uppercase, lowercase, numbers, and symbols
   - Consider using a passphrase (e.g., "Coffee!Morning$Routine2024")

2. **Confirm Password**: Re-enter your master password

3. **Vault Location**: PwGen will create your vault in the default location:
   - **Linux**: `~/.config/pwgen/vault.db`
   - **macOS**: `~/Library/Application Support/pwgen/vault.db`
   - **Windows**: `%LOCALAPPDATA%\pwgen\vault.db`

⚠️ **Important**: Your master password cannot be recovered if forgotten. Store it securely!

## 🎯 Basic Usage

### Managing Passwords

#### Adding Your First Password

**GUI Method:**
1. Click the "➕ Add Entry" button
2. Fill in the form:
   - **Site**: Website or application name (e.g., "github.com")
   - **Username**: Your username or email
   - **Password**: Use the generator or enter manually
   - **Notes**: Optional additional information
   - **Tags**: Optional tags for organization (e.g., "work", "personal")
3. Click "Save"

**CLI Method:**
```bash
pwgen-cli password add --site github.com --username myuser
# You'll be prompted for the password securely
```

#### Copying Passwords

**GUI Method:**
- Click the 🔑 icon next to any entry to copy the password
- Click the 👤 icon to copy the username

**CLI Method:**
```bash
# Search and copy
pwgen-cli search github
pwgen-cli copy --site github.com
```

#### Generating Secure Passwords

**GUI Method:**
1. Go to the "Generator" tab
2. Adjust settings:
   - **Length**: 8-128 characters
   - **Character types**: Uppercase, lowercase, numbers, symbols
   - **Exclude ambiguous**: Remove similar-looking characters
3. Click "🎲 Generate New Password"
4. Copy the generated password

**CLI Method:**
```bash
# Generate with default settings
pwgen-cli generate

# Custom generation
pwgen-cli generate --length 20 --symbols --no-ambiguous
```

### Managing Secrets

PwGen can store more than just passwords:

#### API Keys
1. Go to the "Secrets" tab
2. Click "➕ Add Secret"
3. Select "API Key" type
4. Fill in provider, key, and other details

#### SSH Keys
1. Select "SSH Key" type
2. Paste your private key (will be encrypted)
3. Add public key and metadata

#### Secure Documents
1. Select "Document" type
2. Upload or paste document content
3. Document will be encrypted and stored

## 🔍 Searching and Organization

### Search Features

**Quick Search:**
- Use the search box to find entries instantly
- Searches across site names, usernames, notes, and tags

**Advanced Search:**
- Filter by favorites only
- Search within specific tags
- Sort by different criteria

### Organization with Tags

Tags help you organize your entries:

**Common Tag Examples:**
- `work` - Work-related accounts
- `personal` - Personal accounts
- `banking` - Financial services
- `social` - Social media accounts
- `dev` - Development tools and services

**Adding Tags:**
- Add tags when creating entries
- Edit existing entries to add tags
- Use comma-separated values: `work, development, github`

## 💾 Backup and Security

### Creating Backups

**GUI Method:**
1. Go to "Tools" tab
2. Click "Create Backup"
3. Choose location and filename
4. Backup will be encrypted with your master password

**CLI Method:**
```bash
pwgen-cli backup create ./my-backup-$(date +%Y%m%d).pwgen
```

### Importing from Other Password Managers

**Browser Import:**
1. Export passwords from your browser (CSV format)
2. Use "Tools" → "Import" in GUI
3. Or use CLI: `pwgen-cli import --browser chrome`

**Other Password Managers:**
- LastPass: Export as CSV
- 1Password: Export as 1PIF
- Bitwarden: Export as JSON
- KeePass: Export as XML

## ⚙️ Configuration

### Customizing PwGen

**GUI Settings:**
- Access via "Settings" menu
- Adjust theme, font size, and interface options

**Configuration File:**
Located at `~/.config/pwgen/config.toml`:

```toml
[ui]
theme = "auto"  # auto, light, dark
font_size = 14
window_size = [1200, 800]

[security]
auto_lock_minutes = 15
clipboard_clear_seconds = 30

[generator]
default_length = 16
default_symbols = true
```

## 🔒 Security Best Practices

### Master Password
- Use a unique password not used elsewhere
- Consider using a passphrase for easier memorization
- Store a backup hint (not the password) in a secure location
- Never share your master password

### General Security
- Keep PwGen updated to the latest version
- Regularly create encrypted backups
- Use unique passwords for each account
- Enable two-factor authentication where available

### Vault Security
- Your vault file is encrypted with AES-256-GCM
- Data is only decrypted in memory when needed
- Memory is securely cleared after use
- No data is sent over the network

## 🆘 Troubleshooting

### Common Issues

**Forgot Master Password:**
- Unfortunately, master passwords cannot be recovered
- You'll need to restore from a backup or start fresh
- This is by design for security

**Vault Won't Open:**
- Check that you're entering the correct master password
- Verify the vault file hasn't been corrupted
- Try restoring from a recent backup

**Performance Issues:**
- Large vaults (1000+ entries) may load slowly
- Consider organizing with tags for better navigation
- Regular cleanup of unused entries helps

**Import Issues:**
- Ensure source file is in the correct format
- Check for special characters that might cause parsing errors
- Try importing in smaller batches

### Getting Help

- 📖 [Full Documentation](../README.md)
- 🐛 [Report Issues](https://github.com/your-username/pwgen/issues)
- 💬 [Community Discussions](https://github.com/your-username/pwgen/discussions)

## 🎓 Next Steps

Now that you're up and running:

1. **Import existing passwords** from browsers or other managers
2. **Set up regular backups** (weekly recommended)
3. **Organize with tags** for easy searching
4. **Generate strong passwords** for new accounts
5. **Explore secrets management** for API keys and certificates

### Advanced Features

Once comfortable with basics:
- **CLI automation** for power users
- **Browser extension** integration (coming soon)
- **Team sharing** features
- **API server** for integrations

---

**Welcome to secure password management with PwGen! 🚀**