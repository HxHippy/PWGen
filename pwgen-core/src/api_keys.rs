use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use base64::Engine as _;

use crate::{Result, Error};
use crate::secrets::{SecretData, SecretMetadata, DecryptedSecretEntry, SecretType};

/// API key types and providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiKeyProvider {
    /// Amazon Web Services
    AWS,
    /// Google Cloud Platform
    GCP,
    /// Microsoft Azure
    Azure,
    /// GitHub
    GitHub,
    /// GitLab
    GitLab,
    /// Docker Hub
    DockerHub,
    /// Stripe
    Stripe,
    /// Twilio
    Twilio,
    /// SendGrid
    SendGrid,
    /// Slack
    Slack,
    /// Discord
    Discord,
    /// OpenAI
    OpenAI,
    /// Anthropic
    Anthropic,
    /// Generic API service
    Generic,
    /// Custom provider
    Custom(String),
}

/// API key permission scopes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiKeyPermissions {
    pub read: bool,
    pub write: bool,
    pub admin: bool,
    pub scopes: Vec<String>,
    pub resource_access: HashMap<String, Vec<String>>,
}

/// API key rotation information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RotationInfo {
    pub auto_rotate: bool,
    pub rotation_period_days: Option<u32>,
    pub last_rotated: Option<DateTime<Utc>>,
    pub next_rotation: Option<DateTime<Utc>>,
    pub rotation_reminder_days: Option<u32>,
}

/// API key usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageStats {
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub rate_limit_info: Option<RateLimitInfo>,
    pub error_count: u64,
    pub last_error: Option<String>,
}

/// Rate limiting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub requests_per_minute: Option<u32>,
    pub requests_per_hour: Option<u32>,
    pub requests_per_day: Option<u32>,
    pub current_usage: u32,
    pub reset_time: Option<DateTime<Utc>>,
}

/// API key management utilities
pub struct ApiKeyManager;

impl ApiKeyManager {
    /// Create a new API key entry
    pub fn create_api_key(
        name: String,
        provider: ApiKeyProvider,
        api_key: String,
        api_secret: Option<String>,
        description: Option<String>,
        permissions: Option<ApiKeyPermissions>,
        expires_at: Option<DateTime<Utc>>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let secret_data = SecretData::ApiKey {
            provider: provider.clone(),
            key_id: format!("{}_{}", provider.to_string().to_lowercase(), Uuid::new_v4()),
            api_key,
            api_secret,
            token_type: "Bearer".to_string(),
            permissions: permissions.unwrap_or_default(),
            environment: "production".to_string(),
            endpoint_url: Self::default_endpoint_for_provider(&provider),
            rotation_info: RotationInfo {
                auto_rotate: false,
                rotation_period_days: None,
                last_rotated: None,
                next_rotation: None,
                rotation_reminder_days: Some(30),
            },
            usage_stats: UsageStats {
                last_used: None,
                usage_count: 0,
                rate_limit_info: None,
                error_count: 0,
                last_error: None,
            },
        };

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: SecretType::ApiKey,
            data: secret_data,
            metadata: SecretMetadata::default(),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at,
            favorite: false,
        })
    }

    /// Create a JWT token entry
    pub fn create_jwt_token(
        name: String,
        token: String,
        issuer: Option<String>,
        audience: Option<String>,
        description: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        // Parse JWT to extract information if possible
        let (subject, claims) = Self::parse_jwt_claims(&token)?;

        let secret_data = SecretData::Token {
            token_type: "JWT".to_string(),
            access_token: token,
            refresh_token: None,
            token_secret: None,
            expires_at,
            issued_at: Some(Utc::now()),
            issuer,
            audience,
            subject,
            scopes: Vec::new(),
            claims,
        };

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: SecretType::Token,
            data: secret_data,
            metadata: SecretMetadata::default(),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at,
            favorite: false,
        })
    }

    /// Create an OAuth token entry
    pub fn create_oauth_token(
        name: String,
        access_token: String,
        refresh_token: Option<String>,
        token_secret: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        scopes: Vec<String>,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let secret_data = SecretData::Token {
            token_type: "OAuth".to_string(),
            access_token,
            refresh_token,
            token_secret,
            expires_at,
            issued_at: Some(Utc::now()),
            issuer: None,
            audience: None,
            subject: None,
            scopes,
            claims: HashMap::new(),
        };

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: SecretType::Token,
            data: secret_data,
            metadata: SecretMetadata::default(),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at,
            favorite: false,
        })
    }

    /// Check if an API key or token is expired
    pub fn is_expired(entry: &DecryptedSecretEntry) -> bool {
        if let Some(expires_at) = entry.expires_at {
            return Utc::now() > expires_at;
        }

        // Check token-specific expiration
        match &entry.data {
            SecretData::Token { expires_at: Some(token_expires), .. } => {
                Utc::now() > *token_expires
            }
            _ => false,
        }
    }

    /// Check if an API key or token is expiring soon
    pub fn is_expiring_soon(entry: &DecryptedSecretEntry, warning_days: u32) -> bool {
        let warning_time = Utc::now() + Duration::days(warning_days as i64);

        if let Some(expires_at) = entry.expires_at {
            return expires_at < warning_time;
        }

        match &entry.data {
            SecretData::Token { expires_at: Some(token_expires), .. } => {
                *token_expires < warning_time
            }
            _ => false,
        }
    }

    /// Update API key usage statistics
    pub fn update_usage_stats(
        entry: &mut DecryptedSecretEntry,
        success: bool,
        error_message: Option<String>,
    ) -> Result<()> {
        match &mut entry.data {
            SecretData::ApiKey { usage_stats, .. } => {
                usage_stats.last_used = Some(Utc::now());
                usage_stats.usage_count += 1;
                
                if !success {
                    usage_stats.error_count += 1;
                    if let Some(error) = error_message {
                        usage_stats.last_error = Some(error);
                    }
                }
            }
            _ => return Err(Error::Other("Entry is not an API key".to_string())),
        }

        entry.last_accessed = Some(Utc::now());
        entry.updated_at = Utc::now();
        Ok(())
    }

    /// Set up automatic rotation for an API key
    pub fn setup_rotation(
        entry: &mut DecryptedSecretEntry,
        rotation_period_days: u32,
        reminder_days: u32,
    ) -> Result<()> {
        match &mut entry.data {
            SecretData::ApiKey { rotation_info, .. } => {
                rotation_info.auto_rotate = true;
                rotation_info.rotation_period_days = Some(rotation_period_days);
                rotation_info.rotation_reminder_days = Some(reminder_days);
                rotation_info.next_rotation = Some(Utc::now() + Duration::days(rotation_period_days as i64));
            }
            _ => return Err(Error::Other("Entry is not an API key".to_string())),
        }

        entry.updated_at = Utc::now();
        Ok(())
    }

    /// Get API keys that need rotation
    pub fn get_keys_needing_rotation(entries: &[DecryptedSecretEntry]) -> Vec<&DecryptedSecretEntry> {
        entries.iter()
            .filter(|entry| {
                if let SecretData::ApiKey { rotation_info, .. } = &entry.data {
                    if let Some(next_rotation) = rotation_info.next_rotation {
                        return Utc::now() >= next_rotation;
                    }
                }
                false
            })
            .collect()
    }

    /// Get API keys expiring soon
    pub fn get_expiring_keys(entries: &[DecryptedSecretEntry], warning_days: u32) -> Vec<&DecryptedSecretEntry> {
        entries.iter()
            .filter(|entry| Self::is_expiring_soon(entry, warning_days))
            .collect()
    }

    /// Validate API key format for a provider
    pub fn validate_api_key_format(provider: &ApiKeyProvider, api_key: &str) -> Result<()> {
        match provider {
            ApiKeyProvider::AWS => {
                if !api_key.starts_with("AKIA") || api_key.len() != 20 {
                    return Err(Error::Other("Invalid AWS access key format".to_string()));
                }
            }
            ApiKeyProvider::GitHub => {
                if !api_key.starts_with("ghp_") && !api_key.starts_with("github_pat_") {
                    return Err(Error::Other("Invalid GitHub token format".to_string()));
                }
            }
            ApiKeyProvider::Stripe => {
                if !api_key.starts_with("sk_") && !api_key.starts_with("pk_") {
                    return Err(Error::Other("Invalid Stripe key format".to_string()));
                }
            }
            ApiKeyProvider::OpenAI => {
                if !api_key.starts_with("sk-") {
                    return Err(Error::Other("Invalid OpenAI API key format".to_string()));
                }
            }
            _ => {} // No specific validation for other providers
        }
        
        Ok(())
    }

    /// Get default endpoint URL for a provider
    fn default_endpoint_for_provider(provider: &ApiKeyProvider) -> Option<String> {
        match provider {
            ApiKeyProvider::AWS => Some("https://aws.amazon.com/".to_string()),
            ApiKeyProvider::GCP => Some("https://cloud.google.com/".to_string()),
            ApiKeyProvider::Azure => Some("https://portal.azure.com/".to_string()),
            ApiKeyProvider::GitHub => Some("https://api.github.com/".to_string()),
            ApiKeyProvider::GitLab => Some("https://gitlab.com/api/v4/".to_string()),
            ApiKeyProvider::DockerHub => Some("https://hub.docker.com/".to_string()),
            ApiKeyProvider::Stripe => Some("https://api.stripe.com/".to_string()),
            ApiKeyProvider::Twilio => Some("https://api.twilio.com/".to_string()),
            ApiKeyProvider::SendGrid => Some("https://api.sendgrid.com/".to_string()),
            ApiKeyProvider::Slack => Some("https://slack.com/api/".to_string()),
            ApiKeyProvider::Discord => Some("https://discord.com/api/".to_string()),
            ApiKeyProvider::OpenAI => Some("https://api.openai.com/".to_string()),
            ApiKeyProvider::Anthropic => Some("https://api.anthropic.com/".to_string()),
            _ => None,
        }
    }

    /// Simple JWT parsing (without full validation)
    fn parse_jwt_claims(token: &str) -> Result<(Option<String>, HashMap<String, String>)> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(Error::Other("Invalid JWT format".to_string()));
        }

        // Decode payload (second part)
        let payload = parts[1];
        
        // Add padding if needed for base64 decoding
        let padded_payload = match payload.len() % 4 {
            0 => payload.to_string(),
            2 => format!("{}==", payload),
            3 => format!("{}=", payload),
            _ => return Err(Error::Other("Invalid base64 in JWT payload".to_string())),
        };

        let decoded = base64::engine::general_purpose::STANDARD.decode(&padded_payload)
            .map_err(|_| Error::Other("Failed to decode JWT payload".to_string()))?;
        
        let payload_str = String::from_utf8(decoded)
            .map_err(|_| Error::Other("Invalid UTF-8 in JWT payload".to_string()))?;

        let payload_json: serde_json::Value = serde_json::from_str(&payload_str)
            .map_err(|_| Error::Other("Invalid JSON in JWT payload".to_string()))?;

        let mut claims = HashMap::new();
        let mut subject = None;

        if let serde_json::Value::Object(map) = payload_json {
            for (key, value) in map {
                if key == "sub" {
                    subject = value.as_str().map(|s| s.to_string());
                }
                claims.insert(key, value.to_string());
            }
        }

        Ok((subject, claims))
    }
}

impl Default for ApiKeyPermissions {
    fn default() -> Self {
        Self {
            read: true,
            write: false,
            admin: false,
            scopes: Vec::new(),
            resource_access: HashMap::new(),
        }
    }
}

impl std::fmt::Display for ApiKeyProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiKeyProvider::AWS => write!(f, "aws"),
            ApiKeyProvider::GCP => write!(f, "gcp"),
            ApiKeyProvider::Azure => write!(f, "azure"),
            ApiKeyProvider::GitHub => write!(f, "github"),
            ApiKeyProvider::GitLab => write!(f, "gitlab"),
            ApiKeyProvider::DockerHub => write!(f, "dockerhub"),
            ApiKeyProvider::Stripe => write!(f, "stripe"),
            ApiKeyProvider::Twilio => write!(f, "twilio"),
            ApiKeyProvider::SendGrid => write!(f, "sendgrid"),
            ApiKeyProvider::Slack => write!(f, "slack"),
            ApiKeyProvider::Discord => write!(f, "discord"),
            ApiKeyProvider::OpenAI => write!(f, "openai"),
            ApiKeyProvider::Anthropic => write!(f, "anthropic"),
            ApiKeyProvider::Generic => write!(f, "generic"),
            ApiKeyProvider::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl std::str::FromStr for ApiKeyProvider {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "aws" => Ok(ApiKeyProvider::AWS),
            "gcp" => Ok(ApiKeyProvider::GCP),
            "azure" => Ok(ApiKeyProvider::Azure),
            "github" => Ok(ApiKeyProvider::GitHub),
            "gitlab" => Ok(ApiKeyProvider::GitLab),
            "dockerhub" => Ok(ApiKeyProvider::DockerHub),
            "stripe" => Ok(ApiKeyProvider::Stripe),
            "twilio" => Ok(ApiKeyProvider::Twilio),
            "sendgrid" => Ok(ApiKeyProvider::SendGrid),
            "slack" => Ok(ApiKeyProvider::Slack),
            "discord" => Ok(ApiKeyProvider::Discord),
            "openai" => Ok(ApiKeyProvider::OpenAI),
            "anthropic" => Ok(ApiKeyProvider::Anthropic),
            "generic" => Ok(ApiKeyProvider::Generic),
            s if s.starts_with("custom_") => {
                let name = s.strip_prefix("custom_").unwrap_or(s);
                Ok(ApiKeyProvider::Custom(name.to_string()))
            }
            _ => Err(Error::Other(format!("Unknown API key provider: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_creation() {
        let api_key = ApiKeyManager::create_api_key(
            "Test AWS Key".to_string(),
            ApiKeyProvider::AWS,
            "AKIAIOSFODNN7EXAMPLE".to_string(),
            Some("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string()),
            Some("Test AWS credentials".to_string()),
            None,
            None,
            vec!["aws".to_string(), "production".to_string()],
        ).unwrap();

        assert_eq!(api_key.name, "Test AWS Key");
        assert_eq!(api_key.secret_type, SecretType::ApiKey);
        assert!(matches!(api_key.data, SecretData::ApiKey { .. }));
    }

    #[test]
    fn test_jwt_creation() {
        let jwt = ApiKeyManager::create_jwt_token(
            "Test JWT".to_string(),
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string(),
            Some("test-issuer".to_string()),
            None,
            Some("Test JWT token".to_string()),
            None,
            vec!["auth".to_string()],
        ).unwrap();

        assert_eq!(jwt.name, "Test JWT");
        assert_eq!(jwt.secret_type, SecretType::Token);
        assert!(matches!(jwt.data, SecretData::Token { .. }));
    }

    #[test]
    fn test_provider_parsing() {
        assert_eq!("aws".parse::<ApiKeyProvider>().unwrap(), ApiKeyProvider::AWS);
        assert_eq!("github".parse::<ApiKeyProvider>().unwrap(), ApiKeyProvider::GitHub);
        assert_eq!("custom_myservice".parse::<ApiKeyProvider>().unwrap(), ApiKeyProvider::Custom("myservice".to_string()));
    }

    #[test]
    fn test_api_key_validation() {
        assert!(ApiKeyManager::validate_api_key_format(&ApiKeyProvider::AWS, "AKIA_SAMPLE_NOT_REAL123").is_ok());
        assert!(ApiKeyManager::validate_api_key_format(&ApiKeyProvider::AWS, "invalid").is_err());
        assert!(ApiKeyManager::validate_api_key_format(&ApiKeyProvider::GitHub, "ghp_1234567890abcdef").is_ok());
        assert!(ApiKeyManager::validate_api_key_format(&ApiKeyProvider::Stripe, "sk_test_SAMPLE_NOT_REAL_KEY123").is_ok());
    }
}