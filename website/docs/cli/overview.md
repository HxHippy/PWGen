---
sidebar_position: 1
---

# CLI Overview

Command-line interface for power users and automation.

## Introduction

The PwGen CLI provides full access to password and secrets management through a powerful command-line interface. Perfect for automation, scripting, and power users who prefer terminal-based workflows.

## Installation

The CLI is included with all PwGen installations:

```bash
# Verify installation
pwgen-cli --version

# Get help
pwgen-cli --help

# Command-specific help
pwgen-cli password --help
```

## Basic Usage

### Global Options

```bash
# Specify vault location
pwgen-cli --vault /path/to/vault.db password list

# Set output format
pwgen-cli --format json password list

# Verbose output
pwgen-cli --verbose password add --site example.com

# Quiet mode (errors only)
pwgen-cli --quiet password add --site example.com
```

### Vault Management

```bash
# Create new vault
pwgen-cli vault create --path /path/to/new-vault.db

# Change master password
pwgen-cli vault change-password

# Lock vault
pwgen-cli vault lock

# Unlock vault
pwgen-cli vault unlock

# Vault status
pwgen-cli vault status
```

## Command Structure

### Password Commands

```bash
# Add password entry
pwgen-cli password add --site github.com --username user

# List passwords
pwgen-cli password list

# Search passwords
pwgen-cli password search "github"

# Update password
pwgen-cli password update --site github.com --password "newpass"

# Delete password
pwgen-cli password delete --site github.com

# Copy password to clipboard
pwgen-cli password copy --site github.com
```

### Secret Commands

```bash
# Add API key
pwgen-cli secret add api-key --name "GitHub Token" --provider "GitHub"

# List secrets
pwgen-cli secret list

# Add SSH key
pwgen-cli secret add ssh-key --name "Server Key" --key-file ~/.ssh/id_rsa

# Add document
pwgen-cli secret add document --title "Certificate" --file cert.pem

# Copy secret value
pwgen-cli secret copy --name "GitHub Token"
```

### Generator Commands

```bash
# Generate password
pwgen-cli generate

# Generate with options
pwgen-cli generate --length 20 --symbols --no-ambiguous

# Generate multiple passwords
pwgen-cli generate --count 5

# Generate with pattern
pwgen-cli generate --pattern "Llll-dddd-LLLL"
```

### Backup Commands

```bash
# Create backup
pwgen-cli backup create --output backup.pwgen

# Restore from backup
pwgen-cli restore --file backup.pwgen

# Verify backup
pwgen-cli backup verify --file backup.pwgen

# List backup contents
pwgen-cli backup list --file backup.pwgen
```

### Import/Export Commands

```bash
# Import from CSV
pwgen-cli import csv --file passwords.csv

# Import from LastPass
pwgen-cli import lastpass --file export.csv

# Export to CSV
pwgen-cli export csv --output passwords.csv

# Export to JSON
pwgen-cli export json --output vault.json
```

## Output Formats

### Text Format (Default)

```bash
pwgen-cli password list
# Output:
# Site: github.com
# Username: user@example.com
# Tags: dev, work
# Created: 2024-01-15 10:30:00
# 
# Site: google.com
# Username: user@gmail.com
# Tags: personal
# Created: 2024-01-16 09:15:00
```

### JSON Format

```bash
pwgen-cli --format json password list
# Output:
# [
#   {
#     "id": 1,
#     "site": "github.com",
#     "username": "user@example.com",
#     "tags": ["dev", "work"],
#     "created": "2024-01-15T10:30:00Z"
#   }
# ]
```

### CSV Format

```bash
pwgen-cli --format csv password list
# Output:
# id,site,username,tags,created
# 1,github.com,user@example.com,"dev,work",2024-01-15T10:30:00Z
# 2,google.com,user@gmail.com,personal,2024-01-16T09:15:00Z
```

### Table Format

```bash
pwgen-cli --format table password list
# Output:
# ┌────┬────────────┬──────────────────┬──────────┬─────────────────────┐
# │ ID │ Site       │ Username         │ Tags     │ Created             │
# ├────┼────────────┼──────────────────┼──────────┼─────────────────────┤
# │ 1  │ github.com │ user@example.com │ dev,work │ 2024-01-15 10:30:00 │
# │ 2  │ google.com │ user@gmail.com   │ personal │ 2024-01-16 09:15:00 │
# └────┴────────────┴──────────────────┴──────────┴─────────────────────┘
```

## Configuration

### Configuration File

Default location: `~/.config/pwgen/config.toml`

```toml
[cli]
default_format = "table"
default_vault = "~/.config/pwgen/vault.db"
editor = "vim"
pager = "less"

[output]
colors = true
unicode = true
timestamp_format = "%Y-%m-%d %H:%M:%S"

[clipboard]
clear_after_seconds = 30
use_primary_selection = false

[security]
lock_after_minutes = 15
require_confirmation = true
```

### Environment Variables

```bash
# Vault location
export PWGEN_VAULT_PATH="/path/to/vault.db"

# Output format
export PWGEN_FORMAT="json"

# Disable colors
export PWGEN_NO_COLOR=1

# Editor for interactive commands
export PWGEN_EDITOR="nano"

# Pager for long output
export PWGEN_PAGER="bat"
```

## Interactive Mode

### Secure Input

```bash
# Password prompts (hidden input)
pwgen-cli password add --site github.com --username user
# Password: [hidden input]

# Master password prompt
pwgen-cli vault unlock
# Master password: [hidden input]
```

### Confirmation Prompts

```bash
# Delete confirmation
pwgen-cli password delete --site github.com
# Are you sure you want to delete this entry? [y/N]: y

# Batch operations
pwgen-cli password delete --tag "old" --confirm
# This will delete 5 entries. Continue? [y/N]: y
```

### Interactive Editors

```bash
# Edit notes in external editor
pwgen-cli password update --site github.com --edit-notes

# Edit secret content
pwgen-cli secret update --name "Config" --edit-content
```

## Scripting and Automation

### Exit Codes

- `0`: Success
- `1`: General error
- `2`: Command not found
- `3`: Invalid arguments
- `4`: Vault not found or locked
- `5`: Entry not found
- `6`: Authentication failed
- `7`: Permission denied

### Script Examples

**Backup Script:**
```bash
#!/bin/bash
set -e

# Create dated backup
BACKUP_FILE="vault-backup-$(date +%Y%m%d).pwgen"

if pwgen-cli backup create --output "$BACKUP_FILE"; then
    echo "Backup created: $BACKUP_FILE"
    exit 0
else
    echo "Backup failed"
    exit 1
fi
```

**Password Generation Script:**
```bash
#!/bin/bash

# Generate passwords for new accounts
SITES=("github.com" "gitlab.com" "bitbucket.org")

for site in "${SITES[@]}"; do
    PASSWORD=$(pwgen-cli generate --length 16)
    pwgen-cli password add --site "$site" --username "admin" --password "$PASSWORD"
    echo "Created entry for $site"
done
```

**Security Audit Script:**
```bash
#!/bin/bash

# Find weak passwords
echo "=== Weak Passwords ==="
pwgen-cli analyze weak --format table

# Find duplicate passwords
echo "=== Duplicate Passwords ==="
pwgen-cli analyze duplicates --format table

# Find old passwords
echo "=== Old Passwords (>90 days) ==="
pwgen-cli password list --older-than "90 days" --format table
```

## Advanced Features

### Shell Completions

**Bash Completion:**
```bash
# Install completion
pwgen-cli completion bash > /etc/bash_completion.d/pwgen-cli

# Or for user only
pwgen-cli completion bash > ~/.bash_completion.d/pwgen-cli
```

**Zsh Completion:**
```bash
# Install completion
pwgen-cli completion zsh > ~/.zsh/completions/_pwgen-cli

# Add to .zshrc
fpath=(~/.zsh/completions $fpath)
autoload -U compinit
compinit
```

**Fish Completion:**
```bash
# Install completion
pwgen-cli completion fish > ~/.config/fish/completions/pwgen-cli.fish
```

### Aliases and Functions

**Useful Aliases:**
```bash
# Common operations
alias pw='pwgen-cli password'
alias pwl='pwgen-cli password list'
alias pws='pwgen-cli password search'
alias pwg='pwgen-cli generate'
alias pwc='pwgen-cli password copy'

# Backup operations
alias pwb='pwgen-cli backup create --output "backup-$(date +%Y%m%d).pwgen"'
alias pwr='pwgen-cli restore'
```

**Shell Functions:**
```bash
# Quick password lookup and copy
pwfind() {
    local site="$1"
    pwgen-cli password search "$site" --format table
    read -p "Copy password for which entry? (ID): " id
    pwgen-cli password copy --id "$id"
}

# Generate and add password
pwadd() {
    local site="$1"
    local username="$2"
    local password=$(pwgen-cli generate --length 16)
    pwgen-cli password add --site "$site" --username "$username" --password "$password"
    echo "Added entry for $site with generated password"
}
```

### Pipes and Filters

**Using with Standard Tools:**
```bash
# Count entries
pwgen-cli password list --format csv | wc -l

# Find entries with specific pattern
pwgen-cli password list --format csv | grep -i "google"

# Sort by site
pwgen-cli password list --format csv | sort -k2

# Extract unique tags
pwgen-cli password list --format json | jq -r '.[].tags[]' | sort | uniq

# Find entries without tags
pwgen-cli password list --format json | jq '.[] | select(.tags | length == 0)'
```

## Integration Examples

### Git Hooks

**Pre-commit Hook:**
```bash
#!/bin/bash
# Check for exposed passwords in commits

# Get all changed files
changed_files=$(git diff --cached --name-only)

for file in $changed_files; do
    # Check for potential passwords
    if grep -q "password\|secret\|key" "$file"; then
        echo "Warning: Potential secrets in $file"
        echo "Verify no real passwords are committed"
    fi
done
```

### CI/CD Integration

**GitHub Actions:**
```yaml
name: Deploy
on: [push]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install PwGen
        run: |
          curl -sSL https://raw.githubusercontent.com/hxhippy/pwgen/main/scripts/install.sh | bash
      - name: Get deployment credentials
        run: |
          # Assuming vault is available in CI environment
          API_KEY=$(pwgen-cli secret get --name "Deploy API Key" --format raw)
          echo "::add-mask::$API_KEY"
          echo "API_KEY=$API_KEY" >> $GITHUB_ENV
```

### System Administration

**User Onboarding Script:**
```bash
#!/bin/bash
# Create accounts for new user

USERNAME="$1"
EMAIL="$2"

if [ -z "$USERNAME" ] || [ -z "$EMAIL" ]; then
    echo "Usage: $0 <username> <email>"
    exit 1
fi

# Generate passwords for various systems
SYSTEMS=("gitlab" "jira" "confluence" "vpn")

for system in "${SYSTEMS[@]}"; do
    password=$(pwgen-cli generate --length 12)
    pwgen-cli password add \
        --site "$system.company.com" \
        --username "$USERNAME" \
        --password "$password" \
        --tags "onboarding,$system" \
        --notes "Created for $EMAIL on $(date)"
    
    echo "Created $system account for $USERNAME"
done

echo "Onboarding complete. Passwords stored in vault."
```

## Next Steps

- [Commands Reference](commands) - Complete command documentation
- [Configuration](configuration) - Advanced configuration options
- [User Guide](../user-guide/passwords) - GUI and CLI integration