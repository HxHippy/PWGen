---
sidebar_position: 4
---

# Import & Export

Migrate data from other password managers and create portable backups.

## Overview

PwGen supports importing from major password managers and browsers, as well as exporting your data in various formats for backup or migration purposes.

## Importing Data

### Browser Password Import

**Supported Browsers:**
- Google Chrome / Chromium
- Mozilla Firefox  
- Microsoft Edge
- Safari (macOS)
- Opera / Vivaldi

**Chrome Import:**
```bash
# Export from Chrome first:
# 1. Go to chrome://settings/passwords
# 2. Click "Download file" next to "Export passwords"
# 3. Save as CSV file

# Import to PwGen
pwgen-cli import csv --file chrome-passwords.csv --browser chrome
```

**Firefox Import:**
```bash
# Export from Firefox first:
# 1. Go to about:logins
# 2. Click menu (⋯) > "Export Logins"
# 3. Save as CSV file

# Import to PwGen
pwgen-cli import csv --file firefox-passwords.csv --browser firefox
```

### Password Manager Import

**LastPass:**
```bash
# Export from LastPass:
# 1. Log into LastPass web interface
# 2. Go to Advanced Options > Export
# 3. Save as CSV file

# Import to PwGen
pwgen-cli import lastpass --file lastpass_export.csv
```

**1Password:**
```bash
# Export from 1Password:
# 1. Open 1Password app
# 2. File > Export > All Items
# 3. Choose 1PIF format

# Import to PwGen
pwgen-cli import 1password --file 1password_export.1pif
```

**Bitwarden:**
```bash
# Export from Bitwarden:
# 1. Go to bitwarden.com/vault
# 2. Tools > Export Vault
# 3. Choose JSON format

# Import to PwGen
pwgen-cli import bitwarden --file bitwarden_export.json
```

**KeePass:**
```bash
# Export from KeePass:
# 1. File > Export
# 2. Choose KeePass XML (2.x) format

# Import to PwGen  
pwgen-cli import keepass --file keepass_export.xml
```

**Dashlane:**
```bash
# Export from Dashlane:
# 1. Go to Settings > Export Data
# 2. Download as CSV

# Import to PwGen
pwgen-cli import dashlane --file dashlane_export.csv
```

### Generic CSV Import

**CSV Format Requirements:**
```csv
site,username,password,notes,tags
github.com,user@example.com,password123,Development account,"dev,work"
google.com,user@gmail.com,password456,Personal email,"personal,email"
```

**CSV Import Options:**
```bash
# Basic CSV import
pwgen-cli import csv --file passwords.csv

# Custom column mapping
pwgen-cli import csv --file passwords.csv \
  --columns "url:0,username:1,password:2,title:3,notes:4"

# Skip header row
pwgen-cli import csv --file passwords.csv --skip-header

# Specify delimiter
pwgen-cli import csv --file passwords.tsv --delimiter "\t"
```

### GUI Import

**Using the GUI:**
1. Go to **🔧 Tools** tab
2. Click **📥 Import Data**
3. Select import source:
   - Browser (Chrome, Firefox, etc.)
   - Password Manager (LastPass, 1Password, etc.)
   - Generic CSV file
4. Choose the exported file
5. Review import preview
6. Click **Import** to complete

**Import Preview:**
- Shows how many entries will be imported
- Highlights potential duplicates
- Allows field mapping customization
- Provides import conflict resolution options

## Exporting Data

### Export Formats

**CSV Export:**
```bash
# Export all passwords to CSV
pwgen-cli export csv --output passwords.csv

# Export with specific fields
pwgen-cli export csv --output passwords.csv \
  --fields "site,username,password,notes,tags,created"

# Export filtered entries
pwgen-cli export csv --output work-passwords.csv --tag "work"
```

**JSON Export:**
```bash
# Export to JSON (includes all metadata)
pwgen-cli export json --output vault-backup.json

# Pretty-printed JSON
pwgen-cli export json --output vault-backup.json --pretty

# Export secrets separately
pwgen-cli export json --output secrets.json --type secrets
```

**KeePass XML Export:**
```bash
# Export to KeePass-compatible XML
pwgen-cli export keepass --output vault.xml

# Include password history
pwgen-cli export keepass --output vault.xml --include-history
```

**1Password 1PIF Export:**
```bash
# Export to 1Password format
pwgen-cli export 1password --output vault.1pif
```

### Encrypted Export

**Encrypted Backup:**
```bash
# Create encrypted backup (uses master password)
pwgen-cli export encrypted --output vault-backup.pwgen

# Create encrypted backup with different password
pwgen-cli export encrypted --output vault-backup.pwgen --password

# Include all data types
pwgen-cli export encrypted --output full-backup.pwgen --include-secrets
```

### Selective Export

**Tag-Based Export:**
```bash
# Export only work-related entries
pwgen-cli export csv --output work.csv --tag "work"

# Export multiple tags
pwgen-cli export csv --output important.csv --tags "work,banking"

# Exclude specific tags
pwgen-cli export csv --output personal.csv --exclude-tags "work"
```

**Date-Range Export:**
```bash
# Export entries created in last 30 days
pwgen-cli export csv --output recent.csv --since "30 days ago"

# Export entries modified since specific date
pwgen-cli export csv --output updated.csv --modified-since "2024-01-01"
```

## Migration Strategies

### From LastPass

**Complete Migration:**
1. Export from LastPass to CSV
2. Import to PwGen using LastPass format
3. Verify all entries imported correctly
4. Update weak passwords using PwGen generator
5. Organize with tags for better management

**Migration Script:**
```bash
#!/bin/bash
# LastPass to PwGen migration

# Import passwords
pwgen-cli import lastpass --file lastpass_export.csv

# Import secure notes
pwgen-cli import lastpass-notes --file lastpass_notes.csv

# Generate report
pwgen-cli analyze imported --format report > migration-report.txt

# Update weak passwords
pwgen-cli list --weak | while read -r entry; do
  pwgen-cli password update --id "$entry" --generate
done
```

### From 1Password

**Family/Team Migration:**
```bash
# Export from 1Password (each vault separately)
# Import each vault with different tags

pwgen-cli import 1password --file personal.1pif --add-tag "personal"
pwgen-cli import 1password --file work.1pif --add-tag "work"
pwgen-cli import 1password --file shared.1pif --add-tag "shared"
```

### From Browser Storage

**Chrome Migration:**
```bash
# Export Chrome passwords
# chrome://settings/passwords -> Export

# Import with browser-specific handling
pwgen-cli import chrome --file chrome-passwords.csv

# Clean up duplicate entries
pwgen-cli deduplicate --merge-strategy prefer-newest
```

## Duplicate Handling

### Duplicate Detection

**Automatic Detection:**
- Site URL matching
- Username matching
- Exact password matching
- Similar entry detection

**Detection Options:**
```bash
# Find duplicates
pwgen-cli duplicates list

# Find duplicates by site
pwgen-cli duplicates list --field site

# Find weak duplicates (same password, different sites)
pwgen-cli duplicates list --cross-site
```

### Duplicate Resolution

**Merge Strategies:**
```bash
# Keep newest entry
pwgen-cli duplicates merge --strategy newest

# Keep entry with most metadata
pwgen-cli duplicates merge --strategy complete

# Manual resolution
pwgen-cli duplicates resolve --interactive

# Custom merge rules
pwgen-cli duplicates merge --rules merge-rules.json
```

**Merge Rules Configuration:**
```json
{
  "site_matching": "exact",
  "username_matching": "case_insensitive",
  "password_matching": "exact",
  "merge_strategy": "prefer_newest",
  "preserve_tags": true,
  "merge_notes": true,
  "keep_history": true
}
```

## Data Validation

### Import Validation

**Pre-Import Checks:**
```bash
# Validate CSV format before import
pwgen-cli validate csv --file passwords.csv

# Check for common issues
pwgen-cli validate csv --file passwords.csv --report issues.txt

# Preview import without executing
pwgen-cli import csv --file passwords.csv --dry-run
```

**Common Issues:**
- Invalid CSV format or encoding
- Missing required fields
- Password field empty or malformed
- Special characters in site names
- Extremely long field values

### Post-Import Validation

**Verification Steps:**
```bash
# Count imported entries
pwgen-cli list --count

# Check for import errors
pwgen-cli import-log --last-import

# Verify specific entries
pwgen-cli search "github.com" | head -5

# Generate import report
pwgen-cli analyze import --report import-summary.txt
```

## Automation and Scripting

### Batch Import Scripts

**Multi-Source Import:**
```bash
#!/bin/bash
# Import from multiple sources

# Import browser passwords
for browser in chrome firefox edge; do
  if [ -f "${browser}-passwords.csv" ]; then
    pwgen-cli import csv --file "${browser}-passwords.csv" \
      --browser "$browser" --add-tag "browser-$browser"
  fi
done

# Import password managers
for manager in lastpass bitwarden; do
  if [ -f "${manager}-export.csv" ]; then
    pwgen-cli import "$manager" --file "${manager}-export.csv" \
      --add-tag "migrated-$manager"
  fi
done

# Generate migration report
pwgen-cli analyze migration --output migration-report.html
```

### Export Automation

**Scheduled Backups:**
```bash
#!/bin/bash
# Daily backup script

DATE=$(date +%Y%m%d)
BACKUP_DIR="$HOME/pwgen-backups"

mkdir -p "$BACKUP_DIR"

# Create encrypted backup
pwgen-cli export encrypted --output "$BACKUP_DIR/vault-$DATE.pwgen"

# Create CSV backup for external tools
pwgen-cli export csv --output "$BACKUP_DIR/passwords-$DATE.csv"

# Clean old backups (keep 30 days)
find "$BACKUP_DIR" -name "vault-*.pwgen" -mtime +30 -delete
find "$BACKUP_DIR" -name "passwords-*.csv" -mtime +30 -delete
```

## Security Considerations

### Import Security

**Secure Import Practices:**
1. Verify source file integrity
2. Import from trusted locations only
3. Delete source files after import
4. Review imported entries for anomalies
5. Update weak passwords after import

**File Handling:**
```bash
# Secure file operations
umask 077  # Restrict file permissions

# Import with secure file handling
pwgen-cli import csv --file passwords.csv --secure-delete

# Verify file deletion
shred -vfz passwords.csv
```

### Export Security

**Secure Export Practices:**
1. Use encrypted formats when possible
2. Protect export files with strong passwords
3. Store exports in secure locations
4. Delete exports after use
5. Never export to untrusted systems

**Encrypted Export Options:**
```bash
# Export with additional encryption
pwgen-cli export encrypted --output backup.pwgen --encrypt-with-key

# Export to encrypted USB device
pwgen-cli export encrypted --output /media/encrypted-usb/backup.pwgen

# Create password-protected ZIP
pwgen-cli export csv --output passwords.csv
zip -e backup.zip passwords.csv
shred -vfz passwords.csv
```

## Troubleshooting

### Import Issues

**Common Problems:**
- File encoding issues (use UTF-8)
- CSV delimiter problems
- Missing or extra columns
- Password field truncation
- Special character handling

**Solutions:**
```bash
# Fix encoding issues
iconv -f ISO-8859-1 -t UTF-8 passwords.csv > passwords-utf8.csv

# Adjust CSV settings
pwgen-cli import csv --file passwords.csv \
  --delimiter ";" --quote-char '"' --escape-char '\\'

# Handle large files
split -l 1000 large-passwords.csv chunk-
for chunk in chunk-*; do
  pwgen-cli import csv --file "$chunk"
done
```

### Export Issues

**Common Problems:**
- File permission errors
- Disk space limitations
- Large export sizes
- Format compatibility issues

**Solutions:**
```bash
# Check available space
df -h /path/to/export

# Export in chunks
pwgen-cli export csv --output chunk1.csv --limit 1000 --offset 0
pwgen-cli export csv --output chunk2.csv --limit 1000 --offset 1000

# Compress large exports
pwgen-cli export json --output vault.json
gzip vault.json
```

## Best Practices

### Migration Planning

1. **Inventory Current Data**: List all sources and data types
2. **Plan Import Order**: Start with most important data
3. **Test with Sample**: Import small subset first
4. **Verify Accuracy**: Check random sampling of imported data
5. **Update Security**: Change weak passwords after import

### Backup Strategy

1. **Regular Exports**: Schedule automated backups
2. **Multiple Formats**: Export in different formats
3. **Secure Storage**: Use encrypted backup locations
4. **Test Restores**: Verify backups can be restored
5. **Offsite Storage**: Keep copies in separate locations

### Data Hygiene

1. **Clean Before Export**: Remove unused entries
2. **Standardize Tags**: Use consistent tagging
3. **Update Metadata**: Ensure notes and URLs are current
4. **Resolve Duplicates**: Clean up duplicate entries
5. **Validate Passwords**: Check for weak or expired passwords

## Next Steps

- [Backup & Restore](backup-restore) - Comprehensive backup strategies
- [Security Architecture](../security/architecture) - Understanding data protection
- [CLI Reference](../cli/commands) - Advanced command-line operations