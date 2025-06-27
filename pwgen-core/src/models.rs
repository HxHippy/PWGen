use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub site: String,
    pub username: String,
    pub encrypted_password: Vec<u8>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub password_changed_at: DateTime<Utc>,
    pub favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedPasswordEntry {
    pub id: String,
    pub site: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub password_changed_at: DateTime<Utc>,
    pub favorite: bool,
}

impl Drop for DecryptedPasswordEntry {
    fn drop(&mut self) {
        self.password.zeroize();
        if let Some(ref mut notes) = self.notes {
            notes.zeroize();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: String,
    pub master_password_hash: String,
    pub salt: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub query: Option<String>,
    pub tags: Option<Vec<String>>,
    pub favorite_only: bool,
    pub sort_by: SortField,
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortField {
    Site,
    Username,
    CreatedAt,
    UpdatedAt,
    LastUsed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self {
            query: None,
            tags: None,
            favorite_only: false,
            sort_by: SortField::UpdatedAt,
            sort_order: SortOrder::Descending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportData {
    pub source: ImportSource,
    pub entries: Vec<ImportEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSource {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Bitwarden,
    OnePassword,
    LastPass,
    KeePass,
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEntry {
    pub site: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
}

impl Drop for ImportEntry {
    fn drop(&mut self) {
        self.password.zeroize();
        if let Some(ref mut notes) = self.notes {
            notes.zeroize();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub vault_id: String,
    pub entry_count: usize,
    pub file_size: u64,
    pub checksum: String,
}