---
sidebar_position: 1
---

# Password Management

Master the art of secure password storage and management with PwGen.

## Overview

PwGen's password management system provides secure, encrypted storage for all your login credentials. Each password entry can include multiple fields and metadata for comprehensive organization.

## Entry Structure

Each password entry contains:

- **Site**: Website URL or application name
- **Username**: Login identifier (email, username, etc.)
- **Password**: The actual password (encrypted)
- **Notes**: Additional information or recovery details
- **Tags**: Organization labels for easy filtering
- **Favorites**: Quick access marking
- **Created/Modified**: Automatic timestamps
- **Password History**: Previous passwords for recovery

## Adding Password Entries

### GUI Method

1. **Navigate to Passwords Tab**
   - Click the 🔑 **Passwords** tab in the main interface

2. **Create New Entry**
   - Click **➕ Add Entry** button
   - Fill in the entry form:
     - **Site**: Enter the website or service name
     - **Username**: Your login username or email
     - **Password**: Use generator or enter manually
     - **Notes**: Optional additional information
     - **Tags**: Comma-separated organization tags

3. **Use Password Generator**
   - Click the **🎲 Generate** button next to password field
   - Adjust generation settings if needed
   - Generator will create a secure password automatically

4. **Save Entry**
   - Click **Save** to encrypt and store the entry
   - Entry will appear in your password list immediately

### CLI Method

```bash
# Interactive password entry
pwgen-cli password add --site github.com --username myuser@example.com

# With password specified
pwgen-cli password add --site github.com --username myuser@example.com --password "SecurePass123!"

# Generate password automatically
pwgen-cli password add --site github.com --username myuser@example.com --generate

# With additional metadata
pwgen-cli password add \
  --site github.com \
  --username myuser@example.com \
  --generate \
  --notes "Personal GitHub account" \
  --tags "dev,personal"
```

## Searching and Filtering

### Quick Search

**GUI Search:**
- Use the search box at the top of the password list
- Search is real-time and searches across all fields
- Results filter automatically as you type

**Search Capabilities:**
- Site names and URLs
- Usernames and email addresses
- Notes and additional information
- Tag names and categories

### Advanced Filtering

**Filter by Tags:**
- Click on any tag button to filter entries
- Multiple tags can be selected for intersection filtering
- Use tag combinations for precise organization

**Filter by Favorites:**
- Click the ⭐ button to show only favorite entries
- Great for frequently accessed accounts

**CLI Search:**
```bash
# Search across all fields
pwgen-cli search "github"

# Search specific field
pwgen-cli search --field site "github.com"
pwgen-cli search --field username "myuser"

# Search with tag filter
pwgen-cli search --tag "work" "server"

# List all entries
pwgen-cli list

# List with specific tag
pwgen-cli list --tag "banking"
```

## Accessing Passwords

### Copying Credentials

**GUI Copy Operations:**
- **🔑 Copy Password**: Click to copy password to clipboard
- **👤 Copy Username**: Click to copy username to clipboard
- **Automatic Clearing**: Clipboard clears after 30 seconds for security

**CLI Copy Operations:**
```bash
# Copy password to clipboard
pwgen-cli copy --site github.com

# Copy username instead
pwgen-cli copy --site github.com --username

# Copy specific entry by ID
pwgen-cli copy --id 123
```

### Viewing Entry Details

**GUI Details:**
- Click on any entry to view full details
- All fields are displayed in a detailed view
- Password history is available if entries were updated

**CLI Details:**
```bash
# Show entry details
pwgen-cli show --site github.com

# Show with password visible
pwgen-cli show --site github.com --show-password

# Show entry by ID
pwgen-cli show --id 123
```

## Editing and Updating

### Modifying Existing Entries

**GUI Editing:**
1. Click on the entry to select it
2. Click the **✏️ Edit** button
3. Modify any fields as needed
4. Previous password is automatically saved to history
5. Click **Save** to apply changes

**CLI Editing:**
```bash
# Update password
pwgen-cli password update --site github.com --password "NewPassword123!"

# Update username
pwgen-cli password update --site github.com --username "newuser@example.com"

# Update multiple fields
pwgen-cli password update \
  --site github.com \
  --username "newuser@example.com" \
  --notes "Updated account credentials" \
  --tags "dev,work"

# Generate new password
pwgen-cli password update --site github.com --generate
```

### Password History

PwGen automatically maintains a history of password changes:

**Accessing History:**
- In GUI: View entry details to see password history
- In CLI: Use `pwgen-cli history --site github.com`

**History Features:**
- Up to 10 previous passwords stored
- Timestamps for each password change
- Useful for sites with password history requirements
- Helps recover from accidental password changes

## Organization with Tags

### Tag System

Tags provide flexible organization for your password entries:

**Common Tag Patterns:**
- **Category Tags**: `work`, `personal`, `banking`, `social`
- **Priority Tags**: `important`, `critical`, `backup`
- **Service Tags**: `dev`, `email`, `cloud`, `gaming`
- **Security Tags**: `2fa`, `recovery`, `shared`

### Managing Tags

**Adding Tags:**
```bash
# Add tags to existing entry
pwgen-cli password update --site github.com --add-tags "important,backup"

# Replace all tags
pwgen-cli password update --site github.com --tags "work,dev,github"
```

**Removing Tags:**
```bash
# Remove specific tags
pwgen-cli password update --site github.com --remove-tags "backup"
```

**Tag Operations:**
```bash
# List all tags in use
pwgen-cli tags list

# Find entries with specific tag
pwgen-cli list --tag "work"

# Multiple tag filtering
pwgen-cli list --tags "work,important"
```

## Favorites System

### Marking Favorites

**GUI Favorites:**
- Click the ⭐ icon on any entry to mark as favorite
- Favorite entries appear at the top of lists
- Use the favorites filter to show only starred entries

**CLI Favorites:**
```bash
# Mark as favorite
pwgen-cli password update --site github.com --favorite

# Remove from favorites
pwgen-cli password update --site github.com --no-favorite

# List only favorites
pwgen-cli list --favorites
```

## Security Features

### Password Strength Analysis

PwGen automatically analyzes password strength:

**Strength Indicators:**
- **Weak**: Short passwords, common patterns
- **Medium**: Adequate length, some complexity
- **Strong**: Long passwords with high entropy
- **Very Strong**: Maximum complexity and length

**Improvement Suggestions:**
- Automatic recommendations for weak passwords
- Integration with password generator for upgrades
- Warnings for reused passwords across entries

### Duplicate Detection

**Duplicate Password Detection:**
- Automatic detection of reused passwords
- Warnings when entering duplicate passwords
- Search for all entries using the same password

**CLI Duplicate Analysis:**
```bash
# Find duplicate passwords
pwgen-cli analyze duplicates

# Find weak passwords
pwgen-cli analyze weak

# Security audit of all passwords
pwgen-cli analyze security
```

## Import and Migration

### Bulk Import

**Supported Formats:**
- CSV (browser exports, spreadsheets)
- JSON (other password managers)
- 1PIF (1Password)
- XML (KeePass)

**Import Process:**
```bash
# Import from CSV
pwgen-cli import csv --file passwords.csv

# Import from browser
pwgen-cli import browser --browser chrome

# Import from other managers
pwgen-cli import keepass --file database.xml
pwgen-cli import bitwarden --file export.json
```

## Best Practices

### Password Creation

1. **Use the Generator**: Always prefer generated passwords over manual ones
2. **Unique Passwords**: Never reuse passwords across different services
3. **Appropriate Length**: Use 16+ characters for high-security accounts
4. **Regular Updates**: Change passwords every 90 days for sensitive accounts

### Organization

1. **Consistent Tagging**: Develop a consistent tag system for organization
2. **Descriptive Names**: Use clear, searchable site names
3. **Detailed Notes**: Include recovery information and account details
4. **Regular Cleanup**: Remove old or unused entries periodically

### Security

1. **Regular Audits**: Use the analysis tools to find weak passwords
2. **Immediate Updates**: Change passwords immediately after security breaches
3. **Favorite Management**: Keep frequently used accounts easily accessible
4. **Backup Regularly**: Ensure password database is backed up securely

## Advanced Features

### Password Policies

Set up password policies for different types of accounts:

```bash
# Set minimum requirements
pwgen-cli policy create --name "banking" --min-length 20 --require-symbols

# Apply policy to entries
pwgen-cli password update --site bank.com --policy "banking"
```

### Automated Workflows

**CLI Automation:**
```bash
# Daily security check
pwgen-cli analyze security --format json > security-report.json

# Batch password updates
pwgen-cli list --tag "old" | xargs -I {} pwgen-cli password update --id {} --generate

# Export specific categories
pwgen-cli export --tag "work" --format csv > work-passwords.csv
```

## Troubleshooting

### Common Issues

**Entry Not Found:**
- Check spelling in search terms
- Try searching by username instead of site
- Verify correct vault is unlocked

**Copy Not Working:**
- Check clipboard permissions
- Try manual selection from entry details
- Restart application if clipboard is stuck

**Slow Performance:**
- Large password databases (1000+ entries) may load slowly
- Use search and filtering to narrow results
- Consider archiving old entries

### Recovery Options

**Password Recovery:**
- Use password history for recently changed passwords
- Check notes for recovery information
- Review email for password reset options

**Entry Recovery:**
- Check backups for accidentally deleted entries
- Use import functionality to restore from backups
- Contact support if database corruption is suspected

## Next Steps

- [Secrets Management](secrets) - Beyond passwords
- [Password Generator](generator) - Advanced generation options
- [Import & Export](import-export) - Data migration tools