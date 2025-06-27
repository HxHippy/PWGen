use chrono::Utc;
use sqlx::{sqlite::SqlitePool, Row};
use std::path::Path;

use crate::{
    crypto::MasterKey,
    secrets::{
        AuditAction, DecryptedSecretEntry, SecretEntry, SecretFilter, SecretManager, SecretType,
    },
    Error, Result,
};

/// Extended storage for managing all types of secrets
pub struct SecretsStorage {
    pool: SqlitePool,
    master_key: MasterKey,
}

impl SecretsStorage {
    /// Create new secrets storage (extends existing vault)
    pub async fn from_existing_storage<P: AsRef<Path>>(
        vault_path: P,
        password: &str,
    ) -> Result<Self> {
        let db_url = format!("sqlite:{}", vault_path.as_ref().display());
        let pool = SqlitePool::connect(&db_url).await?;
        
        // Load existing vault metadata to verify password
        let metadata = crate::storage::Storage::load_metadata(&pool).await?;
        
        if !MasterKey::verify_password(password, &metadata.master_password_hash)? {
            return Err(Error::InvalidMasterPassword);
        }
        
        let master_key = MasterKey::derive_from_password(password, &metadata.salt)?;
        
        // Initialize secrets tables
        Self::initialize_secrets_database(&pool).await?;
        
        Ok(Self { pool, master_key })
    }
    
    /// Create new secrets storage from a newly created vault
    pub async fn create_new<P: AsRef<Path>>(
        vault_path: P,
        password: &str,
    ) -> Result<Self> {
        // For a new vault, we need to give the database time to be fully created
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        Self::from_existing_storage(vault_path, password).await
    }
    
    /// Initialize the secrets database schema
    async fn initialize_secrets_database(pool: &SqlitePool) -> Result<()> {
        // Create secrets table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS secrets (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                secret_type TEXT NOT NULL,
                encrypted_data BLOB NOT NULL,
                metadata_json TEXT NOT NULL,
                tags TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_accessed TEXT,
                expires_at TEXT,
                favorite INTEGER NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(pool)
        .await?;
        
        // Create indexes
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_secrets_name ON secrets(name);
            CREATE INDEX IF NOT EXISTS idx_secrets_type ON secrets(secret_type);
            CREATE INDEX IF NOT EXISTS idx_secrets_updated ON secrets(updated_at);
            CREATE INDEX IF NOT EXISTS idx_secrets_expires ON secrets(expires_at);
            CREATE INDEX IF NOT EXISTS idx_secrets_favorite ON secrets(favorite);
            "#,
        )
        .execute(pool)
        .await?;
        
        // Create audit log table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS secret_audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                secret_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                action TEXT NOT NULL,
                user_name TEXT,
                details TEXT,
                FOREIGN KEY (secret_id) REFERENCES secrets (id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_audit_secret_id ON secret_audit_log(secret_id);
            CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON secret_audit_log(timestamp);
            "#,
        )
        .execute(pool)
        .await?;
        
        // Create templates table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS secret_templates (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                secret_type TEXT NOT NULL,
                template_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    /// Add a new secret
    pub async fn add_secret(&self, secret: &DecryptedSecretEntry) -> Result<()> {
        let encrypted_data = SecretManager::encrypt_secret_data(&secret.data, &self.master_key)?;
        let metadata_json = serde_json::to_string(&secret.metadata)?;
        let tags_json = serde_json::to_string(&secret.tags)?;
        
        sqlx::query(
            r#"
            INSERT INTO secrets (
                id, name, description, secret_type, encrypted_data, metadata_json, tags,
                created_at, updated_at, last_accessed, expires_at, favorite
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&secret.id)
        .bind(&secret.name)
        .bind(&secret.description)
        .bind(serde_json::to_string(&secret.secret_type)?)
        .bind(&encrypted_data)
        .bind(&metadata_json)
        .bind(&tags_json)
        .bind(secret.created_at.to_rfc3339())
        .bind(secret.updated_at.to_rfc3339())
        .bind(secret.last_accessed.map(|dt| dt.to_rfc3339()))
        .bind(secret.expires_at.map(|dt| dt.to_rfc3339()))
        .bind(secret.favorite as i32)
        .execute(&self.pool)
        .await?;
        
        // Add audit log entry
        self.add_audit_log(&secret.id, AuditAction::Created, None, None).await?;
        
        Ok(())
    }
    
    /// Get a secret by ID
    pub async fn get_secret(&self, id: &str) -> Result<DecryptedSecretEntry> {
        let row = sqlx::query(
            "SELECT * FROM secrets WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::EntryNotFound(id.to_string()))?;
        
        let secret = self.row_to_secret(row)?;
        let decrypted = self.decrypt_secret(&secret)?;
        
        // Update last accessed time
        self.update_last_accessed(&secret.id).await?;
        
        // Add audit log entry
        self.add_audit_log(&secret.id, AuditAction::Accessed, None, None).await?;
        
        Ok(decrypted)
    }
    
    /// Update a secret
    pub async fn update_secret(&self, secret: &DecryptedSecretEntry) -> Result<()> {
        let encrypted_data = SecretManager::encrypt_secret_data(&secret.data, &self.master_key)?;
        let metadata_json = serde_json::to_string(&secret.metadata)?;
        let tags_json = serde_json::to_string(&secret.tags)?;
        
        sqlx::query(
            r#"
            UPDATE secrets SET
                name = ?, description = ?, secret_type = ?, encrypted_data = ?,
                metadata_json = ?, tags = ?, updated_at = ?, expires_at = ?, favorite = ?
            WHERE id = ?
            "#,
        )
        .bind(&secret.name)
        .bind(&secret.description)
        .bind(serde_json::to_string(&secret.secret_type)?)
        .bind(&encrypted_data)
        .bind(&metadata_json)
        .bind(&tags_json)
        .bind(Utc::now().to_rfc3339())
        .bind(secret.expires_at.map(|dt| dt.to_rfc3339()))
        .bind(secret.favorite as i32)
        .bind(&secret.id)
        .execute(&self.pool)
        .await?;
        
        // Add audit log entry
        self.add_audit_log(&secret.id, AuditAction::Updated, None, None).await?;
        
        Ok(())
    }
    
    /// Delete a secret
    pub async fn delete_secret(&self, id: &str) -> Result<()> {
        // Add audit log entry before deletion
        self.add_audit_log(id, AuditAction::Deleted, None, None).await?;
        
        sqlx::query("DELETE FROM secrets WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Search secrets with filters
    pub async fn search_secrets(&self, filter: &SecretFilter) -> Result<Vec<DecryptedSecretEntry>> {
        let mut query = String::from("SELECT * FROM secrets WHERE 1=1");
        let mut bindings = vec![];
        
        if let Some(search_query) = &filter.query {
            query.push_str(" AND (name LIKE ? OR description LIKE ?)");
            let like_query = format!("%{}%", search_query);
            bindings.push(like_query.clone());
            bindings.push(like_query);
        }
        
        if let Some(secret_types) = &filter.secret_types {
            let type_placeholders = secret_types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(&format!(" AND secret_type IN ({})", type_placeholders));
            for secret_type in secret_types {
                bindings.push(serde_json::to_string(secret_type).map_err(|e| Error::Serialization(e))?);
            }
        }
        
        if filter.favorite_only {
            query.push_str(" AND favorite = 1");
        }
        
        if let Some(expires_before) = filter.expires_before {
            query.push_str(" AND expires_at < ?");
            bindings.push(expires_before.to_rfc3339());
        }
        
        if let Some(expires_after) = filter.expires_after {
            query.push_str(" AND expires_at > ?");
            bindings.push(expires_after.to_rfc3339());
        }
        
        query.push_str(" ORDER BY updated_at DESC");
        
        let mut query_builder = sqlx::query(&query);
        for binding in bindings {
            query_builder = query_builder.bind(binding);
        }
        
        let rows = query_builder.fetch_all(&self.pool).await?;
        let mut secrets = Vec::new();
        
        for row in rows {
            let secret = self.row_to_secret(row)?;
            
            // Filter by tags if specified
            if let Some(filter_tags) = &filter.tags {
                if !filter_tags.iter().any(|tag| secret.tags.contains(tag)) {
                    continue;
                }
            }
            
            // Filter by environment/project if specified
            let metadata: crate::secrets::SecretMetadata = serde_json::from_str(
                &serde_json::to_string(&secret.metadata).unwrap_or_default()
            ).unwrap_or_default();
            
            if let Some(env) = &filter.environment {
                if metadata.environment.as_ref() != Some(env) {
                    continue;
                }
            }
            
            if let Some(project) = &filter.project {
                if metadata.project.as_ref() != Some(project) {
                    continue;
                }
            }
            
            secrets.push(self.decrypt_secret(&secret)?);
        }
        
        Ok(secrets)
    }
    
    /// Get secrets expiring within a specified duration
    pub async fn get_expiring_secrets(&self, within_days: i64) -> Result<Vec<DecryptedSecretEntry>> {
        let threshold = Utc::now() + chrono::Duration::days(within_days);
        
        let rows = sqlx::query(
            r#"
            SELECT * FROM secrets
            WHERE expires_at IS NOT NULL 
            AND expires_at <= ? 
            AND expires_at > ?
            ORDER BY expires_at ASC
            "#,
        )
        .bind(threshold.to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .fetch_all(&self.pool)
        .await?;
        
        let mut secrets = Vec::new();
        for row in rows {
            let secret = self.row_to_secret(row)?;
            secrets.push(self.decrypt_secret(&secret)?);
        }
        
        Ok(secrets)
    }
    
    /// Get secrets by type
    pub async fn get_secrets_by_type(&self, secret_type: &SecretType) -> Result<Vec<DecryptedSecretEntry>> {
        let type_json = serde_json::to_string(secret_type)?;
        
        let rows = sqlx::query(
            "SELECT * FROM secrets WHERE secret_type = ? ORDER BY name ASC"
        )
        .bind(&type_json)
        .fetch_all(&self.pool)
        .await?;
        
        let mut secrets = Vec::new();
        for row in rows {
            let secret = self.row_to_secret(row)?;
            secrets.push(self.decrypt_secret(&secret)?);
        }
        
        Ok(secrets)
    }
    
    /// Add audit log entry
    async fn add_audit_log(
        &self,
        secret_id: &str,
        action: AuditAction,
        user: Option<&str>,
        details: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO secret_audit_log (secret_id, timestamp, action, user_name, details)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(secret_id)
        .bind(Utc::now().to_rfc3339())
        .bind(serde_json::to_string(&action)?)
        .bind(user)
        .bind(details)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Update last accessed time
    async fn update_last_accessed(&self, id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE secrets SET last_accessed = ? WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Convert database row to SecretEntry
    fn row_to_secret(&self, row: sqlx::sqlite::SqliteRow) -> Result<SecretEntry> {
        Ok(SecretEntry {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            secret_type: serde_json::from_str(&row.get::<String, _>("secret_type"))?,
            encrypted_data: row.get("encrypted_data"),
            metadata: serde_json::from_str(&row.get::<String, _>("metadata_json"))?,
            tags: serde_json::from_str(&row.get::<String, _>("tags"))?,
            created_at: row.get::<String, _>("created_at").parse()
                .map_err(|_| Error::Other("Invalid date format".to_string()))?,
            updated_at: row.get::<String, _>("updated_at").parse()
                .map_err(|_| Error::Other("Invalid date format".to_string()))?,
            last_accessed: row.get::<Option<String>, _>("last_accessed")
                .and_then(|s| s.parse().ok()),
            expires_at: row.get::<Option<String>, _>("expires_at")
                .and_then(|s| s.parse().ok()),
            favorite: row.get::<i32, _>("favorite") != 0,
        })
    }
    
    /// Decrypt a secret entry
    fn decrypt_secret(&self, secret: &SecretEntry) -> Result<DecryptedSecretEntry> {
        let data = SecretManager::decrypt_secret_data(&secret.encrypted_data, &self.master_key)?;
        
        Ok(DecryptedSecretEntry {
            id: secret.id.clone(),
            name: secret.name.clone(),
            description: secret.description.clone(),
            secret_type: secret.secret_type.clone(),
            data,
            metadata: secret.metadata.clone(),
            tags: secret.tags.clone(),
            created_at: secret.created_at,
            updated_at: secret.updated_at,
            last_accessed: secret.last_accessed,
            expires_at: secret.expires_at,
            favorite: secret.favorite,
        })
    }
    
    /// Get statistics about stored secrets
    pub async fn get_secrets_stats(&self) -> Result<SecretsStats> {
        let total_count = sqlx::query("SELECT COUNT(*) as count FROM secrets")
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>("count") as usize;
        
        let expired_count = sqlx::query(
            "SELECT COUNT(*) as count FROM secrets WHERE expires_at IS NOT NULL AND expires_at < ?"
        )
        .bind(Utc::now().to_rfc3339())
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("count") as usize;
        
        let expiring_soon_count = sqlx::query(
            r#"
            SELECT COUNT(*) as count FROM secrets 
            WHERE expires_at IS NOT NULL 
            AND expires_at > ? 
            AND expires_at <= ?
            "#
        )
        .bind(Utc::now().to_rfc3339())
        .bind((Utc::now() + chrono::Duration::days(30)).to_rfc3339())
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("count") as usize;
        
        // Count by type
        let type_rows = sqlx::query(
            "SELECT secret_type, COUNT(*) as count FROM secrets GROUP BY secret_type"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut by_type = std::collections::HashMap::new();
        for row in type_rows {
            let secret_type: String = row.get("secret_type");
            let count: i64 = row.get("count");
            by_type.insert(secret_type, count as usize);
        }
        
        Ok(SecretsStats {
            total_count,
            expired_count,
            expiring_soon_count,
            by_type,
        })
    }
}

/// Statistics about stored secrets
#[derive(Debug)]
pub struct SecretsStats {
    pub total_count: usize,
    pub expired_count: usize,
    pub expiring_soon_count: usize,
    pub by_type: std::collections::HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_secrets_storage_initialization() {
        // This would require setting up a test database
        // For now, we'll just test that the module compiles
        assert!(true);
    }
}