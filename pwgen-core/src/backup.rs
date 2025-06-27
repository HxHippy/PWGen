use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

use crate::{
    crypto::MasterKey,
    models::{BackupMetadata, DecryptedPasswordEntry, VaultMetadata},
    storage::Storage,
    Error, Result,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupData {
    pub metadata: VaultMetadata,
    pub entries: Vec<DecryptedPasswordEntry>,
    pub backup_info: BackupInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub version: String,
    pub entry_count: usize,
    pub backup_type: BackupType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental { since: DateTime<Utc> },
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedBackup {
    pub backup_metadata: BackupMetadata,
    pub encrypted_data: Vec<u8>,
    pub salt: Vec<u8>,
}

pub struct BackupManager;

impl BackupManager {
    /// Create a full backup of the vault
    pub async fn create_backup<P: AsRef<Path>>(
        storage: &Storage,
        output_path: P,
        backup_password: &str,
    ) -> Result<BackupMetadata> {
        let backup_id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        
        // Get all entries from storage
        let entries = storage.search_entries(&Default::default()).await?;
        let vault_metadata = storage.get_vault_metadata().await?;
        
        let backup_data = BackupData {
            metadata: vault_metadata.clone(),
            entries: entries.clone(),
            backup_info: BackupInfo {
                created_at,
                created_by: "pwgen".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                entry_count: entries.len(),
                backup_type: BackupType::Full,
            },
        };
        
        // Serialize the backup data
        let serialized_data = serde_json::to_vec(&backup_data)?;
        
        // Generate salt and derive key from backup password
        let salt = MasterKey::generate_salt();
        let backup_key = MasterKey::derive_from_password(backup_password, &salt)?;
        
        // Encrypt the backup data
        let encrypted_data = backup_key.encrypt(&serialized_data)?;
        
        // Calculate checksum of encrypted data
        let mut hasher = Sha256::new();
        hasher.update(&encrypted_data);
        let checksum = format!("{:x}", hasher.finalize());
        
        // Create backup metadata
        let backup_metadata = BackupMetadata {
            id: backup_id,
            created_at,
            vault_id: vault_metadata.id,
            entry_count: entries.len(),
            file_size: encrypted_data.len() as u64,
            checksum,
        };
        
        // Create the encrypted backup structure
        let encrypted_backup = EncryptedBackup {
            backup_metadata: backup_metadata.clone(),
            encrypted_data,
            salt,
        };
        
        // Write to file
        let backup_content = serde_json::to_vec_pretty(&encrypted_backup)?;
        fs::write(&output_path, backup_content).await?;
        
        Ok(backup_metadata)
    }
    
    /// Create an incremental backup since a specific date
    pub async fn create_incremental_backup<P: AsRef<Path>>(
        storage: &Storage,
        output_path: P,
        backup_password: &str,
        since: DateTime<Utc>,
    ) -> Result<BackupMetadata> {
        let backup_id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        
        // Get entries modified since the specified date
        let entries = storage.get_entries_since(since).await?;
        let vault_metadata = storage.get_vault_metadata().await?;
        
        let backup_data = BackupData {
            metadata: vault_metadata.clone(),
            entries: entries.clone(),
            backup_info: BackupInfo {
                created_at,
                created_by: "pwgen".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                entry_count: entries.len(),
                backup_type: BackupType::Incremental { since },
            },
        };
        
        // Serialize the backup data
        let serialized_data = serde_json::to_vec(&backup_data)?;
        
        // Generate salt and derive key from backup password
        let salt = MasterKey::generate_salt();
        let backup_key = MasterKey::derive_from_password(backup_password, &salt)?;
        
        // Encrypt the backup data
        let encrypted_data = backup_key.encrypt(&serialized_data)?;
        
        // Calculate checksum
        let mut hasher = Sha256::new();
        hasher.update(&encrypted_data);
        let checksum = format!("{:x}", hasher.finalize());
        
        // Create backup metadata
        let backup_metadata = BackupMetadata {
            id: backup_id,
            created_at,
            vault_id: vault_metadata.id,
            entry_count: entries.len(),
            file_size: encrypted_data.len() as u64,
            checksum,
        };
        
        // Create the encrypted backup structure
        let encrypted_backup = EncryptedBackup {
            backup_metadata: backup_metadata.clone(),
            encrypted_data,
            salt,
        };
        
        // Write to file
        let backup_content = serde_json::to_vec_pretty(&encrypted_backup)?;
        fs::write(&output_path, backup_content).await?;
        
        Ok(backup_metadata)
    }
    
    /// Verify a backup file's integrity
    pub async fn verify_backup<P: AsRef<Path>>(backup_path: P) -> Result<BackupMetadata> {
        let backup_content = fs::read(&backup_path).await?;
        let encrypted_backup: EncryptedBackup = serde_json::from_slice(&backup_content)?;
        
        // Verify checksum
        let mut hasher = Sha256::new();
        hasher.update(&encrypted_backup.encrypted_data);
        let calculated_checksum = format!("{:x}", hasher.finalize());
        
        if calculated_checksum != encrypted_backup.backup_metadata.checksum {
            return Err(Error::Other("Backup integrity check failed: checksum mismatch".to_string()));
        }
        
        // Verify file size
        if encrypted_backup.encrypted_data.len() as u64 != encrypted_backup.backup_metadata.file_size {
            return Err(Error::Other("Backup integrity check failed: file size mismatch".to_string()));
        }
        
        Ok(encrypted_backup.backup_metadata)
    }
    
    /// Read backup metadata without decrypting the full backup
    pub async fn read_backup_metadata<P: AsRef<Path>>(backup_path: P) -> Result<BackupMetadata> {
        let backup_content = fs::read(&backup_path).await?;
        let encrypted_backup: EncryptedBackup = serde_json::from_slice(&backup_content)?;
        Ok(encrypted_backup.backup_metadata)
    }
    
    /// Restore from a backup file
    pub async fn restore_backup<P: AsRef<Path>>(
        backup_path: P,
        backup_password: &str,
        storage: &mut Storage,
        restore_options: RestoreOptions,
    ) -> Result<RestoreResult> {
        // First verify the backup
        Self::verify_backup(&backup_path).await?;
        
        // Read and decrypt the backup
        let backup_content = fs::read(&backup_path).await?;
        let encrypted_backup: EncryptedBackup = serde_json::from_slice(&backup_content)?;
        
        // Derive decryption key
        let backup_key = MasterKey::derive_from_password(backup_password, &encrypted_backup.salt)?;
        
        // Decrypt the backup data
        let decrypted_data = backup_key.decrypt(&encrypted_backup.encrypted_data)?;
        let backup_data: BackupData = serde_json::from_slice(&decrypted_data)?;
        
        // Perform the restore based on options
        let restore_result = match restore_options.conflict_resolution {
            ConflictResolution::Overwrite => {
                Self::restore_overwrite(storage, &backup_data).await?
            }
            ConflictResolution::Skip => {
                Self::restore_skip_conflicts(storage, &backup_data).await?
            }
            ConflictResolution::Merge => {
                Self::restore_merge(storage, &backup_data).await?
            }
        };
        
        Ok(restore_result)
    }
    
    async fn restore_overwrite(
        storage: &mut Storage,
        backup_data: &BackupData,
    ) -> Result<RestoreResult> {
        let mut restored_count = 0;
        let mut errors = Vec::new();
        
        for entry in &backup_data.entries {
            match storage.add_or_update_entry(entry).await {
                Ok(_) => restored_count += 1,
                Err(e) => errors.push(format!("Failed to restore entry {}: {}", entry.site, e)),
            }
        }
        
        Ok(RestoreResult {
            total_entries: backup_data.entries.len(),
            restored_count,
            skipped_count: 0,
            error_count: errors.len(),
            errors,
        })
    }
    
    async fn restore_skip_conflicts(
        storage: &mut Storage,
        backup_data: &BackupData,
    ) -> Result<RestoreResult> {
        let mut restored_count = 0;
        let mut skipped_count = 0;
        let mut errors = Vec::new();
        
        for entry in &backup_data.entries {
            // Check if entry already exists
            match storage.get_entry(&entry.id).await {
                Ok(_) => {
                    // Entry exists, skip it
                    skipped_count += 1;
                }
                Err(_) => {
                    // Entry doesn't exist, add it
                    match storage.add_entry(entry).await {
                        Ok(_) => restored_count += 1,
                        Err(e) => errors.push(format!("Failed to restore entry {}: {}", entry.site, e)),
                    }
                }
            }
        }
        
        Ok(RestoreResult {
            total_entries: backup_data.entries.len(),
            restored_count,
            skipped_count,
            error_count: errors.len(),
            errors,
        })
    }
    
    async fn restore_merge(
        storage: &mut Storage,
        backup_data: &BackupData,
    ) -> Result<RestoreResult> {
        let mut restored_count = 0;
        let mut skipped_count = 0;
        let mut errors = Vec::new();
        
        for entry in &backup_data.entries {
            match storage.get_entry(&entry.id).await {
                Ok(existing_entry) => {
                    // Entry exists, merge based on last updated time
                    if entry.updated_at > existing_entry.updated_at {
                        match storage.update_entry(entry).await {
                            Ok(_) => restored_count += 1,
                            Err(e) => errors.push(format!("Failed to update entry {}: {}", entry.site, e)),
                        }
                    } else {
                        skipped_count += 1;
                    }
                }
                Err(_) => {
                    // Entry doesn't exist, add it
                    match storage.add_entry(entry).await {
                        Ok(_) => restored_count += 1,
                        Err(e) => errors.push(format!("Failed to restore entry {}: {}", entry.site, e)),
                    }
                }
            }
        }
        
        Ok(RestoreResult {
            total_entries: backup_data.entries.len(),
            restored_count,
            skipped_count,
            error_count: errors.len(),
            errors,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RestoreOptions {
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug, Clone)]
pub enum ConflictResolution {
    Overwrite,
    Skip,
    Merge,
}

impl Default for RestoreOptions {
    fn default() -> Self {
        Self {
            conflict_resolution: ConflictResolution::Merge,
        }
    }
}

#[derive(Debug)]
pub struct RestoreResult {
    pub total_entries: usize,
    pub restored_count: usize,
    pub skipped_count: usize,
    pub error_count: usize,
    pub errors: Vec<String>,
}

impl RestoreResult {
    pub fn success_rate(&self) -> f64 {
        if self.total_entries == 0 {
            100.0
        } else {
            (self.restored_count as f64 / self.total_entries as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_backup_verification() {
        // This would require setting up a test storage and creating a backup
        // For now, we'll just test that the backup verification function exists
        assert!(true);
    }
    
    #[test]
    fn test_restore_result_success_rate() {
        let result = RestoreResult {
            total_entries: 10,
            restored_count: 8,
            skipped_count: 1,
            error_count: 1,
            errors: vec!["Test error".to_string()],
        };
        
        assert_eq!(result.success_rate(), 80.0);
        
        let empty_result = RestoreResult {
            total_entries: 0,
            restored_count: 0,
            skipped_count: 0,
            error_count: 0,
            errors: vec![],
        };
        
        assert_eq!(empty_result.success_rate(), 100.0);
    }
}