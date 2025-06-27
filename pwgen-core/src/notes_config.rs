use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{Result, Error};
use crate::secrets::{SecretData, DecryptedSecretEntry, SecretType, SecretMetadata, NoteFormat, ConfigFormat};

/// Note categories for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NoteCategory {
    /// General notes
    General,
    /// Meeting notes
    Meeting,
    /// Project documentation
    Project,
    /// Personal notes
    Personal,
    /// Technical documentation
    Technical,
    /// Ideas and brainstorming
    Ideas,
    /// TODO lists and tasks
    Todo,
    /// Reference material
    Reference,
    /// Custom category
    Custom(String),
}

/// Configuration file types and formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigType {
    /// Application configuration
    Application,
    /// Environment variables
    Environment,
    /// Service configuration
    Service,
    /// Database configuration
    Database,
    /// Web server configuration
    WebServer,
    /// API configuration
    Api,
    /// Deployment configuration
    Deployment,
    /// Custom configuration type
    Custom(String),
}

/// Configuration template for common formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigTemplate {
    pub name: String,
    pub description: String,
    pub format: ConfigFormat,
    pub variables: HashMap<String, ConfigVariable>,
    pub default_values: HashMap<String, String>,
    pub validation_rules: HashMap<String, String>,
}

/// Configuration variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigVariable {
    pub name: String,
    pub description: Option<String>,
    pub var_type: ConfigVariableType,
    pub required: bool,
    pub sensitive: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

/// Types of configuration variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigVariableType {
    String,
    Integer,
    Float,
    Boolean,
    Url,
    Email,
    Path,
    Password,
    Json,
    Array,
}

/// Note metadata for enhanced organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub category: NoteCategory,
    pub priority: NotePriority,
    pub linked_secrets: Vec<String>, // IDs of related secrets
    pub attachments: Vec<String>,    // File attachment references
    pub collaborators: Vec<String>,  // User IDs who can access
    pub version: u32,
    pub change_log: Vec<NoteChange>,
}

/// Note priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Note change tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteChange {
    pub timestamp: DateTime<Utc>,
    pub user: Option<String>,
    pub change_type: ChangeType,
    pub description: String,
}

/// Types of changes to notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Updated,
    Formatted,
    CategoryChanged,
    PriorityChanged,
    Shared,
    Archived,
}

/// Manager for secure notes and configuration files
pub struct NotesConfigManager;

impl NotesConfigManager {
    /// Create a new secure note
    pub fn create_note(
        title: String,
        content: String,
        format: NoteFormat,
        category: NoteCategory,
        priority: NotePriority,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let note_metadata = NoteMetadata {
            category,
            priority,
            linked_secrets: Vec::new(),
            attachments: Vec::new(),
            collaborators: Vec::new(),
            version: 1,
            change_log: vec![NoteChange {
                timestamp: Utc::now(),
                user: None,
                change_type: ChangeType::Created,
                description: "Note created".to_string(),
            }],
        };

        let secret_data = SecretData::SecureNote {
            title: title.clone(),
            content,
            format,
        };

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name: title,
            description,
            secret_type: SecretType::SecureNote,
            data: secret_data,
            metadata: SecretMetadata::default(),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at: None,
            favorite: false,
        })
    }

    /// Create a configuration file entry
    pub fn create_config(
        name: String,
        format: ConfigFormat,
        config_type: ConfigType,
        variables: HashMap<String, String>,
        template_name: Option<String>,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let secret_data = SecretData::Configuration {
            format,
            variables,
            template: template_name,
        };

        let mut metadata = SecretMetadata::default();
        metadata.template = Some(format!("{:?}", config_type));

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: SecretType::Configuration,
            data: secret_data,
            metadata,
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at: None,
            favorite: false,
        })
    }

    /// Update note content and track changes
    pub fn update_note(
        entry: &mut DecryptedSecretEntry,
        new_content: Option<String>,
        new_title: Option<String>,
        new_format: Option<NoteFormat>,
        user: Option<String>,
    ) -> Result<()> {
        if let SecretData::SecureNote { title, content, format } = &mut entry.data {
            let mut changes = Vec::new();

            if let Some(new_title_val) = new_title {
                if *title != new_title_val {
                    *title = new_title_val.clone();
                    entry.name = new_title_val;
                    changes.push("title updated".to_string());
                }
            }

            if let Some(new_content_val) = new_content {
                if *content != new_content_val {
                    *content = new_content_val;
                    changes.push("content updated".to_string());
                }
            }

            if let Some(new_format_val) = new_format {
                if *format != new_format_val {
                    *format = new_format_val;
                    changes.push("format changed".to_string());
                }
            }

            if !changes.is_empty() {
                entry.updated_at = Utc::now();
                // Note: In a full implementation, we'd store note metadata separately
                // For now, we just update the entry timestamp
            }
        } else {
            return Err(Error::Other("Entry is not a secure note".to_string()));
        }

        Ok(())
    }

    /// Update configuration variables
    pub fn update_config(
        entry: &mut DecryptedSecretEntry,
        new_variables: HashMap<String, String>,
        merge: bool,
    ) -> Result<()> {
        if let SecretData::Configuration { variables, .. } = &mut entry.data {
            if merge {
                // Merge new variables with existing ones
                for (key, value) in new_variables {
                    variables.insert(key, value);
                }
            } else {
                // Replace all variables
                *variables = new_variables;
            }
            entry.updated_at = Utc::now();
        } else {
            return Err(Error::Other("Entry is not a configuration".to_string()));
        }

        Ok(())
    }

    /// Convert note to different format
    pub fn convert_note_format(
        entry: &mut DecryptedSecretEntry,
        target_format: NoteFormat,
    ) -> Result<()> {
        if let SecretData::SecureNote { content, format, .. } = &mut entry.data {
            if *format == target_format {
                return Ok(()); // Already in target format
            }

            let converted_content = Self::convert_content(content, format, &target_format)?;
            *content = converted_content;
            *format = target_format;
            entry.updated_at = Utc::now();
        } else {
            return Err(Error::Other("Entry is not a secure note".to_string()));
        }

        Ok(())
    }

    /// Export configuration to file format
    pub fn export_config_to_string(
        entry: &DecryptedSecretEntry,
        target_format: Option<ConfigFormat>,
    ) -> Result<String> {
        if let SecretData::Configuration { format, variables, .. } = &entry.data {
            let export_format = target_format.unwrap_or(format.clone());
            Self::serialize_config(variables, &export_format)
        } else {
            Err(Error::Other("Entry is not a configuration".to_string()))
        }
    }

    /// Import configuration from string
    pub fn import_config_from_string(
        content: &str,
        format: ConfigFormat,
        name: String,
        config_type: ConfigType,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let variables = Self::parse_config(content, &format)?;
        
        Self::create_config(
            name,
            format,
            config_type,
            variables,
            None,
            description,
            tags,
        )
    }

    /// Get configuration templates
    pub fn get_config_templates() -> Vec<ConfigTemplate> {
        vec![
            Self::docker_compose_template(),
            Self::env_file_template(),
            Self::nginx_config_template(),
            Self::database_config_template(),
            Self::api_config_template(),
        ]
    }

    /// Search notes by content
    pub fn search_notes_content<'a>(
        entries: &'a [DecryptedSecretEntry],
        query: &str,
        case_sensitive: bool,
    ) -> Vec<&'a DecryptedSecretEntry> {
        let search_query = if case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        entries.iter()
            .filter(|entry| {
                if let SecretData::SecureNote { title, content, .. } = &entry.data {
                    let search_text = if case_sensitive {
                        format!("{} {}", title, content)
                    } else {
                        format!("{} {}", title.to_lowercase(), content.to_lowercase())
                    };
                    search_text.contains(&search_query)
                } else {
                    false
                }
            })
            .collect()
    }

    /// Validate configuration against template
    pub fn validate_config(
        variables: &HashMap<String, String>,
        template: &ConfigTemplate,
    ) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        // Check required variables
        for (var_name, var_def) in &template.variables {
            if var_def.required && !variables.contains_key(var_name) {
                errors.push(format!("Required variable '{}' is missing", var_name));
            }
        }

        // Validate variable formats
        for (var_name, value) in variables {
            if let Some(var_def) = template.variables.get(var_name) {
                if let Some(pattern) = &var_def.validation_pattern {
                    let regex = regex::Regex::new(pattern)
                        .map_err(|e| Error::Other(format!("Invalid regex pattern: {}", e)))?;
                    if !regex.is_match(value) {
                        errors.push(format!("Variable '{}' does not match required pattern", var_name));
                    }
                }
            }
        }

        Ok(errors)
    }

    // Helper methods

    fn convert_content(
        content: &str,
        from_format: &NoteFormat,
        to_format: &NoteFormat,
    ) -> Result<String> {
        match (from_format, to_format) {
            (NoteFormat::PlainText, NoteFormat::Markdown) => {
                // Simple conversion: wrap in code blocks if it looks like code
                if content.contains('\n') && (content.contains('{') || content.contains('<')) {
                    Ok(format!("```\n{}\n```", content))
                } else {
                    Ok(content.to_string())
                }
            }
            (NoteFormat::Markdown, NoteFormat::PlainText) => {
                // Strip markdown formatting (basic implementation)
                let mut plain_text = content.to_string();
                plain_text = plain_text.replace("**", "");
                plain_text = plain_text.replace("*", "");
                plain_text = plain_text.replace("```", "");
                plain_text = plain_text.replace("#", "");
                Ok(plain_text)
            }
            (NoteFormat::Markdown, NoteFormat::Html) => {
                // Basic markdown to HTML conversion
                let mut html = content.to_string();
                html = html.replace("\n\n", "</p><p>");
                html = html.replace("\n", "<br>");
                html = format!("<p>{}</p>", html);
                Ok(html)
            }
            _ => {
                // For unsupported conversions, return content as-is
                Ok(content.to_string())
            }
        }
    }

    fn serialize_config(
        variables: &HashMap<String, String>,
        format: &ConfigFormat,
    ) -> Result<String> {
        match format {
            ConfigFormat::EnvFile => {
                let mut output = String::new();
                for (key, value) in variables {
                    // Quote values that contain spaces or special characters
                    if value.contains(' ') || value.contains('$') || value.contains('"') {
                        output.push_str(&format!("{}=\"{}\"\n", key, value.replace('"', "\\\"")));
                    } else {
                        output.push_str(&format!("{}={}\n", key, value));
                    }
                }
                Ok(output)
            }
            ConfigFormat::Json => {
                serde_json::to_string_pretty(variables)
                    .map_err(|e| Error::Other(format!("JSON serialization failed: {}", e)))
            }
            ConfigFormat::Yaml => {
                serde_yaml::to_string(variables)
                    .map_err(|e| Error::Other(format!("YAML serialization failed: {}", e)))
            }
            ConfigFormat::Toml => {
                toml::to_string(variables)
                    .map_err(|e| Error::Other(format!("TOML serialization failed: {}", e)))
            }
            ConfigFormat::Properties => {
                let mut output = String::new();
                for (key, value) in variables {
                    output.push_str(&format!("{}={}\n", key, value));
                }
                Ok(output)
            }
            _ => Err(Error::Other(format!("Unsupported format: {:?}", format))),
        }
    }

    pub fn parse_config(
        content: &str,
        format: &ConfigFormat,
    ) -> Result<HashMap<String, String>> {
        match format {
            ConfigFormat::EnvFile | ConfigFormat::Properties => {
                let mut variables = HashMap::new();
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    if let Some(eq_pos) = line.find('=') {
                        let key = line[..eq_pos].trim().to_string();
                        let mut value = line[eq_pos + 1..].trim().to_string();
                        
                        // Remove quotes if present
                        if value.starts_with('"') && value.ends_with('"') {
                            value = value[1..value.len()-1].to_string();
                            value = value.replace("\\\"", "\"");
                        }
                        
                        variables.insert(key, value);
                    }
                }
                Ok(variables)
            }
            ConfigFormat::Json => {
                serde_json::from_str(content)
                    .map_err(|e| Error::Other(format!("JSON parsing failed: {}", e)))
            }
            ConfigFormat::Yaml => {
                serde_yaml::from_str(content)
                    .map_err(|e| Error::Other(format!("YAML parsing failed: {}", e)))
            }
            ConfigFormat::Toml => {
                toml::from_str(content)
                    .map_err(|e| Error::Other(format!("TOML parsing failed: {}", e)))
            }
            _ => Err(Error::Other(format!("Unsupported format: {:?}", format))),
        }
    }

    // Configuration templates

    fn docker_compose_template() -> ConfigTemplate {
        let mut variables = HashMap::new();
        variables.insert("app_name".to_string(), ConfigVariable {
            name: "app_name".to_string(),
            description: Some("Application name".to_string()),
            var_type: ConfigVariableType::String,
            required: true,
            sensitive: false,
            default_value: Some("myapp".to_string()),
            validation_pattern: Some(r"^[a-z0-9_-]+$".to_string()),
        });
        variables.insert("app_port".to_string(), ConfigVariable {
            name: "app_port".to_string(),
            description: Some("Application port".to_string()),
            var_type: ConfigVariableType::Integer,
            required: true,
            sensitive: false,
            default_value: Some("3000".to_string()),
            validation_pattern: Some(r"^[0-9]+$".to_string()),
        });

        ConfigTemplate {
            name: "Docker Compose".to_string(),
            description: "Docker Compose environment variables".to_string(),
            format: ConfigFormat::EnvFile,
            variables,
            default_values: HashMap::new(),
            validation_rules: HashMap::new(),
        }
    }

    fn env_file_template() -> ConfigTemplate {
        let mut variables = HashMap::new();
        variables.insert("NODE_ENV".to_string(), ConfigVariable {
            name: "NODE_ENV".to_string(),
            description: Some("Node.js environment".to_string()),
            var_type: ConfigVariableType::String,
            required: true,
            sensitive: false,
            default_value: Some("production".to_string()),
            validation_pattern: Some(r"^(development|production|test)$".to_string()),
        });
        variables.insert("DATABASE_URL".to_string(), ConfigVariable {
            name: "DATABASE_URL".to_string(),
            description: Some("Database connection URL".to_string()),
            var_type: ConfigVariableType::Url,
            required: true,
            sensitive: true,
            default_value: None,
            validation_pattern: None,
        });

        ConfigTemplate {
            name: "Environment File".to_string(),
            description: "Standard .env file format".to_string(),
            format: ConfigFormat::EnvFile,
            variables,
            default_values: HashMap::new(),
            validation_rules: HashMap::new(),
        }
    }

    fn nginx_config_template() -> ConfigTemplate {
        ConfigTemplate {
            name: "Nginx Configuration".to_string(),
            description: "Nginx server configuration".to_string(),
            format: ConfigFormat::Custom("nginx".to_string()),
            variables: HashMap::new(),
            default_values: HashMap::new(),
            validation_rules: HashMap::new(),
        }
    }

    fn database_config_template() -> ConfigTemplate {
        let mut variables = HashMap::new();
        variables.insert("DB_HOST".to_string(), ConfigVariable {
            name: "DB_HOST".to_string(),
            description: Some("Database host".to_string()),
            var_type: ConfigVariableType::String,
            required: true,
            sensitive: false,
            default_value: Some("localhost".to_string()),
            validation_pattern: None,
        });
        variables.insert("DB_PASSWORD".to_string(), ConfigVariable {
            name: "DB_PASSWORD".to_string(),
            description: Some("Database password".to_string()),
            var_type: ConfigVariableType::Password,
            required: true,
            sensitive: true,
            default_value: None,
            validation_pattern: None,
        });

        ConfigTemplate {
            name: "Database Configuration".to_string(),
            description: "Database connection configuration".to_string(),
            format: ConfigFormat::EnvFile,
            variables,
            default_values: HashMap::new(),
            validation_rules: HashMap::new(),
        }
    }

    fn api_config_template() -> ConfigTemplate {
        let mut variables = HashMap::new();
        variables.insert("API_BASE_URL".to_string(), ConfigVariable {
            name: "API_BASE_URL".to_string(),
            description: Some("API base URL".to_string()),
            var_type: ConfigVariableType::Url,
            required: true,
            sensitive: false,
            default_value: Some("https://api.example.com".to_string()),
            validation_pattern: Some(r"^https?://".to_string()),
        });

        ConfigTemplate {
            name: "API Configuration".to_string(),
            description: "API service configuration".to_string(),
            format: ConfigFormat::Json,
            variables,
            default_values: HashMap::new(),
            validation_rules: HashMap::new(),
        }
    }
}

impl Default for NoteMetadata {
    fn default() -> Self {
        Self {
            category: NoteCategory::General,
            priority: NotePriority::Medium,
            linked_secrets: Vec::new(),
            attachments: Vec::new(),
            collaborators: Vec::new(),
            version: 1,
            change_log: Vec::new(),
        }
    }
}

impl std::fmt::Display for NoteCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteCategory::General => write!(f, "general"),
            NoteCategory::Meeting => write!(f, "meeting"),
            NoteCategory::Project => write!(f, "project"),
            NoteCategory::Personal => write!(f, "personal"),
            NoteCategory::Technical => write!(f, "technical"),
            NoteCategory::Ideas => write!(f, "ideas"),
            NoteCategory::Todo => write!(f, "todo"),
            NoteCategory::Reference => write!(f, "reference"),
            NoteCategory::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl std::str::FromStr for NoteCategory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "general" => Ok(NoteCategory::General),
            "meeting" => Ok(NoteCategory::Meeting),
            "project" => Ok(NoteCategory::Project),
            "personal" => Ok(NoteCategory::Personal),
            "technical" => Ok(NoteCategory::Technical),
            "ideas" => Ok(NoteCategory::Ideas),
            "todo" => Ok(NoteCategory::Todo),
            "reference" => Ok(NoteCategory::Reference),
            s if s.starts_with("custom_") => {
                let name = s.strip_prefix("custom_").unwrap_or(s);
                Ok(NoteCategory::Custom(name.to_string()))
            }
            _ => Err(Error::Other(format!("Unknown note category: {}", s))),
        }
    }
}

impl std::fmt::Display for ConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigType::Application => write!(f, "application"),
            ConfigType::Environment => write!(f, "environment"),
            ConfigType::Service => write!(f, "service"),
            ConfigType::Database => write!(f, "database"),
            ConfigType::WebServer => write!(f, "webserver"),
            ConfigType::Api => write!(f, "api"),
            ConfigType::Deployment => write!(f, "deployment"),
            ConfigType::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl std::str::FromStr for ConfigType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "application" => Ok(ConfigType::Application),
            "environment" => Ok(ConfigType::Environment),
            "service" => Ok(ConfigType::Service),
            "database" => Ok(ConfigType::Database),
            "webserver" => Ok(ConfigType::WebServer),
            "api" => Ok(ConfigType::Api),
            "deployment" => Ok(ConfigType::Deployment),
            s if s.starts_with("custom_") => {
                let name = s.strip_prefix("custom_").unwrap_or(s);
                Ok(ConfigType::Custom(name.to_string()))
            }
            _ => Err(Error::Other(format!("Unknown config type: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = NotesConfigManager::create_note(
            "Test Note".to_string(),
            "This is a test note content".to_string(),
            NoteFormat::Markdown,
            NoteCategory::Technical,
            NotePriority::High,
            Some("A test note".to_string()),
            vec!["test".to_string(), "note".to_string()],
        ).unwrap();

        assert_eq!(note.name, "Test Note");
        assert_eq!(note.secret_type, SecretType::SecureNote);
        assert!(matches!(note.data, SecretData::SecureNote { .. }));
    }

    #[test]
    fn test_config_creation() {
        let mut variables = HashMap::new();
        variables.insert("API_KEY".to_string(), "test-key".to_string());
        variables.insert("BASE_URL".to_string(), "https://api.example.com".to_string());

        let config = NotesConfigManager::create_config(
            "API Config".to_string(),
            ConfigFormat::EnvFile,
            ConfigType::Api,
            variables,
            None,
            Some("API configuration".to_string()),
            vec!["api".to_string(), "config".to_string()],
        ).unwrap();

        assert_eq!(config.name, "API Config");
        assert_eq!(config.secret_type, SecretType::Configuration);
        assert!(matches!(config.data, SecretData::Configuration { .. }));
    }

    #[test]
    fn test_env_file_parsing() {
        let content = r#"
# This is a comment
API_KEY=test-key-123
BASE_URL="https://api.example.com"
DEBUG=true
        "#;

        let variables = NotesConfigManager::parse_config(content, &ConfigFormat::EnvFile).unwrap();
        
        assert_eq!(variables.get("API_KEY"), Some(&"test-key-123".to_string()));
        assert_eq!(variables.get("BASE_URL"), Some(&"https://api.example.com".to_string()));
        assert_eq!(variables.get("DEBUG"), Some(&"true".to_string()));
        assert!(!variables.contains_key("# This is a comment"));
    }

    #[test]
    fn test_content_conversion() {
        let plain_text = "This is plain text";
        let markdown = NotesConfigManager::convert_content(
            plain_text,
            &NoteFormat::PlainText,
            &NoteFormat::Markdown,
        ).unwrap();

        assert_eq!(markdown, plain_text); // Simple text should remain unchanged

        let code_text = "function test() {\n  return true;\n}";
        let markdown_code = NotesConfigManager::convert_content(
            code_text,
            &NoteFormat::PlainText,
            &NoteFormat::Markdown,
        ).unwrap();

        assert!(markdown_code.contains("```"));
    }

    #[test]
    fn test_config_serialization() {
        let mut variables = HashMap::new();
        variables.insert("KEY1".to_string(), "value1".to_string());
        variables.insert("KEY2".to_string(), "value with spaces".to_string());

        let env_output = NotesConfigManager::serialize_config(&variables, &ConfigFormat::EnvFile).unwrap();
        
        assert!(env_output.contains("KEY1=value1"));
        assert!(env_output.contains("KEY2=\"value with spaces\""));
    }
}