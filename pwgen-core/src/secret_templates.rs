use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{Result, Error};
use crate::secrets::{SecretData, DecryptedSecretEntry, SecretType, SecretMetadata};
use crate::api_keys::ApiKeyProvider;
use crate::env_connections::{EnvVariable, EnvVarType};

/// Template category for organizing secret templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateCategory {
    CloudProvider,
    Database,
    ContainerRegistry,
    VersionControl,
    ApiService,
    CiCd,
    Monitoring,
    Communication,
    Custom(String),
}

/// Template for creating secrets with predefined configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub secret_type: SecretType,
    pub fields: Vec<TemplateField>,
    pub environment_variables: Option<HashMap<String, EnvVariable>>,
    pub metadata_defaults: SecretMetadata,
    pub tags: Vec<String>,
    pub documentation_url: Option<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Field definition for template inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateField {
    pub name: String,
    pub field_type: FieldType,
    pub description: String,
    pub required: bool,
    pub sensitive: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
}

/// Field types for template inputs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Text,
    Password,
    Email,
    Url,
    Number,
    Select(Vec<String>),
    MultiSelect(Vec<String>),
    Boolean,
    Json,
    Base64,
    File,
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Text => write!(f, "text"),
            FieldType::Password => write!(f, "password"),
            FieldType::Email => write!(f, "email"),
            FieldType::Url => write!(f, "url"),
            FieldType::Number => write!(f, "number"),
            FieldType::Select(options) => write!(f, "select({})", options.join(", ")),
            FieldType::MultiSelect(options) => write!(f, "multi-select({})", options.join(", ")),
            FieldType::Boolean => write!(f, "boolean"),
            FieldType::Json => write!(f, "json"),
            FieldType::Base64 => write!(f, "base64"),
            FieldType::File => write!(f, "file"),
        }
    }
}

/// Validation rule for template fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub field_name: String,
    pub rule_type: ValidationRuleType,
    pub message: String,
}

/// Types of validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    RequiredIf(String), // Required if another field has a value
    OneOf(Vec<String>),
    Custom(String), // Custom validation function name
}

/// Manager for secret templates
pub struct SecretTemplateManager;

impl SecretTemplateManager {
    /// Get all available secret templates
    pub fn get_all_templates() -> Vec<SecretTemplate> {
        vec![
            Self::aws_credentials_template(),
            Self::aws_s3_template(),
            Self::gcp_service_account_template(),
            Self::azure_service_principal_template(),
            Self::docker_registry_template(),
            Self::github_token_template(),
            Self::gitlab_token_template(),
            Self::slack_webhook_template(),
            Self::stripe_api_keys_template(),
            Self::sendgrid_api_key_template(),
            Self::twilio_credentials_template(),
            Self::jwt_secret_template(),
            Self::oauth_client_template(),
            Self::database_admin_template(),
            Self::ssl_certificate_template(),
            Self::ssh_key_template(),
            Self::vpn_credentials_template(),
            Self::monitoring_tokens_template(),
            Self::kubernetes_config_template(),
            Self::terraform_variables_template(),
        ]
    }

    /// Get templates by category
    pub fn get_templates_by_category(category: &TemplateCategory) -> Vec<SecretTemplate> {
        Self::get_all_templates()
            .into_iter()
            .filter(|template| &template.category == category)
            .collect()
    }

    /// Get template by ID
    pub fn get_template_by_id(id: &str) -> Option<SecretTemplate> {
        Self::get_all_templates()
            .into_iter()
            .find(|template| template.id == id)
    }

    /// Create a secret from template with user input
    pub fn create_secret_from_template(
        template_id: &str,
        field_values: HashMap<String, String>,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let template = Self::get_template_by_id(template_id)
            .ok_or_else(|| Error::Other(format!("Template not found: {}", template_id)))?;

        // Validate required fields
        for field in &template.fields {
            if field.required && !field_values.contains_key(&field.name) {
                return Err(Error::Other(format!("Required field '{}' is missing", field.name)));
            }
        }

        // Validate field values
        Self::validate_field_values(&template, &field_values)?;

        // Create secret data based on template type
        let secret_data = Self::build_secret_data(&template, &field_values)?;

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: template.secret_type.clone(),
            data: secret_data,
            metadata: template.metadata_defaults.clone(),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at: None,
            favorite: false,
        })
    }

    /// Export template as JSON
    pub fn export_template(template: &SecretTemplate) -> Result<String> {
        serde_json::to_string_pretty(template)
            .map_err(|e| Error::Other(format!("Failed to export template: {}", e)))
    }

    /// Import template from JSON
    pub fn import_template(json: &str) -> Result<SecretTemplate> {
        serde_json::from_str(json)
            .map_err(|e| Error::Other(format!("Failed to import template: {}", e)))
    }

    // Helper methods

    fn validate_field_values(
        template: &SecretTemplate,
        field_values: &HashMap<String, String>,
    ) -> Result<()> {
        for rule in &template.validation_rules {
            let field_value = field_values.get(&rule.field_name);
            
            match &rule.rule_type {
                ValidationRuleType::MinLength(min_len) => {
                    if let Some(value) = field_value {
                        if value.len() < *min_len {
                            return Err(Error::Other(rule.message.clone()));
                        }
                    }
                }
                ValidationRuleType::MaxLength(max_len) => {
                    if let Some(value) = field_value {
                        if value.len() > *max_len {
                            return Err(Error::Other(rule.message.clone()));
                        }
                    }
                }
                ValidationRuleType::Pattern(pattern) => {
                    if let Some(value) = field_value {
                        if let Ok(regex) = regex::Regex::new(pattern) {
                            if !regex.is_match(value) {
                                return Err(Error::Other(rule.message.clone()));
                            }
                        }
                    }
                }
                ValidationRuleType::RequiredIf(required_field) => {
                    if field_values.contains_key(required_field) && field_value.is_none() {
                        return Err(Error::Other(rule.message.clone()));
                    }
                }
                ValidationRuleType::OneOf(valid_values) => {
                    if let Some(value) = field_value {
                        if !valid_values.contains(value) {
                            return Err(Error::Other(rule.message.clone()));
                        }
                    }
                }
                ValidationRuleType::Custom(_) => {
                    // Custom validation would be implemented based on specific needs
                }
            }
        }
        Ok(())
    }

    fn build_secret_data(
        template: &SecretTemplate,
        field_values: &HashMap<String, String>,
    ) -> Result<SecretData> {
        match template.secret_type {
            SecretType::ApiKey => {
                let provider = field_values.get("provider")
                    .and_then(|p| p.parse::<ApiKeyProvider>().ok())
                    .unwrap_or(ApiKeyProvider::Custom("unknown".to_string()));
                
                let api_key = field_values.get("api_key")
                    .or_else(|| field_values.get("access_key_id"))
                    .or_else(|| field_values.get("token"))
                    .cloned()
                    .unwrap_or_default();
                
                Ok(SecretData::ApiKey {
                    provider,
                    key_id: field_values.get("key_id").cloned().unwrap_or_default(),
                    api_key,
                    api_secret: field_values.get("api_secret")
                        .or_else(|| field_values.get("secret_access_key"))
                        .or_else(|| field_values.get("client_secret"))
                        .cloned(),
                    token_type: field_values.get("token_type").cloned().unwrap_or_else(|| "bearer".to_string()),
                    permissions: Default::default(),
                    environment: field_values.get("environment").cloned().unwrap_or_else(|| "production".to_string()),
                    endpoint_url: field_values.get("endpoint").or_else(|| field_values.get("endpoint_url")).cloned(),
                    rotation_info: Default::default(),
                    usage_stats: Default::default(),
                })
            }
            SecretType::Configuration => {
                let mut variables = HashMap::new();
                for field in &template.fields {
                    if let Some(value) = field_values.get(&field.name) {
                        variables.insert(field.name.clone(), value.clone());
                    }
                }
                
                Ok(SecretData::Configuration {
                    format: crate::secrets::ConfigFormat::EnvFile,
                    variables,
                    template: Some(template.name.clone()),
                })
            }
            SecretType::SshKey => {
                Ok(SecretData::SshKey {
                    key_type: crate::secrets::SshKeyType::Ed25519,
                    private_key: field_values.get("private_key").cloned(),
                    public_key: field_values.get("public_key").cloned(),
                    passphrase: field_values.get("passphrase").cloned(),
                    comment: field_values.get("comment").cloned(),
                    fingerprint: None,
                })
            }
            SecretType::Certificate => {
                Ok(SecretData::Certificate {
                    cert_type: crate::secrets::CertificateType::X509,
                    certificate: field_values.get("certificate").cloned().unwrap_or_default(),
                    private_key: field_values.get("private_key").cloned(),
                    ca_chain: field_values.get("certificate_chain").map(|chain| vec![chain.clone()]),
                    subject: field_values.get("subject").cloned(),
                    issuer: field_values.get("issuer").cloned(),
                })
            }
            SecretType::ConnectionString => {
                Ok(SecretData::ConnectionString {
                    database_type: crate::secrets::DatabaseType::PostgreSQL,
                    host: field_values.get("host").cloned().unwrap_or_default(),
                    port: field_values.get("port").and_then(|p| p.parse().ok()),
                    database: field_values.get("database").cloned().unwrap_or_default(),
                    username: field_values.get("username").cloned().unwrap_or_default(),
                    password: field_values.get("password").cloned().unwrap_or_default(),
                    connection_string: String::new(), // Will be built automatically
                    ssl_config: None,
                })
            }
            SecretType::SecureNote => {
                Ok(SecretData::SecureNote {
                    title: field_values.get("title").cloned().unwrap_or_else(|| "Untitled".to_string()),
                    content: field_values.get("content").cloned().unwrap_or_default(),
                    format: crate::secrets::NoteFormat::PlainText,
                })
            }
            SecretType::Document => {
                Ok(SecretData::Document {
                    filename: field_values.get("filename").cloned().unwrap_or_else(|| "document".to_string()),
                    content_type: field_values.get("content_type").cloned().unwrap_or_else(|| "text/plain".to_string()),
                    content: field_values.get("content").map(|c| c.as_bytes().to_vec()).unwrap_or_default(),
                    checksum: String::new(), // Will be calculated
                })
            }
            SecretType::Token => {
                Ok(SecretData::Token {
                    token_type: field_values.get("token_type").cloned().unwrap_or_else(|| "bearer".to_string()),
                    access_token: field_values.get("access_token").or_else(|| field_values.get("token")).cloned().unwrap_or_default(),
                    refresh_token: field_values.get("refresh_token").cloned(),
                    token_secret: field_values.get("token_secret").cloned(),
                    expires_at: None,
                    issued_at: None,
                    issuer: field_values.get("issuer").cloned(),
                    audience: field_values.get("audience").cloned(),
                    subject: field_values.get("subject").cloned(),
                    scopes: field_values.get("scopes")
                        .map(|s| s.split(',').map(|scope| scope.trim().to_string()).collect())
                        .unwrap_or_default(),
                    claims: HashMap::new(),
                })
            }
            _ => {
                // For other types, create a basic password entry
                Ok(SecretData::Password {
                    username: field_values.get("username").cloned().unwrap_or_default(),
                    password: field_values.get("password").cloned().unwrap_or_default(),
                    url: field_values.get("url").cloned(),
                    notes: field_values.get("notes").cloned(),
                })
            }
        }
    }

    // Template definitions

    fn aws_credentials_template() -> SecretTemplate {
        SecretTemplate {
            id: "aws_credentials".to_string(),
            name: "AWS Credentials".to_string(),
            description: "AWS Access Key and Secret Key for programmatic access".to_string(),
            category: TemplateCategory::CloudProvider,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "access_key_id".to_string(),
                    field_type: FieldType::Text,
                    description: "AWS Access Key ID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^AKIA[0-9A-Z]{16}$".to_string()),
                    placeholder: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
                    help_text: Some("20-character access key starting with AKIA".to_string()),
                },
                TemplateField {
                    name: "secret_access_key".to_string(),
                    field_type: FieldType::Password,
                    description: "AWS Secret Access Key".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^[A-Za-z0-9/+=]{40}$".to_string()),
                    placeholder: Some("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string()),
                    help_text: Some("40-character secret key".to_string()),
                },
                TemplateField {
                    name: "region".to_string(),
                    field_type: FieldType::Select(vec![
                        "us-east-1".to_string(), "us-east-2".to_string(), "us-west-1".to_string(), 
                        "us-west-2".to_string(), "eu-west-1".to_string(), "eu-central-1".to_string(),
                        "ap-southeast-1".to_string(), "ap-northeast-1".to_string(),
                    ]),
                    description: "Default AWS region".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("us-east-1".to_string()),
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("AWS region for API calls".to_string()),
                },
            ],
            environment_variables: Some({
                let mut env_vars = HashMap::new();
                env_vars.insert("AWS_ACCESS_KEY_ID".to_string(), EnvVariable {
                    name: "AWS_ACCESS_KEY_ID".to_string(),
                    value: "{{access_key_id}}".to_string(),
                    var_type: EnvVarType::Secret,
                    description: Some("AWS Access Key ID".to_string()),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    environment_specific: false,
                });
                env_vars.insert("AWS_SECRET_ACCESS_KEY".to_string(), EnvVariable {
                    name: "AWS_SECRET_ACCESS_KEY".to_string(),
                    value: "{{secret_access_key}}".to_string(),
                    var_type: EnvVarType::Secret,
                    description: Some("AWS Secret Access Key".to_string()),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    environment_specific: false,
                });
                env_vars
            }),
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["aws".to_string(), "cloud".to_string(), "credentials".to_string()],
            documentation_url: Some("https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html".to_string()),
            validation_rules: vec![
                ValidationRule {
                    field_name: "access_key_id".to_string(),
                    rule_type: ValidationRuleType::Pattern(r"^AKIA[0-9A-Z]{16}$".to_string()),
                    message: "AWS Access Key ID must start with AKIA and be 20 characters long".to_string(),
                },
                ValidationRule {
                    field_name: "secret_access_key".to_string(),
                    rule_type: ValidationRuleType::MinLength(40),
                    message: "AWS Secret Access Key must be 40 characters long".to_string(),
                },
            ],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn aws_s3_template() -> SecretTemplate {
        SecretTemplate {
            id: "aws_s3".to_string(),
            name: "AWS S3 Configuration".to_string(),
            description: "AWS S3 bucket configuration with access credentials".to_string(),
            category: TemplateCategory::CloudProvider,
            secret_type: SecretType::Configuration,
            fields: vec![
                TemplateField {
                    name: "bucket_name".to_string(),
                    field_type: FieldType::Text,
                    description: "S3 Bucket Name".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^[a-z0-9.-]{3,63}$".to_string()),
                    placeholder: Some("my-bucket-name".to_string()),
                    help_text: Some("Bucket name must be globally unique".to_string()),
                },
                TemplateField {
                    name: "access_key_id".to_string(),
                    field_type: FieldType::Text,
                    description: "AWS Access Key ID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^AKIA[0-9A-Z]{16}$".to_string()),
                    placeholder: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "secret_access_key".to_string(),
                    field_type: FieldType::Password,
                    description: "AWS Secret Access Key".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
                TemplateField {
                    name: "region".to_string(),
                    field_type: FieldType::Select(vec![
                        "us-east-1".to_string(), "us-west-2".to_string(), "eu-west-1".to_string(),
                    ]),
                    description: "S3 Region".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: Some("us-east-1".to_string()),
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["aws".to_string(), "s3".to_string(), "storage".to_string()],
            documentation_url: Some("https://docs.aws.amazon.com/s3/".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn gcp_service_account_template() -> SecretTemplate {
        SecretTemplate {
            id: "gcp_service_account".to_string(),
            name: "GCP Service Account".to_string(),
            description: "Google Cloud Platform service account key file".to_string(),
            category: TemplateCategory::CloudProvider,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "project_id".to_string(),
                    field_type: FieldType::Text,
                    description: "GCP Project ID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^[a-z][a-z0-9-]{4,28}[a-z0-9]$".to_string()),
                    placeholder: Some("my-project-123456".to_string()),
                    help_text: Some("Project ID must be 6-30 characters, lowercase letters, numbers, and hyphens".to_string()),
                },
                TemplateField {
                    name: "client_email".to_string(),
                    field_type: FieldType::Email,
                    description: "Service Account Email".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^.+@.+\.iam\.gserviceaccount\.com$".to_string()),
                    placeholder: Some("my-service@my-project.iam.gserviceaccount.com".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "private_key".to_string(),
                    field_type: FieldType::Password,
                    description: "Private Key (JSON format)".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Complete service account key JSON file content".to_string()),
                },
            ],
            environment_variables: Some({
                let mut env_vars = HashMap::new();
                env_vars.insert("GOOGLE_CLOUD_PROJECT".to_string(), EnvVariable {
                    name: "GOOGLE_CLOUD_PROJECT".to_string(),
                    value: "{{project_id}}".to_string(),
                    var_type: EnvVarType::String,
                    description: Some("GCP Project ID".to_string()),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    environment_specific: false,
                });
                env_vars
            }),
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["gcp".to_string(), "google".to_string(), "service-account".to_string()],
            documentation_url: Some("https://cloud.google.com/iam/docs/service-accounts".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn azure_service_principal_template() -> SecretTemplate {
        SecretTemplate {
            id: "azure_service_principal".to_string(),
            name: "Azure Service Principal".to_string(),
            description: "Azure Active Directory service principal credentials".to_string(),
            category: TemplateCategory::CloudProvider,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "tenant_id".to_string(),
                    field_type: FieldType::Text,
                    description: "Azure Tenant ID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$".to_string()),
                    placeholder: Some("12345678-1234-1234-1234-123456789012".to_string()),
                    help_text: Some("Azure AD tenant UUID".to_string()),
                },
                TemplateField {
                    name: "client_id".to_string(),
                    field_type: FieldType::Text,
                    description: "Application (Client) ID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$".to_string()),
                    placeholder: Some("87654321-4321-4321-4321-210987654321".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "client_secret".to_string(),
                    field_type: FieldType::Password,
                    description: "Client Secret".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Application secret value".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["azure".to_string(), "microsoft".to_string(), "service-principal".to_string()],
            documentation_url: Some("https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn docker_registry_template() -> SecretTemplate {
        SecretTemplate {
            id: "docker_registry".to_string(),
            name: "Docker Registry".to_string(),
            description: "Docker registry authentication credentials".to_string(),
            category: TemplateCategory::ContainerRegistry,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "registry_url".to_string(),
                    field_type: FieldType::Url,
                    description: "Registry URL".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: Some("https://index.docker.io/v1/".to_string()),
                    validation_pattern: None,
                    placeholder: Some("https://my-registry.com".to_string()),
                    help_text: Some("Docker registry endpoint URL".to_string()),
                },
                TemplateField {
                    name: "username".to_string(),
                    field_type: FieldType::Text,
                    description: "Username".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
                TemplateField {
                    name: "password".to_string(),
                    field_type: FieldType::Password,
                    description: "Password or Access Token".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Password or personal access token".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["docker".to_string(), "registry".to_string(), "container".to_string()],
            documentation_url: Some("https://docs.docker.com/registry/".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn github_token_template() -> SecretTemplate {
        SecretTemplate {
            id: "github_token".to_string(),
            name: "GitHub Personal Access Token".to_string(),
            description: "GitHub personal access token for API access".to_string(),
            category: TemplateCategory::VersionControl,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "token".to_string(),
                    field_type: FieldType::Password,
                    description: "Personal Access Token".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^gh[ps]_[a-zA-Z0-9]{36}$".to_string()),
                    placeholder: Some("ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("GitHub personal access token (classic or fine-grained)".to_string()),
                },
                TemplateField {
                    name: "username".to_string(),
                    field_type: FieldType::Text,
                    description: "GitHub Username".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
                TemplateField {
                    name: "scopes".to_string(),
                    field_type: FieldType::MultiSelect(vec![
                        "repo".to_string(), "public_repo".to_string(), "repo_deployment".to_string(),
                        "user".to_string(), "read:user".to_string(), "user:email".to_string(),
                        "admin:org".to_string(), "write:packages".to_string(), "read:packages".to_string(),
                    ]),
                    description: "Token Scopes".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Permissions granted to this token".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["github".to_string(), "git".to_string(), "token".to_string()],
            documentation_url: Some("https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn gitlab_token_template() -> SecretTemplate {
        SecretTemplate {
            id: "gitlab_token".to_string(),
            name: "GitLab Personal Access Token".to_string(),
            description: "GitLab personal access token for API access".to_string(),
            category: TemplateCategory::VersionControl,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "token".to_string(),
                    field_type: FieldType::Password,
                    description: "Personal Access Token".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^glpat-[a-zA-Z0-9_-]{20}$".to_string()),
                    placeholder: Some("glpat-xxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("GitLab personal access token".to_string()),
                },
                TemplateField {
                    name: "gitlab_url".to_string(),
                    field_type: FieldType::Url,
                    description: "GitLab Instance URL".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("https://gitlab.com".to_string()),
                    validation_pattern: None,
                    placeholder: Some("https://gitlab.example.com".to_string()),
                    help_text: Some("GitLab instance URL (leave default for gitlab.com)".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["gitlab".to_string(), "git".to_string(), "token".to_string()],
            documentation_url: Some("https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn slack_webhook_template() -> SecretTemplate {
        SecretTemplate {
            id: "slack_webhook".to_string(),
            name: "Slack Webhook".to_string(),
            description: "Slack incoming webhook for notifications".to_string(),
            category: TemplateCategory::Communication,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "webhook_url".to_string(),
                    field_type: FieldType::Url,
                    description: "Webhook URL".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^https://hooks\.slack\.com/services/[A-Z0-9]+/[A-Z0-9]+/[a-zA-Z0-9]+$".to_string()),
                    placeholder: Some("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX".to_string()),
                    help_text: Some("Slack incoming webhook URL".to_string()),
                },
                TemplateField {
                    name: "channel".to_string(),
                    field_type: FieldType::Text,
                    description: "Default Channel".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^#[a-z0-9_-]+$".to_string()),
                    placeholder: Some("#general".to_string()),
                    help_text: Some("Default channel for webhook messages".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["slack".to_string(), "webhook".to_string(), "notifications".to_string()],
            documentation_url: Some("https://api.slack.com/messaging/webhooks".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn stripe_api_keys_template() -> SecretTemplate {
        SecretTemplate {
            id: "stripe_api_keys".to_string(),
            name: "Stripe API Keys".to_string(),
            description: "Stripe payment processing API keys".to_string(),
            category: TemplateCategory::ApiService,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "publishable_key".to_string(),
                    field_type: FieldType::Text,
                    description: "Publishable Key".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^pk_(test_|live_)[a-zA-Z0-9]{24}$".to_string()),
                    placeholder: Some("pk_test_xxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("Stripe publishable key (client-side)".to_string()),
                },
                TemplateField {
                    name: "secret_key".to_string(),
                    field_type: FieldType::Password,
                    description: "Secret Key".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^sk_(test_|live_)[a-zA-Z0-9]{24}$".to_string()),
                    placeholder: Some("sk_test_xxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("Stripe secret key (server-side)".to_string()),
                },
                TemplateField {
                    name: "webhook_secret".to_string(),
                    field_type: FieldType::Password,
                    description: "Webhook Endpoint Secret".to_string(),
                    required: false,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^whsec_[a-zA-Z0-9]{32}$".to_string()),
                    placeholder: Some("whsec_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("Webhook signing secret for verifying events".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["stripe".to_string(), "payment".to_string(), "api".to_string()],
            documentation_url: Some("https://stripe.com/docs/keys".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn sendgrid_api_key_template() -> SecretTemplate {
        SecretTemplate {
            id: "sendgrid_api_key".to_string(),
            name: "SendGrid API Key".to_string(),
            description: "SendGrid email service API key".to_string(),
            category: TemplateCategory::ApiService,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "api_key".to_string(),
                    field_type: FieldType::Password,
                    description: "API Key".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}$".to_string()),
                    placeholder: Some("SG.xxxxxxxxxxxxxxxxxxxxxx.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("SendGrid API key starting with SG.".to_string()),
                },
                TemplateField {
                    name: "from_email".to_string(),
                    field_type: FieldType::Email,
                    description: "Default From Email".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("noreply@example.com".to_string()),
                    help_text: Some("Default sender email address".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["sendgrid".to_string(), "email".to_string(), "api".to_string()],
            documentation_url: Some("https://docs.sendgrid.com/ui/account-and-settings/api-keys".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn twilio_credentials_template() -> SecretTemplate {
        SecretTemplate {
            id: "twilio_credentials".to_string(),
            name: "Twilio Credentials".to_string(),
            description: "Twilio SMS/Voice service credentials".to_string(),
            category: TemplateCategory::ApiService,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "account_sid".to_string(),
                    field_type: FieldType::Text,
                    description: "Account SID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^AC[a-f0-9]{32}$".to_string()),
                    placeholder: Some("ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()),
                    help_text: Some("Twilio Account SID starting with AC".to_string()),
                },
                TemplateField {
                    name: "auth_token".to_string(),
                    field_type: FieldType::Password,
                    description: "Auth Token".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: Some(r"^[a-f0-9]{32}$".to_string()),
                    placeholder: None,
                    help_text: Some("Twilio authentication token".to_string()),
                },
                TemplateField {
                    name: "phone_number".to_string(),
                    field_type: FieldType::Text,
                    description: "Twilio Phone Number".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: Some(r"^\+[1-9]\d{1,14}$".to_string()),
                    placeholder: Some("+15551234567".to_string()),
                    help_text: Some("Twilio phone number in E.164 format".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["twilio".to_string(), "sms".to_string(), "voice".to_string()],
            documentation_url: Some("https://www.twilio.com/docs/iam/api/account".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn jwt_secret_template() -> SecretTemplate {
        SecretTemplate {
            id: "jwt_secret".to_string(),
            name: "JWT Secret".to_string(),
            description: "JSON Web Token signing secret".to_string(),
            category: TemplateCategory::ApiService,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "secret".to_string(),
                    field_type: FieldType::Password,
                    description: "JWT Secret".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Secret key for signing JWT tokens (minimum 32 characters recommended)".to_string()),
                },
                TemplateField {
                    name: "algorithm".to_string(),
                    field_type: FieldType::Select(vec![
                        "HS256".to_string(), "HS384".to_string(), "HS512".to_string(),
                        "RS256".to_string(), "RS384".to_string(), "RS512".to_string(),
                    ]),
                    description: "Signing Algorithm".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("HS256".to_string()),
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("JWT signing algorithm".to_string()),
                },
                TemplateField {
                    name: "issuer".to_string(),
                    field_type: FieldType::Text,
                    description: "Issuer (iss)".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("https://yourdomain.com".to_string()),
                    help_text: Some("JWT issuer claim".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["jwt".to_string(), "authentication".to_string(), "secret".to_string()],
            documentation_url: Some("https://jwt.io/introduction/".to_string()),
            validation_rules: vec![
                ValidationRule {
                    field_name: "secret".to_string(),
                    rule_type: ValidationRuleType::MinLength(32),
                    message: "JWT secret should be at least 32 characters for security".to_string(),
                },
            ],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn oauth_client_template() -> SecretTemplate {
        SecretTemplate {
            id: "oauth_client".to_string(),
            name: "OAuth Client".to_string(),
            description: "OAuth 2.0 client credentials".to_string(),
            category: TemplateCategory::ApiService,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "client_id".to_string(),
                    field_type: FieldType::Text,
                    description: "Client ID".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("OAuth client identifier".to_string()),
                },
                TemplateField {
                    name: "client_secret".to_string(),
                    field_type: FieldType::Password,
                    description: "Client Secret".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("OAuth client secret".to_string()),
                },
                TemplateField {
                    name: "redirect_uri".to_string(),
                    field_type: FieldType::Url,
                    description: "Redirect URI".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("https://yourapp.com/callback".to_string()),
                    help_text: Some("OAuth redirect URI".to_string()),
                },
                TemplateField {
                    name: "scopes".to_string(),
                    field_type: FieldType::Text,
                    description: "Scopes".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("read write".to_string()),
                    help_text: Some("Space-separated list of OAuth scopes".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["oauth".to_string(), "authentication".to_string(), "client".to_string()],
            documentation_url: Some("https://oauth.net/2/".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn database_admin_template() -> SecretTemplate {
        SecretTemplate {
            id: "database_admin".to_string(),
            name: "Database Admin".to_string(),
            description: "Database administrator credentials".to_string(),
            category: TemplateCategory::Database,
            secret_type: SecretType::ConnectionString,
            fields: vec![
                TemplateField {
                    name: "host".to_string(),
                    field_type: FieldType::Text,
                    description: "Database Host".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: Some("localhost".to_string()),
                    validation_pattern: None,
                    placeholder: Some("db.example.com".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "port".to_string(),
                    field_type: FieldType::Number,
                    description: "Database Port".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("5432".to_string()),
                    validation_pattern: Some(r"^\d{1,5}$".to_string()),
                    placeholder: Some("5432".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "database".to_string(),
                    field_type: FieldType::Text,
                    description: "Database Name".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("myapp_production".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "username".to_string(),
                    field_type: FieldType::Text,
                    description: "Username".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("admin".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "password".to_string(),
                    field_type: FieldType::Password,
                    description: "Password".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["database".to_string(), "admin".to_string(), "credentials".to_string()],
            documentation_url: None,
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn ssl_certificate_template() -> SecretTemplate {
        SecretTemplate {
            id: "ssl_certificate".to_string(),
            name: "SSL Certificate".to_string(),
            description: "SSL/TLS certificate and private key".to_string(),
            category: TemplateCategory::Custom("Security".to_string()),
            secret_type: SecretType::Certificate,
            fields: vec![
                TemplateField {
                    name: "certificate".to_string(),
                    field_type: FieldType::Text,
                    description: "Certificate (PEM format)".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----".to_string()),
                    help_text: Some("X.509 certificate in PEM format".to_string()),
                },
                TemplateField {
                    name: "private_key".to_string(),
                    field_type: FieldType::Password,
                    description: "Private Key (PEM format)".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----".to_string()),
                    help_text: Some("Private key in PEM format".to_string()),
                },
                TemplateField {
                    name: "certificate_chain".to_string(),
                    field_type: FieldType::Text,
                    description: "Certificate Chain (optional)".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Intermediate certificates if required".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["ssl".to_string(), "tls".to_string(), "certificate".to_string()],
            documentation_url: None,
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn ssh_key_template() -> SecretTemplate {
        SecretTemplate {
            id: "ssh_key".to_string(),
            name: "SSH Key".to_string(),
            description: "SSH public/private key pair".to_string(),
            category: TemplateCategory::Custom("Security".to_string()),
            secret_type: SecretType::SshKey,
            fields: vec![
                TemplateField {
                    name: "public_key".to_string(),
                    field_type: FieldType::Text,
                    description: "Public Key".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("ssh-rsa AAAAB3NzaC1yc2E... user@host".to_string()),
                    help_text: Some("SSH public key".to_string()),
                },
                TemplateField {
                    name: "private_key".to_string(),
                    field_type: FieldType::Password,
                    description: "Private Key".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("-----BEGIN OPENSSH PRIVATE KEY-----\n...\n-----END OPENSSH PRIVATE KEY-----".to_string()),
                    help_text: Some("SSH private key".to_string()),
                },
                TemplateField {
                    name: "comment".to_string(),
                    field_type: FieldType::Text,
                    description: "Comment/Label".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("user@hostname".to_string()),
                    help_text: Some("SSH key comment or description".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["ssh".to_string(), "key".to_string(), "authentication".to_string()],
            documentation_url: Some("https://docs.github.com/en/authentication/connecting-to-github-with-ssh".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn vpn_credentials_template() -> SecretTemplate {
        SecretTemplate {
            id: "vpn_credentials".to_string(),
            name: "VPN Credentials".to_string(),
            description: "VPN connection credentials".to_string(),
            category: TemplateCategory::Custom("Network".to_string()),
            secret_type: SecretType::Password,
            fields: vec![
                TemplateField {
                    name: "server".to_string(),
                    field_type: FieldType::Text,
                    description: "VPN Server".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("vpn.example.com".to_string()),
                    help_text: Some("VPN server hostname or IP address".to_string()),
                },
                TemplateField {
                    name: "username".to_string(),
                    field_type: FieldType::Text,
                    description: "Username".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
                TemplateField {
                    name: "password".to_string(),
                    field_type: FieldType::Password,
                    description: "Password".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
                TemplateField {
                    name: "protocol".to_string(),
                    field_type: FieldType::Select(vec![
                        "OpenVPN".to_string(), "IKEv2".to_string(), "WireGuard".to_string(),
                        "L2TP/IPSec".to_string(), "PPTP".to_string(),
                    ]),
                    description: "VPN Protocol".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("OpenVPN".to_string()),
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["vpn".to_string(), "network".to_string(), "credentials".to_string()],
            documentation_url: None,
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn monitoring_tokens_template() -> SecretTemplate {
        SecretTemplate {
            id: "monitoring_tokens".to_string(),
            name: "Monitoring Service Tokens".to_string(),
            description: "Application monitoring and observability service tokens".to_string(),
            category: TemplateCategory::Monitoring,
            secret_type: SecretType::ApiKey,
            fields: vec![
                TemplateField {
                    name: "service".to_string(),
                    field_type: FieldType::Select(vec![
                        "Datadog".to_string(), "New Relic".to_string(), "Prometheus".to_string(),
                        "Grafana".to_string(), "Sentry".to_string(), "LogRocket".to_string(),
                    ]),
                    description: "Monitoring Service".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: None,
                },
                TemplateField {
                    name: "api_key".to_string(),
                    field_type: FieldType::Password,
                    description: "API Key".to_string(),
                    required: true,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Service API key or token".to_string()),
                },
                TemplateField {
                    name: "app_key".to_string(),
                    field_type: FieldType::Password,
                    description: "Application Key (if required)".to_string(),
                    required: false,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Additional application-specific key (Datadog, etc.)".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["monitoring".to_string(), "observability".to_string(), "apm".to_string()],
            documentation_url: None,
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn kubernetes_config_template() -> SecretTemplate {
        SecretTemplate {
            id: "kubernetes_config".to_string(),
            name: "Kubernetes Config".to_string(),
            description: "Kubernetes cluster configuration and credentials".to_string(),
            category: TemplateCategory::Custom("Infrastructure".to_string()),
            secret_type: SecretType::Configuration,
            fields: vec![
                TemplateField {
                    name: "cluster_name".to_string(),
                    field_type: FieldType::Text,
                    description: "Cluster Name".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("production-cluster".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "server".to_string(),
                    field_type: FieldType::Url,
                    description: "API Server URL".to_string(),
                    required: true,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("https://kubernetes.example.com:6443".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "token".to_string(),
                    field_type: FieldType::Password,
                    description: "Service Account Token".to_string(),
                    required: false,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: None,
                    help_text: Some("Bearer token for authentication".to_string()),
                },
                TemplateField {
                    name: "namespace".to_string(),
                    field_type: FieldType::Text,
                    description: "Default Namespace".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("default".to_string()),
                    validation_pattern: Some(r"^[a-z0-9-]+$".to_string()),
                    placeholder: Some("production".to_string()),
                    help_text: None,
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["kubernetes".to_string(), "k8s".to_string(), "infrastructure".to_string()],
            documentation_url: Some("https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn terraform_variables_template() -> SecretTemplate {
        SecretTemplate {
            id: "terraform_variables".to_string(),
            name: "Terraform Variables".to_string(),
            description: "Terraform configuration variables and secrets".to_string(),
            category: TemplateCategory::Custom("Infrastructure".to_string()),
            secret_type: SecretType::Configuration,
            fields: vec![
                TemplateField {
                    name: "workspace".to_string(),
                    field_type: FieldType::Text,
                    description: "Terraform Workspace".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: Some("default".to_string()),
                    validation_pattern: None,
                    placeholder: Some("production".to_string()),
                    help_text: None,
                },
                TemplateField {
                    name: "backend_config".to_string(),
                    field_type: FieldType::Text,
                    description: "Backend Configuration".to_string(),
                    required: false,
                    sensitive: false,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("bucket=my-terraform-state".to_string()),
                    help_text: Some("Backend configuration parameters".to_string()),
                },
                TemplateField {
                    name: "variables".to_string(),
                    field_type: FieldType::Json,
                    description: "Variables (JSON)".to_string(),
                    required: false,
                    sensitive: true,
                    default_value: None,
                    validation_pattern: None,
                    placeholder: Some("{\"region\": \"us-east-1\", \"instance_type\": \"t3.micro\"}".to_string()),
                    help_text: Some("Terraform variables in JSON format".to_string()),
                },
            ],
            environment_variables: None,
            metadata_defaults: SecretMetadata::default(),
            tags: vec!["terraform".to_string(), "infrastructure".to_string(), "iac".to_string()],
            documentation_url: Some("https://www.terraform.io/docs/language/values/variables.html".to_string()),
            validation_rules: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCategory::CloudProvider => write!(f, "Cloud Provider"),
            TemplateCategory::Database => write!(f, "Database"),
            TemplateCategory::ContainerRegistry => write!(f, "Container Registry"),
            TemplateCategory::VersionControl => write!(f, "Version Control"),
            TemplateCategory::ApiService => write!(f, "API Service"),
            TemplateCategory::CiCd => write!(f, "CI/CD"),
            TemplateCategory::Monitoring => write!(f, "Monitoring"),
            TemplateCategory::Communication => write!(f, "Communication"),
            TemplateCategory::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::str::FromStr for TemplateCategory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Cloud Provider" | "cloud_provider" => Ok(TemplateCategory::CloudProvider),
            "Database" | "database" => Ok(TemplateCategory::Database),
            "Container Registry" | "container_registry" => Ok(TemplateCategory::ContainerRegistry),
            "Version Control" | "version_control" => Ok(TemplateCategory::VersionControl),
            "API Service" | "api_service" => Ok(TemplateCategory::ApiService),
            "CI/CD" | "cicd" => Ok(TemplateCategory::CiCd),
            "Monitoring" | "monitoring" => Ok(TemplateCategory::Monitoring),
            "Communication" | "communication" => Ok(TemplateCategory::Communication),
            s => Ok(TemplateCategory::Custom(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_templates() {
        let templates = SecretTemplateManager::get_all_templates();
        assert!(!templates.is_empty());
        assert!(templates.len() >= 20);
    }

    #[test]
    fn test_get_templates_by_category() {
        let cloud_templates = SecretTemplateManager::get_templates_by_category(&TemplateCategory::CloudProvider);
        assert!(!cloud_templates.is_empty());
        
        for template in cloud_templates {
            assert_eq!(template.category, TemplateCategory::CloudProvider);
        }
    }

    #[test]
    fn test_get_template_by_id() {
        let template = SecretTemplateManager::get_template_by_id("aws_credentials");
        assert!(template.is_some());
        
        let template = template.unwrap();
        assert_eq!(template.id, "aws_credentials");
        assert_eq!(template.name, "AWS Credentials");
    }

    #[test]
    fn test_create_secret_from_aws_template() {
        let mut field_values = HashMap::new();
        field_values.insert("access_key_id".to_string(), "AKIAIOSFODNN7EXAMPLE".to_string());
        field_values.insert("secret_access_key".to_string(), "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string());
        field_values.insert("region".to_string(), "us-east-1".to_string());

        let secret = SecretTemplateManager::create_secret_from_template(
            "aws_credentials",
            field_values,
            "My AWS Account".to_string(),
            Some("Production AWS credentials".to_string()),
            vec!["aws".to_string(), "production".to_string()],
        ).unwrap();

        assert_eq!(secret.name, "My AWS Account");
        assert_eq!(secret.secret_type, SecretType::ApiKey);
        assert!(secret.tags.contains(&"aws".to_string()));
    }

    #[test]
    fn test_template_export_import() {
        let template = SecretTemplateManager::get_template_by_id("github_token").unwrap();
        
        let exported = SecretTemplateManager::export_template(&template).unwrap();
        assert!(exported.contains("github_token"));
        
        let imported = SecretTemplateManager::import_template(&exported).unwrap();
        assert_eq!(imported.id, template.id);
        assert_eq!(imported.name, template.name);
    }

    #[test]
    fn test_validation_rules() {
        let mut field_values = HashMap::new();
        field_values.insert("access_key_id".to_string(), "INVALID_KEY".to_string());
        field_values.insert("secret_access_key".to_string(), "short".to_string());

        let result = SecretTemplateManager::create_secret_from_template(
            "aws_credentials",
            field_values,
            "Test".to_string(),
            None,
            vec![],
        );

        assert!(result.is_err());
    }
}