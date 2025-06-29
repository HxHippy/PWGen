---
sidebar_position: 3
---

# Basic Usage

Learn the essential operations for managing your passwords and secrets.

## Managing Passwords

### Adding Entries

**GUI Method:**
1. Click **➕ Add Entry** in the Passwords tab
2. Fill in the required fields
3. Use the password generator for secure passwords
4. Add tags for organization
5. Click **Save**

**CLI Method:**
```bash
# Add with interactive prompt
pwgen-cli password add --site example.com --username user@example.com

# Add with password
pwgen-cli password add --site example.com --username user@example.com --password "SecurePass123!"

# Add with generated password
pwgen-cli password add --site example.com --username user@example.com --generate
```

### Searching Entries

**Quick Search:**
- Use the search box at the top
- Searches across all fields
- Real-time filtering as you type

**Advanced Search:**
- Click 🔧 for advanced options
- Filter by specific fields
- Search within tags or notes

**CLI Search:**
```bash
# Search all fields
pwgen-cli search "github"

# Search specific field
pwgen-cli search --field site "github"

# List all entries
pwgen-cli list
```

### Copying Credentials

**GUI:**
- 🔑 Copy password
- 👤 Copy username
- Automatic clipboard clearing after 30 seconds

**CLI:**
```bash
# Copy password
pwgen-cli copy --site github.com

# Copy username
pwgen-cli copy --site github.com --username
```

### Editing Entries

**GUI:**
1. Click on any entry
2. Click ✏️ Edit
3. Modify fields
4. Click **Save**

**CLI:**
```bash
# Update password
pwgen-cli password update --site github.com --password "NewPassword123!"

# Update username
pwgen-cli password update --site github.com --username new@email.com
```

## Password Generation

### GUI Generator

1. Go to the **Generator** tab
2. Configure:
   - **Length**: 8-128 characters
   - **Character types**: Upper, lower, numbers, symbols
   - **Exclude ambiguous**: Remove similar characters
3. Click **🎲 Generate**
4. Copy or use directly

### CLI Generator

```bash
# Default generation (16 chars)
pwgen-cli generate

# Custom length
pwgen-cli generate --length 24

# With specific requirements
pwgen-cli generate --length 20 --symbols --no-ambiguous

# Multiple passwords
pwgen-cli generate --count 5
```

## Organization with Tags

### Using Tags

Tags help organize your entries:

**Common Tags:**
- `work` - Professional accounts
- `personal` - Personal accounts
- `banking` - Financial services
- `social` - Social media
- `dev` - Development tools

**Adding Tags:**
```bash
# GUI: Add comma-separated in tag field
# CLI: Use --tags flag
pwgen-cli password add --site example.com --tags "work,important"
```

### Filtering by Tags

**GUI:**
- Use tag buttons for quick filtering
- Combine with search for precision

**CLI:**
```bash
# Filter by tag
pwgen-cli list --tag work

# Multiple tags
pwgen-cli list --tags "work,banking"
```

## Security Features

### Auto-Lock

PwGen-rust automatically locks after:
- 15 minutes of inactivity (default)
- Manual lock via File → Lock Vault
- Configurable in Settings

### Secure Notes

Store sensitive text securely:

```bash
# Add secure note
pwgen-cli note add --title "Server Config" --content "Sensitive configuration..."

# View notes
pwgen-cli note list
```

### Password History

- PwGen-rust tracks password changes
- View previous passwords for recovery
- Useful for sites with password history requirements
- All historical passwords remain encrypted

## Keyboard Shortcuts

### Global Shortcuts

- `Ctrl/Cmd + L` - Lock vault
- `Ctrl/Cmd + F` - Focus search
- `Ctrl/Cmd + N` - New entry
- `Ctrl/Cmd + G` - Generate password

### Entry Management

- `Enter` - Open selected entry
- `Ctrl/Cmd + C` - Copy password
- `Ctrl/Cmd + Shift + C` - Copy username
- `Delete` - Delete entry (with confirmation)

## Best Practices

### Strong Master Password

- Use a passphrase you'll remember
- Never reuse from other services
- Consider: `correct-horse-battery-staple`
- Write down hint (not password) securely

### Regular Backups

```bash
# Create backup
pwgen-cli backup create

# With custom location
pwgen-cli backup create --output ~/backups/pwgen-backup.enc
```

### Update Passwords Regularly

- Change passwords every 90 days for sensitive accounts
- Use the generator for each new password
- Update immediately after breaches

## Troubleshooting

### Common Issues

**Can't find entry:**
- Check spelling in search
- Try searching by username
- Verify correct vault is unlocked

**Copy not working:**
- Check clipboard permissions
- Try manual selection
- Restart application

**Performance issues:**
- Large vaults may load slowly
- Use pagination for better performance
- Consider archiving old entries

## Next Steps

- [Advanced Features](../user-guide/passwords) - Power user features
- [Secrets Management](../user-guide/secrets) - Beyond passwords
- [Import/Export](../user-guide/import-export) - Data migration