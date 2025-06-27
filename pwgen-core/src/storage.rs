use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePool, Row};
use std::path::Path;

use crate::{
    crypto::MasterKey,
    models::{DecryptedPasswordEntry, PasswordEntry, SearchFilter, SortField, SortOrder, VaultMetadata},
    Error, Result,
};

pub struct Storage {
    pool: SqlitePool,
    master_key: MasterKey,
}

impl Storage {
    pub async fn create_new<P: AsRef<Path>>(path: P, password: &str) -> Result<Self> {
        let salt = MasterKey::generate_salt();
        let master_key = MasterKey::derive_from_password(password, &salt)?;
        let password_hash = MasterKey::hash_password_for_storage(password)?;
        
        // Ensure the file can be created with read-write-create mode
        let db_url = format!("sqlite:{}?mode=rwc", path.as_ref().display());
        let pool = SqlitePool::connect(&db_url).await?;
        
        Self::initialize_database(&pool).await?;
        
        let vault_metadata = VaultMetadata {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Personal Vault".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: "1.0.0".to_string(),
            master_password_hash: password_hash,
            salt,
        };
        
        Self::save_metadata(&pool, &vault_metadata).await?;
        
        Ok(Self { pool, master_key })
    }
    
    pub async fn open<P: AsRef<Path>>(path: P, password: &str) -> Result<Self> {
        let db_url = format!("sqlite:{}", path.as_ref().display());
        let pool = SqlitePool::connect(&db_url).await?;
        
        let metadata = Self::load_metadata(&pool).await?;
        
        if !MasterKey::verify_password(password, &metadata.master_password_hash)? {
            return Err(Error::InvalidMasterPassword);
        }
        
        let master_key = MasterKey::derive_from_password(password, &metadata.salt)?;
        
        Ok(Self { pool, master_key })
    }
    
    async fn initialize_database(pool: &SqlitePool) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS vault_metadata (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                version TEXT NOT NULL,
                master_password_hash TEXT NOT NULL,
                salt BLOB NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS password_entries (
                id TEXT PRIMARY KEY,
                site TEXT NOT NULL,
                username TEXT NOT NULL,
                encrypted_password BLOB NOT NULL,
                notes TEXT,
                tags TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_used TEXT,
                password_changed_at TEXT NOT NULL,
                favorite INTEGER NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_site ON password_entries(site);
            CREATE INDEX IF NOT EXISTS idx_username ON password_entries(username);
            CREATE INDEX IF NOT EXISTS idx_favorite ON password_entries(favorite);
            CREATE INDEX IF NOT EXISTS idx_updated_at ON password_entries(updated_at);
            "#,
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    async fn save_metadata(pool: &SqlitePool, metadata: &VaultMetadata) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vault_metadata (
                id, name, created_at, updated_at, version, master_password_hash, salt
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&metadata.id)
        .bind(&metadata.name)
        .bind(metadata.created_at.to_rfc3339())
        .bind(metadata.updated_at.to_rfc3339())
        .bind(&metadata.version)
        .bind(&metadata.master_password_hash)
        .bind(&metadata.salt)
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn load_metadata(pool: &SqlitePool) -> Result<VaultMetadata> {
        let row = sqlx::query(
            r#"
            SELECT id, name, created_at, updated_at, version, master_password_hash, salt
            FROM vault_metadata
            LIMIT 1
            "#,
        )
        .fetch_one(pool)
        .await?;
        
        Ok(VaultMetadata {
            id: row.get("id"),
            name: row.get("name"),
            created_at: row.get::<String, _>("created_at").parse().map_err(|_| Error::Other("Invalid date format".to_string()))?,
            updated_at: row.get::<String, _>("updated_at").parse().map_err(|_| Error::Other("Invalid date format".to_string()))?,
            version: row.get("version"),
            master_password_hash: row.get("master_password_hash"),
            salt: row.get("salt"),
        })
    }
    
    pub async fn add_entry(&self, entry: &DecryptedPasswordEntry) -> Result<()> {
        let encrypted_password = self.master_key.encrypt(entry.password.as_bytes())?;
        let tags_json = serde_json::to_string(&entry.tags)?;
        
        sqlx::query(
            r#"
            INSERT INTO password_entries (
                id, site, username, encrypted_password, notes, tags,
                created_at, updated_at, last_used, password_changed_at, favorite
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&entry.id)
        .bind(&entry.site)
        .bind(&entry.username)
        .bind(&encrypted_password)
        .bind(&entry.notes)
        .bind(&tags_json)
        .bind(entry.created_at.to_rfc3339())
        .bind(entry.updated_at.to_rfc3339())
        .bind(entry.last_used.map(|dt| dt.to_rfc3339()))
        .bind(entry.password_changed_at.to_rfc3339())
        .bind(entry.favorite as i32)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn update_entry(&self, entry: &DecryptedPasswordEntry) -> Result<()> {
        let encrypted_password = self.master_key.encrypt(entry.password.as_bytes())?;
        let tags_json = serde_json::to_string(&entry.tags)?;
        
        sqlx::query(
            r#"
            UPDATE password_entries SET
                site = ?, username = ?, encrypted_password = ?, notes = ?, tags = ?,
                updated_at = ?, last_used = ?, password_changed_at = ?, favorite = ?
            WHERE id = ?
            "#,
        )
        .bind(&entry.site)
        .bind(&entry.username)
        .bind(&encrypted_password)
        .bind(&entry.notes)
        .bind(&tags_json)
        .bind(Utc::now().to_rfc3339())
        .bind(entry.last_used.map(|dt| dt.to_rfc3339()))
        .bind(entry.password_changed_at.to_rfc3339())
        .bind(entry.favorite as i32)
        .bind(&entry.id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_entry(&self, id: &str) -> Result<DecryptedPasswordEntry> {
        let entry = self.get_encrypted_entry(id).await?;
        self.decrypt_entry(&entry)
    }
    
    async fn get_encrypted_entry(&self, id: &str) -> Result<PasswordEntry> {
        let row = sqlx::query(
            r#"
            SELECT * FROM password_entries WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::EntryNotFound(id.to_string()))?;
        
        Ok(self.row_to_entry(row)?)
    }
    
    pub async fn delete_entry(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM password_entries WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn search_entries(&self, filter: &SearchFilter) -> Result<Vec<DecryptedPasswordEntry>> {
        let mut query = String::from("SELECT * FROM password_entries WHERE 1=1");
        let mut bindings = vec![];
        
        if let Some(search_query) = &filter.query {
            query.push_str(" AND (site LIKE ? OR username LIKE ? OR notes LIKE ?)");
            let like_query = format!("%{}%", search_query);
            bindings.push(like_query.clone());
            bindings.push(like_query.clone());
            bindings.push(like_query);
        }
        
        if filter.favorite_only {
            query.push_str(" AND favorite = 1");
        }
        
        query.push_str(&format!(
            " ORDER BY {} {}",
            match filter.sort_by {
                SortField::Site => "site",
                SortField::Username => "username",
                SortField::CreatedAt => "created_at",
                SortField::UpdatedAt => "updated_at",
                SortField::LastUsed => "COALESCE(last_used, created_at)",
            },
            match filter.sort_order {
                SortOrder::Ascending => "ASC",
                SortOrder::Descending => "DESC",
            }
        ));
        
        let mut query_builder = sqlx::query(&query);
        for binding in bindings {
            query_builder = query_builder.bind(binding);
        }
        
        let rows = query_builder.fetch_all(&self.pool).await?;
        let mut entries = Vec::new();
        
        for row in rows {
            let entry = self.row_to_entry(row)?;
            if let Some(tags) = &filter.tags {
                let entry_tags: Vec<String> = serde_json::from_str(
                    entry.tags.get(0).map(|s| s.as_str()).unwrap_or("[]")
                )?;
                if !tags.iter().any(|tag| entry_tags.contains(tag)) {
                    continue;
                }
            }
            entries.push(self.decrypt_entry(&entry)?);
        }
        
        Ok(entries)
    }
    
    fn row_to_entry(&self, row: sqlx::sqlite::SqliteRow) -> Result<PasswordEntry> {
        Ok(PasswordEntry {
            id: row.get("id"),
            site: row.get("site"),
            username: row.get("username"),
            encrypted_password: row.get("encrypted_password"),
            notes: row.get("notes"),
            tags: serde_json::from_str(row.get::<Option<String>, _>("tags").as_deref().unwrap_or("[]"))?,
            created_at: row.get::<String, _>("created_at").parse().map_err(|_| Error::Other("Invalid date format".to_string()))?,
            updated_at: row.get::<String, _>("updated_at").parse().map_err(|_| Error::Other("Invalid date format".to_string()))?,
            last_used: row.get::<Option<String>, _>("last_used")
                .and_then(|s| s.parse().ok()),
            password_changed_at: row.get::<String, _>("password_changed_at").parse().map_err(|_| Error::Other("Invalid date format".to_string()))?,
            favorite: row.get::<i32, _>("favorite") != 0,
        })
    }
    
    fn decrypt_entry(&self, entry: &PasswordEntry) -> Result<DecryptedPasswordEntry> {
        let decrypted_password = self.master_key.decrypt(&entry.encrypted_password)?;
        let password = String::from_utf8(decrypted_password)
            .map_err(|_| Error::Decryption("Invalid UTF-8 in decrypted password".to_string()))?;
        
        Ok(DecryptedPasswordEntry {
            id: entry.id.clone(),
            site: entry.site.clone(),
            username: entry.username.clone(),
            password,
            notes: entry.notes.clone(),
            tags: entry.tags.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
            last_used: entry.last_used,
            password_changed_at: entry.password_changed_at,
            favorite: entry.favorite,
        })
    }
    
    pub async fn mark_as_used(&self, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE password_entries SET last_used = ? WHERE id = ?
            "#,
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get vault metadata for backup purposes
    pub async fn get_vault_metadata(&self) -> Result<VaultMetadata> {
        Self::load_metadata(&self.pool).await
    }
    
    /// Get entries modified since a specific date (for incremental backups)
    pub async fn get_entries_since(&self, since: DateTime<Utc>) -> Result<Vec<DecryptedPasswordEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM password_entries 
            WHERE updated_at > ? OR created_at > ?
            ORDER BY updated_at DESC
            "#,
        )
        .bind(since.to_rfc3339())
        .bind(since.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;
        
        let mut entries = Vec::new();
        for row in rows {
            let entry = self.row_to_entry(row)?;
            entries.push(self.decrypt_entry(&entry)?);
        }
        
        Ok(entries)
    }
    
    /// Add or update an entry (for restore operations)
    pub async fn add_or_update_entry(&self, entry: &DecryptedPasswordEntry) -> Result<()> {
        // Check if entry exists
        match self.get_entry(&entry.id).await {
            Ok(_) => {
                // Entry exists, update it
                self.update_entry(entry).await
            }
            Err(_) => {
                // Entry doesn't exist, add it
                self.add_entry(entry).await
            }
        }
    }
    
    /// Get the total number of entries in the vault
    pub async fn get_entry_count(&self) -> Result<usize> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM password_entries")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.get::<i64, _>("count") as usize)
    }
    
    /// Get vault statistics for backup metadata
    pub async fn get_vault_stats(&self) -> Result<VaultStats> {
        let entry_count = self.get_entry_count().await?;
        
        let recent_row = sqlx::query(
            "SELECT MAX(updated_at) as last_modified FROM password_entries"
        )
        .fetch_one(&self.pool)
        .await?;
        
        let last_modified: Option<String> = recent_row.get("last_modified");
        let last_modified = last_modified
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(Utc::now);
        
        Ok(VaultStats {
            entry_count,
            last_modified,
        })
    }
}

#[derive(Debug)]
pub struct VaultStats {
    pub entry_count: usize,
    pub last_modified: DateTime<Utc>,
}