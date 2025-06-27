---
sidebar_position: 5
---

# Backup & Restore

Protect your password vault with comprehensive backup and recovery strategies.

## Overview

Regular backups are essential for protecting your password vault against data loss, corruption, or hardware failures. PwGen provides multiple backup options with strong encryption and verification features.

## Backup Types

### Full Vault Backup

**Complete Backup:**
- Includes all passwords, secrets, and metadata
- Preserves tags, notes, and creation dates
- Maintains password history
- Encrypted with master password

**Creating Full Backups:**

**GUI Method:**
1. Go to **🔧 Tools** tab
2. Click **💾 Create Backup**
3. Choose backup location and filename
4. Select backup options:
   - Include password history
   - Include secrets
   - Compression level
5. Click **Create Backup**

**CLI Method:**
```bash
# Basic full backup
pwgen-cli backup create --output ~/backups/vault-$(date +%Y%m%d).pwgen

# Backup with custom encryption
pwgen-cli backup create --output vault-backup.pwgen --encrypt-password

# Include all data types
pwgen-cli backup create --output complete-backup.pwgen \
  --include-passwords --include-secrets --include-history
```

### Incremental Backup

**Changed Data Only:**
- Backs up only modified entries since last backup
- Faster backup process for large vaults
- Requires base backup for complete restoration

**Creating Incremental Backups:**
```bash
# Create base backup first
pwgen-cli backup create --output base-backup.pwgen --full

# Create incremental backups
pwgen-cli backup create --output incremental-$(date +%Y%m%d).pwgen \
  --incremental --since-backup base-backup.pwgen

# Daily incremental backup script
pwgen-cli backup create --output "incremental-$(date +%Y%m%d).pwgen" \
  --incremental --since "yesterday"
```

### Selective Backup

**Tag-Based Backup:**
```bash
# Backup work-related entries only
pwgen-cli backup create --output work-backup.pwgen --tag "work"

# Backup multiple categories
pwgen-cli backup create --output important-backup.pwgen \
  --tags "work,banking,critical"

# Exclude specific categories
pwgen-cli backup create --output personal-backup.pwgen \
  --exclude-tags "work,temporary"
```

**Type-Specific Backup:**
```bash
# Backup only passwords
pwgen-cli backup create --output passwords-only.pwgen --passwords-only

# Backup only secrets
pwgen-cli backup create --output secrets-only.pwgen --secrets-only

# Backup specific secret types
pwgen-cli backup create --output api-keys.pwgen --secret-types "api-key,env-var"
```

## Backup Formats

### Native PwGen Format (.pwgen)

**Encrypted Archive:**
- Native PwGen format with full fidelity
- Preserves all metadata and relationships
- Encrypted with AES-256-GCM
- Includes integrity verification

**Advantages:**
- Complete data preservation
- Fast restore process
- Built-in encryption
- Integrity checking

### Portable Formats

**JSON Backup:**
```bash
# JSON backup (human-readable)
pwgen-cli backup create --output vault.json --format json

# Encrypted JSON backup
pwgen-cli backup create --output vault.json.enc --format json --encrypt
```

**CSV Backup:**
```bash
# CSV backup (compatible with spreadsheets)
pwgen-cli backup create --output vault.csv --format csv

# CSV with custom fields
pwgen-cli backup create --output vault.csv --format csv \
  --fields "site,username,password,notes,tags,created,modified"
```

**XML Backup:**
```bash
# KeePass-compatible XML
pwgen-cli backup create --output vault.xml --format keepass-xml
```

## Backup Security

### Encryption Options

**Master Password Encryption:**
```bash
# Use current master password (default)
pwgen-cli backup create --output backup.pwgen

# Prompt for different password
pwgen-cli backup create --output backup.pwgen --encrypt-password

# Use keyfile
pwgen-cli backup create --output backup.pwgen --keyfile backup.key
```

**Additional Encryption:**
```bash
# Encrypt with GPG
pwgen-cli backup create --output backup.pwgen
gpg --encrypt --recipient "user@example.com" backup.pwgen

# Encrypt with age
pwgen-cli backup create --output backup.pwgen
age -r age1public_key_here backup.pwgen > backup.pwgen.age
```

### Backup Verification

**Integrity Checking:**
```bash
# Verify backup integrity
pwgen-cli backup verify --file backup.pwgen

# Detailed verification report
pwgen-cli backup verify --file backup.pwgen --detailed

# Test restore without applying changes
pwgen-cli backup verify --file backup.pwgen --test-restore
```

**Automated Verification:**
```bash
#!/bin/bash
# Backup verification script

BACKUP_FILE="vault-backup.pwgen"

# Create backup
pwgen-cli backup create --output "$BACKUP_FILE"

# Verify integrity
if pwgen-cli backup verify --file "$BACKUP_FILE"; then
  echo "Backup verified successfully"
else
  echo "Backup verification failed!"
  exit 1
fi

# Test restore process
if pwgen-cli backup verify --file "$BACKUP_FILE" --test-restore; then
  echo "Restore test passed"
else
  echo "Restore test failed!"
  exit 1
fi
```

## Backup Storage

### Local Storage

**File System Backup:**
```bash
# Local directory backup
BACKUP_DIR="$HOME/pwgen-backups"
mkdir -p "$BACKUP_DIR"
pwgen-cli backup create --output "$BACKUP_DIR/vault-$(date +%Y%m%d).pwgen"

# External drive backup
pwgen-cli backup create --output "/media/backup-drive/pwgen-backup.pwgen"

# Network attached storage
pwgen-cli backup create --output "/mnt/nas/backups/pwgen-backup.pwgen"
```

### Cloud Storage

**Cloud Backup Script:**
```bash
#!/bin/bash
# Cloud backup automation

BACKUP_FILE="vault-$(date +%Y%m%d).pwgen"

# Create encrypted backup
pwgen-cli backup create --output "$BACKUP_FILE"

# Upload to different cloud providers
# AWS S3
aws s3 cp "$BACKUP_FILE" "s3://my-backup-bucket/pwgen/"

# Google Cloud Storage
gsutil cp "$BACKUP_FILE" "gs://my-backup-bucket/pwgen/"

# Dropbox (using dropbox_uploader)
./dropbox_uploader.sh upload "$BACKUP_FILE" "/Apps/PwGen/"

# Clean local backup
rm "$BACKUP_FILE"
```

**Encrypted Cloud Storage:**
```bash
# Encrypt before cloud upload
pwgen-cli backup create --output backup.pwgen
gpg --encrypt --recipient "backup@example.com" backup.pwgen

# Upload encrypted file
rclone copy backup.pwgen.gpg remote:backups/pwgen/
rm backup.pwgen backup.pwgen.gpg
```

## Automated Backup

### Scheduled Backups

**Cron Job Setup:**
```bash
# Edit crontab
crontab -e

# Daily backup at 2 AM
0 2 * * * /usr/local/bin/pwgen-cli backup create --output "$HOME/backups/daily-$(date +\%Y\%m\%d).pwgen"

# Weekly full backup on Sunday at 3 AM
0 3 * * 0 /usr/local/bin/pwgen-cli backup create --output "$HOME/backups/weekly-$(date +\%Y\%m\%d).pwgen" --full

# Monthly archive on 1st of month
0 4 1 * * /usr/local/bin/pwgen-cli backup create --output "$HOME/archives/monthly-$(date +\%Y\%m).pwgen"
```

**Windows Task Scheduler:**
```powershell
# PowerShell script for Windows
$BackupDir = "$env:USERPROFILE\PwGen\Backups"
$BackupFile = "vault-$(Get-Date -Format 'yyyyMMdd').pwgen"

New-Item -ItemType Directory -Force -Path $BackupDir
& pwgen-cli backup create --output "$BackupDir\$BackupFile"

# Clean old backups (keep 30 days)
Get-ChildItem "$BackupDir\vault-*.pwgen" | Where-Object {$_.LastWriteTime -lt (Get-Date).AddDays(-30)} | Remove-Item
```

### Backup Rotation

**Rotation Strategy:**
```bash
#!/bin/bash
# Backup rotation script

BACKUP_DIR="$HOME/pwgen-backups"
DATE=$(date +%Y%m%d)

# Create today's backup
pwgen-cli backup create --output "$BACKUP_DIR/daily-$DATE.pwgen"

# Weekly backup (Sunday)
if [ $(date +%u) -eq 7 ]; then
  cp "$BACKUP_DIR/daily-$DATE.pwgen" "$BACKUP_DIR/weekly-$DATE.pwgen"
fi

# Monthly backup (1st of month)
if [ $(date +%d) -eq 01 ]; then
  cp "$BACKUP_DIR/daily-$DATE.pwgen" "$BACKUP_DIR/monthly-$DATE.pwgen"
fi

# Cleanup old backups
# Keep daily backups for 7 days
find "$BACKUP_DIR" -name "daily-*.pwgen" -mtime +7 -delete

# Keep weekly backups for 4 weeks
find "$BACKUP_DIR" -name "weekly-*.pwgen" -mtime +28 -delete

# Keep monthly backups for 12 months
find "$BACKUP_DIR" -name "monthly-*.pwgen" -mtime +365 -delete
```

## Restore Process

### Full Restore

**Complete Vault Restoration:**

**GUI Restore:**
1. Go to **🔧 Tools** tab
2. Click **📁 Restore from Backup**
3. Select backup file
4. Choose restore options:
   - Replace current vault
   - Merge with current vault
   - Restore to new vault
5. Enter backup password if different
6. Click **Restore**

**CLI Restore:**
```bash
# Replace current vault
pwgen-cli restore --file backup.pwgen --replace

# Merge with current vault
pwgen-cli restore --file backup.pwgen --merge

# Restore to new vault file
pwgen-cli restore --file backup.pwgen --output new-vault.db

# Restore with conflict resolution
pwgen-cli restore --file backup.pwgen --merge --conflict-resolution newest
```

### Selective Restore

**Partial Restoration:**
```bash
# Restore only passwords
pwgen-cli restore --file backup.pwgen --passwords-only

# Restore specific tags
pwgen-cli restore --file backup.pwgen --tags "work,banking"

# Restore entries modified after specific date
pwgen-cli restore --file backup.pwgen --modified-since "2024-01-01"

# Restore deleted entries only
pwgen-cli restore --file backup.pwgen --deleted-only
```

### Conflict Resolution

**Merge Strategies:**
```bash
# Keep newer entries
pwgen-cli restore --file backup.pwgen --merge --conflict-resolution newer

# Keep backup entries
pwgen-cli restore --file backup.pwgen --merge --conflict-resolution backup

# Keep current entries
pwgen-cli restore --file backup.pwgen --merge --conflict-resolution current

# Interactive conflict resolution
pwgen-cli restore --file backup.pwgen --merge --conflict-resolution interactive
```

**Custom Merge Rules:**
```json
{
  "duplicate_handling": "merge",
  "password_conflicts": "prefer_newer",
  "tag_conflicts": "merge_all",
  "note_conflicts": "append",
  "metadata_conflicts": "prefer_backup"
}
```

## Disaster Recovery

### Recovery Scenarios

**Complete System Loss:**
1. Install PwGen on new system
2. Restore from most recent backup
3. Verify all data restored correctly
4. Update any changed passwords
5. Resume regular backup schedule

**Vault Corruption:**
1. Stop using corrupted vault immediately
2. Restore from most recent verified backup
3. Compare against previous backups if needed
4. Investigate cause of corruption
5. Implement additional backup measures

**Forgotten Master Password:**
- Master passwords cannot be recovered
- Must restore from backup created with known password
- Consider backup encryption with separate key
- Implement password hint system for future

### Recovery Testing

**Regular Recovery Tests:**
```bash
#!/bin/bash
# Monthly recovery test

TEST_DIR="/tmp/pwgen-recovery-test"
BACKUP_FILE="$HOME/backups/latest-backup.pwgen"

# Create test environment
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Test restore process
pwgen-cli restore --file "$BACKUP_FILE" --output "test-vault.db"

# Verify restored data
pwgen-cli --vault "test-vault.db" list --count > restore-count.txt
pwgen-cli --vault "test-vault.db" analyze integrity > integrity-report.txt

# Compare with original
echo "Recovery test completed. Check restore-count.txt and integrity-report.txt"

# Cleanup
cd -
rm -rf "$TEST_DIR"
```

## Backup Best Practices

### 3-2-1 Backup Rule

**Implementation:**
- **3 Copies**: Original + 2 backups
- **2 Different Media**: Local drive + cloud/external
- **1 Offsite**: Cloud storage or remote location

**Example Setup:**
```bash
#!/bin/bash
# 3-2-1 backup implementation

DATE=$(date +%Y%m%d)

# Copy 1: Original vault (daily use)
# Copy 2: Local backup
pwgen-cli backup create --output "$HOME/backups/local-$DATE.pwgen"

# Copy 3: External drive backup
pwgen-cli backup create --output "/media/external/pwgen-$DATE.pwgen"

# Copy 4: Cloud backup (offsite)
pwgen-cli backup create --output "cloud-$DATE.pwgen"
rclone copy "cloud-$DATE.pwgen" "gdrive:backups/pwgen/"
rm "cloud-$DATE.pwgen"
```

### Security Guidelines

1. **Encrypt All Backups**: Never store unencrypted backups
2. **Verify Regularly**: Test restore process monthly
3. **Secure Storage**: Use encrypted storage locations
4. **Access Control**: Limit backup access to authorized users
5. **Audit Trail**: Log backup and restore operations

### Operational Guidelines

1. **Automate Backups**: Use scheduled automated backups
2. **Monitor Success**: Verify backup completion
3. **Rotate Backups**: Implement proper retention policies
4. **Document Procedures**: Maintain recovery documentation
5. **Test Regularly**: Perform recovery tests

## Troubleshooting

### Backup Issues

**Common Problems:**
- Insufficient disk space
- Permission denied errors
- Backup corruption
- Network connectivity issues
- Encryption key problems

**Solutions:**
```bash
# Check disk space
df -h /backup/location

# Fix permissions
chmod 700 /backup/location
chown user:user /backup/location

# Verify backup integrity
pwgen-cli backup verify --file backup.pwgen

# Test network connectivity
ping backup-server.example.com

# Reset encryption keys
pwgen-cli backup create --output backup.pwgen --reset-encryption
```

### Restore Issues

**Common Problems:**
- Incorrect backup password
- Corrupted backup files
- Version compatibility issues
- Merge conflicts
- Missing dependencies

**Solutions:**
```bash
# Verify backup file
file backup.pwgen
pwgen-cli backup info --file backup.pwgen

# Try different passwords
pwgen-cli backup unlock --file backup.pwgen --try-passwords

# Check version compatibility
pwgen-cli backup version --file backup.pwgen

# Force restore with conflict resolution
pwgen-cli restore --file backup.pwgen --force --conflict-resolution backup
```

## Advanced Features

### Backup Encryption

**Multiple Encryption Layers:**
```bash
# Create backup with custom encryption
pwgen-cli backup create --output backup.pwgen --encrypt-algorithm "AES-256-GCM"

# Add additional encryption layer
gpg --encrypt --recipient "backup@example.com" backup.pwgen

# Age encryption
age -r age1public_key backup.pwgen > backup.pwgen.age
```

### Backup Monitoring

**Monitoring Script:**
```bash
#!/bin/bash
# Backup monitoring and alerting

BACKUP_DIR="$HOME/backups"
LATEST_BACKUP=$(ls -t "$BACKUP_DIR"/*.pwgen | head -1)
BACKUP_AGE=$(( ($(date +%s) - $(stat -c %Y "$LATEST_BACKUP")) / 86400 ))

if [ $BACKUP_AGE -gt 1 ]; then
  echo "WARNING: Latest backup is $BACKUP_AGE days old"
  # Send alert email or notification
  mail -s "PwGen Backup Alert" admin@example.com < backup-alert.txt
fi

# Verify latest backup
if ! pwgen-cli backup verify --file "$LATEST_BACKUP"; then
  echo "ERROR: Latest backup verification failed"
  # Send critical alert
fi
```

## Next Steps

- [Security Architecture](../security/architecture) - Understanding encryption
- [CLI Reference](../cli/commands) - Advanced backup commands
- [Import & Export](import-export) - Data migration tools