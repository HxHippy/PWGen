use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::Zeroize;

use crate::{crypto::MasterKey, Result};

/// Comprehensive secret entry that can store various types of sensitive data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretEntry {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub secret_type: SecretType,
    pub encrypted_data: Vec<u8>,
    pub metadata: SecretMetadata,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub favorite: bool,
}

/// Decrypted version of a secret entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedSecretEntry {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub secret_type: SecretType,
    pub data: SecretData,
    pub metadata: SecretMetadata,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub favorite: bool,
}

impl Drop for DecryptedSecretEntry {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}

/// Types of secrets that can be stored
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecretType {
    /// Traditional password entry
    Password,
    /// SSH private/public key pairs
    SshKey,
    /// API keys and authentication tokens
    ApiKey,
    /// Authentication tokens (JWT, OAuth, etc.)
    Token,
    /// Secure documents and files
    Document,
    /// Configuration files and environment variables
    Configuration,
    /// Secure notes and text documents
    SecureNote,
    /// Digital certificates and keys
    Certificate,
    /// Database connection strings
    ConnectionString,
    /// Cloud service credentials
    CloudCredentials,
    /// Custom secret type
    Custom(String),
}

/// The actual secret data (encrypted when stored)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretData {
    Password {
        username: String,
        password: String,
        url: Option<String>,
        notes: Option<String>,
    },
    SshKey {
        key_type: SshKeyType,
        private_key: Option<String>,
        public_key: Option<String>,
        passphrase: Option<String>,
        comment: Option<String>,
        fingerprint: Option<String>,
    },
    ApiKey {
        provider: crate::api_keys::ApiKeyProvider,
        key_id: String,
        api_key: String,
        api_secret: Option<String>,
        token_type: String,
        permissions: crate::api_keys::ApiKeyPermissions,
        environment: String,
        endpoint_url: Option<String>,
        rotation_info: crate::api_keys::RotationInfo,
        usage_stats: crate::api_keys::UsageStats,
    },
    Token {
        token_type: String,
        access_token: String,
        refresh_token: Option<String>,
        token_secret: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        issued_at: Option<DateTime<Utc>>,
        issuer: Option<String>,
        audience: Option<String>,
        subject: Option<String>,
        scopes: Vec<String>,
        claims: std::collections::HashMap<String, String>,
    },
    Document {
        filename: String,
        content_type: String,
        content: Vec<u8>,
        checksum: String,
    },
    Configuration {
        format: ConfigFormat,
        variables: HashMap<String, String>,
        template: Option<String>,
    },
    SecureNote {
        title: String,
        content: String,
        format: NoteFormat,
    },
    Certificate {
        cert_type: CertificateType,
        certificate: String,
        private_key: Option<String>,
        ca_chain: Option<Vec<String>>,
        subject: Option<String>,
        issuer: Option<String>,
    },
    ConnectionString {
        database_type: DatabaseType,
        host: String,
        port: Option<u16>,
        database: String,
        username: String,
        password: String,
        connection_string: String,
        ssl_config: Option<SslConfig>,
    },
    CloudCredentials {
        provider: CloudProvider,
        access_key: String,
        secret_key: String,
        region: Option<String>,
        additional_config: HashMap<String, String>,
    },
    Custom {
        schema: String,
        fields: HashMap<String, String>,
    },
}

impl Zeroize for SecretData {
    fn zeroize(&mut self) {
        match self {
            SecretData::Password { username, password, url, notes } => {
                username.zeroize();
                password.zeroize();
                if let Some(url) = url {
                    url.zeroize();
                }
                if let Some(notes) = notes {
                    notes.zeroize();
                }
            }
            SecretData::SshKey { private_key, public_key, passphrase, comment, fingerprint, .. } => {
                if let Some(private_key) = private_key {
                    private_key.zeroize();
                }
                if let Some(public_key) = public_key {
                    public_key.zeroize();
                }
                if let Some(passphrase) = passphrase {
                    passphrase.zeroize();
                }
                if let Some(comment) = comment {
                    comment.zeroize();
                }
                if let Some(fingerprint) = fingerprint {
                    fingerprint.zeroize();
                }
            }
            SecretData::ApiKey { api_key, api_secret, permissions, rotation_info, usage_stats, .. } => {
                api_key.zeroize();
                if let Some(secret) = api_secret {
                    secret.zeroize();
                }
                // Zeroize sensitive fields in nested structs
                permissions.scopes.zeroize();
                for (_, values) in permissions.resource_access.iter_mut() {
                    values.zeroize();
                }
                permissions.resource_access.clear();
                if let Some(error) = &mut usage_stats.last_error {
                    error.zeroize();
                }
            }
            SecretData::Token { access_token, refresh_token, token_secret, scopes, claims, .. } => {
                access_token.zeroize();
                if let Some(refresh) = refresh_token {
                    refresh.zeroize();
                }
                if let Some(secret) = token_secret {
                    secret.zeroize();
                }
                scopes.zeroize();
                for (_, value) in claims.iter_mut() {
                    value.zeroize();
                }
                claims.clear();
            }
            SecretData::Document { filename, content_type, content, checksum } => {
                filename.zeroize();
                content_type.zeroize();
                content.zeroize();
                checksum.zeroize();
            }
            SecretData::Configuration { variables, template, .. } => {
                for (_, value) in variables.iter_mut() {
                    value.zeroize();
                }
                variables.clear();
                if let Some(template) = template {
                    template.zeroize();
                }
            }
            SecretData::SecureNote { title, content, .. } => {
                title.zeroize();
                content.zeroize();
            }
            SecretData::Certificate { certificate, private_key, ca_chain, subject, issuer, .. } => {
                certificate.zeroize();
                if let Some(private_key) = private_key {
                    private_key.zeroize();
                }
                if let Some(ca_chain) = ca_chain {
                    ca_chain.zeroize();
                }
                if let Some(subject) = subject {
                    subject.zeroize();
                }
                if let Some(issuer) = issuer {
                    issuer.zeroize();
                }
            }
            SecretData::ConnectionString { host, database, username, password, connection_string, ssl_config, .. } => {
                host.zeroize();
                database.zeroize();
                username.zeroize();
                password.zeroize();
                connection_string.zeroize();
                if let Some(ssl_config) = ssl_config {
                    ssl_config.zeroize();
                }
            }
            SecretData::CloudCredentials { access_key, secret_key, region, additional_config, .. } => {
                access_key.zeroize();
                secret_key.zeroize();
                if let Some(region) = region {
                    region.zeroize();
                }
                for (_, value) in additional_config.iter_mut() {
                    value.zeroize();
                }
                additional_config.clear();
            }
            SecretData::Custom { schema, fields } => {
                schema.zeroize();
                for (_, value) in fields.iter_mut() {
                    value.zeroize();
                }
                fields.clear();
            }
        }
    }
}

impl Drop for SecretData {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// SSH key types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SshKeyType {
    Rsa,
    Ed25519,
    Ecdsa,
    Dsa,
}

/// Configuration file formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigFormat {
    EnvFile,
    Json,
    Yaml,
    Toml,
    Xml,
    Properties,
    Custom(String),
}

/// Note formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NoteFormat {
    PlainText,
    Markdown,
    Html,
    RichText,
}

/// Certificate types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificateType {
    X509,
    Ssl,
    CodeSigning,
    ClientAuth,
    EmailProtection,
    Custom(String),
}

/// Database types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    PostgreSQL,
    MySQL,
    SQLite,
    MongoDB,
    Redis,
    Oracle,
    SQLServer,
    Custom(String),
}

/// SSL configuration for connections
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub struct SslConfig {
    pub enabled: bool,
    pub verify_ssl: bool,
    pub ca_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

/// Cloud service providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
    DigitalOcean,
    Linode,
    Vultr,
    Heroku,
    Cloudflare,
    Custom(String),
}

/// Metadata associated with secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMetadata {
    pub template: Option<String>,
    pub environment: Option<String>,
    pub project: Option<String>,
    pub owner: Option<String>,
    pub team: Option<String>,
    pub compliance: Option<ComplianceInfo>,
    pub audit_log: Vec<AuditEntry>,
}

/// Compliance and regulatory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    pub classification: DataClassification,
    pub retention_period: Option<chrono::Duration>,
    pub compliance_tags: Vec<String>,
    pub data_location: Option<String>,
}

/// Data classification levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: AuditAction,
    pub user: Option<String>,
    pub details: Option<String>,
}

/// Actions that can be audited
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    Created,
    Updated,
    Accessed,
    Copied,
    Exported,
    Deleted,
    Shared,
    PermissionsChanged,
}

/// Secret templates for common services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretTemplate {
    pub name: String,
    pub description: String,
    pub secret_type: SecretType,
    pub fields: Vec<TemplateField>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Template field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateField {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub validation: Option<String>,
}

/// Field types for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Password,
    Email,
    Url,
    Number,
    Boolean,
    Date,
    TextArea,
    File,
    Json,
}

/// Search and filter criteria for secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretFilter {
    pub query: Option<String>,
    pub secret_types: Option<Vec<SecretType>>,
    pub tags: Option<Vec<String>>,
    pub environment: Option<String>,
    pub project: Option<String>,
    pub expires_before: Option<DateTime<Utc>>,
    pub expires_after: Option<DateTime<Utc>>,
    pub favorite_only: bool,
    pub classification: Option<DataClassification>,
}

impl Default for SecretFilter {
    fn default() -> Self {
        Self {
            query: None,
            secret_types: None,
            tags: None,
            environment: None,
            project: None,
            expires_before: None,
            expires_after: None,
            favorite_only: false,
            classification: None,
        }
    }
}

impl Default for SecretMetadata {
    fn default() -> Self {
        Self {
            template: None,
            environment: None,
            project: None,
            owner: None,
            team: None,
            compliance: None,
            audit_log: vec![],
        }
    }
}

/// Manager for secret operations
pub struct SecretManager;

impl SecretManager {
    /// Encrypt secret data
    pub fn encrypt_secret_data(
        data: &SecretData,
        master_key: &MasterKey,
    ) -> Result<Vec<u8>> {
        let serialized = serde_json::to_vec(data)?;
        master_key.encrypt(&serialized)
    }
    
    /// Decrypt secret data
    pub fn decrypt_secret_data(
        encrypted_data: &[u8],
        master_key: &MasterKey,
    ) -> Result<SecretData> {
        let decrypted = master_key.decrypt(encrypted_data)?;
        let data: SecretData = serde_json::from_slice(&decrypted)?;
        Ok(data)
    }
    
    /// Generate a new audit entry
    pub fn create_audit_entry(action: AuditAction, user: Option<String>, details: Option<String>) -> AuditEntry {
        AuditEntry {
            timestamp: Utc::now(),
            action,
            user,
            details,
        }
    }
    
    /// Check if a secret is expired
    pub fn is_expired(secret: &SecretEntry) -> bool {
        if let Some(expires_at) = secret.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
    
    /// Get secrets expiring within a given duration
    pub fn get_expiring_secrets(
        secrets: &[SecretEntry],
        within: chrono::Duration,
    ) -> Vec<&SecretEntry> {
        let threshold = Utc::now() + within;
        secrets
            .iter()
            .filter(|secret| {
                if let Some(expires_at) = secret.expires_at {
                    expires_at <= threshold && expires_at > Utc::now()
                } else {
                    false
                }
            })
            .collect()
    }
}

/// Pre-defined templates for common services
pub struct SecretTemplates;

impl SecretTemplates {
    pub fn aws_credentials() -> SecretTemplate {
        SecretTemplate {
            name: "AWS Credentials".to_string(),
            description: "Amazon Web Services access credentials".to_string(),
            secret_type: SecretType::CloudCredentials,
            fields: vec![
                TemplateField {
                    name: "access_key_id".to_string(),
                    field_type: FieldType::Text,
                    required: true,
                    description: Some("AWS Access Key ID".to_string()),
                    default_value: None,
                    validation: Some("^AKIA[0-9A-Z]{16}$".to_string()),
                },
                TemplateField {
                    name: "secret_access_key".to_string(),
                    field_type: FieldType::Password,
                    required: true,
                    description: Some("AWS Secret Access Key".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "region".to_string(),
                    field_type: FieldType::Text,
                    required: false,
                    description: Some("Default AWS region".to_string()),
                    default_value: Some("us-east-1".to_string()),
                    validation: None,
                },
            ],
            tags: vec!["aws".to_string(), "cloud".to_string(), "credentials".to_string()],
            metadata: HashMap::from([
                ("provider".to_string(), "AWS".to_string()),
                ("docs_url".to_string(), "https://docs.aws.amazon.com/".to_string()),
            ]),
        }
    }
    
    pub fn database_connection() -> SecretTemplate {
        SecretTemplate {
            name: "Database Connection".to_string(),
            description: "Database connection credentials and configuration".to_string(),
            secret_type: SecretType::ConnectionString,
            fields: vec![
                TemplateField {
                    name: "host".to_string(),
                    field_type: FieldType::Text,
                    required: true,
                    description: Some("Database host or IP address".to_string()),
                    default_value: Some("localhost".to_string()),
                    validation: None,
                },
                TemplateField {
                    name: "port".to_string(),
                    field_type: FieldType::Number,
                    required: false,
                    description: Some("Database port number".to_string()),
                    default_value: Some("5432".to_string()),
                    validation: None,
                },
                TemplateField {
                    name: "database".to_string(),
                    field_type: FieldType::Text,
                    required: true,
                    description: Some("Database name".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "username".to_string(),
                    field_type: FieldType::Text,
                    required: true,
                    description: Some("Database username".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "password".to_string(),
                    field_type: FieldType::Password,
                    required: true,
                    description: Some("Database password".to_string()),
                    default_value: None,
                    validation: None,
                },
            ],
            tags: vec!["database".to_string(), "connection".to_string()],
            metadata: HashMap::new(),
        }
    }
    
    pub fn ssh_key() -> SecretTemplate {
        SecretTemplate {
            name: "SSH Key".to_string(),
            description: "SSH private/public key pair for server access".to_string(),
            secret_type: SecretType::SshKey,
            fields: vec![
                TemplateField {
                    name: "key_type".to_string(),
                    field_type: FieldType::Text,
                    required: true,
                    description: Some("SSH key type (rsa, ed25519, ecdsa)".to_string()),
                    default_value: Some("ed25519".to_string()),
                    validation: None,
                },
                TemplateField {
                    name: "private_key".to_string(),
                    field_type: FieldType::TextArea,
                    required: true,
                    description: Some("SSH private key (PEM format)".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "public_key".to_string(),
                    field_type: FieldType::TextArea,
                    required: false,
                    description: Some("SSH public key".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "passphrase".to_string(),
                    field_type: FieldType::Password,
                    required: false,
                    description: Some("Key passphrase (if encrypted)".to_string()),
                    default_value: None,
                    validation: None,
                },
            ],
            tags: vec!["ssh".to_string(), "key".to_string(), "server".to_string()],
            metadata: HashMap::new(),
        }
    }
    
    pub fn api_key() -> SecretTemplate {
        SecretTemplate {
            name: "API Key".to_string(),
            description: "API key or token for service authentication".to_string(),
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "api_key".to_string(),
                    field_type: FieldType::Password,
                    required: true,
                    description: Some("API key or token".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "secret".to_string(),
                    field_type: FieldType::Password,
                    required: false,
                    description: Some("API secret (if required)".to_string()),
                    default_value: None,
                    validation: None,
                },
                TemplateField {
                    name: "endpoint".to_string(),
                    field_type: FieldType::Url,
                    required: false,
                    description: Some("API endpoint URL".to_string()),
                    default_value: None,
                    validation: None,
                },
            ],
            tags: vec!["api".to_string(), "key".to_string(), "token".to_string()],
            metadata: HashMap::new(),
        }
    }
    
    /// Get all available templates
    pub fn all_templates() -> Vec<SecretTemplate> {
        vec![
            Self::aws_credentials(),
            Self::database_connection(),
            Self::ssh_key(),
            Self::api_key(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secret_expiration() {
        let mut secret = SecretEntry {
            id: "test".to_string(),
            name: "Test Secret".to_string(),
            description: None,
            secret_type: SecretType::ApiKey,
            encrypted_data: vec![],
            metadata: SecretMetadata::default(),
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at: Some(Utc::now() - chrono::Duration::days(1)), // Expired yesterday
            favorite: false,
        };
        
        assert!(SecretManager::is_expired(&secret));
        
        secret.expires_at = Some(Utc::now() + chrono::Duration::days(1)); // Expires tomorrow
        assert!(!SecretManager::is_expired(&secret));
        
        secret.expires_at = None; // Never expires
        assert!(!SecretManager::is_expired(&secret));
    }
    
    #[test]
    fn test_template_creation() {
        let aws_template = SecretTemplates::aws_credentials();
        assert_eq!(aws_template.name, "AWS Credentials");
        assert_eq!(aws_template.secret_type, SecretType::CloudCredentials);
        assert_eq!(aws_template.fields.len(), 3);
    }
}