---
sidebar_position: 2
---

# First Run

Setting up PwGen for the first time.

## Starting PwGen

After installation, launch PwGen:

```bash
# GUI Application
pwgen-gui

# CLI Application (for help)
pwgen-cli --help
```

## Creating Your First Vault

When you first launch PwGen, you'll be prompted to create a new vault:

### 1. Master Password

Choose a strong, memorable master password:
- Use at least 12 characters
- Include uppercase, lowercase, numbers, and symbols
- Consider using a passphrase (e.g., "Coffee!Morning$Routine2024")

⚠️ **Important**: Your master password cannot be recovered if forgotten. Store it securely!

### 2. Confirm Password

Re-enter your master password to confirm.

### 3. Vault Location

PwGen will create your vault in the default location:
- **Linux**: `~/.config/pwgen/vault.db`
- **macOS**: `~/Library/Application Support/pwgen/vault.db`
- **Windows**: `%LOCALAPPDATA%\pwgen\vault.db`

## Initial Setup

Once your vault is created, you'll see the main interface with these tabs:

- **🔑 Passwords**: Website and application credentials
- **🗝️ Secrets**: API keys, SSH keys, documents
- **🎲 Generator**: Password generation tool
- **🔧 Tools**: Import/export, backup, statistics
- **⚙️ Settings**: Application preferences

## Adding Your First Entry

### GUI Method

1. Go to the **Passwords** tab
2. Click **➕ Add Entry**
3. Fill in:
   - **Site**: Website or application name
   - **Username**: Your username or email
   - **Password**: Generate or enter manually
   - **Tags**: Optional organization tags
4. Click **Save**

### CLI Method

```bash
pwgen-cli password add --site github.com --username myuser
# You'll be prompted for the password securely
```

## Quick Tips

- **Copy Passwords**: Click the 🔑 icon to copy passwords
- **Search**: Use the search box to find entries quickly
- **Tags**: Organize entries with tags like "work", "personal"
- **Favorites**: Star important entries for quick access

## Next Steps

- [Basic Usage](basic-usage) - Learn everyday operations
- [Password Generator](../user-guide/generator) - Create strong passwords
- [Import Data](../user-guide/import-export) - Migrate from other managers