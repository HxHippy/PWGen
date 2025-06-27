use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use url::Url;

use crate::{Result, Error};
use crate::secrets::{SecretData, DecryptedSecretEntry, SecretType, SecretMetadata, DatabaseType, SslConfig};

/// Environment types for different deployment stages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentType {
    Development,
    Staging,
    Testing,
    Production,
    Local,
    Demo,
    Custom(String),
}

/// Environment variable types and their characteristics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvVarType {
    /// Plain text value
    String,
    /// Numeric value
    Number,
    /// Boolean value (true/false)
    Boolean,
    /// URL or endpoint
    Url,
    /// File path
    Path,
    /// JSON formatted string
    Json,
    /// Comma-separated list
    List,
    /// Base64 encoded value
    Base64,
    /// Secret value (password, token, etc.)
    Secret,
}

/// Environment variable definition with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
    pub var_type: EnvVarType,
    pub description: Option<String>,
    pub required: bool,
    pub sensitive: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
    pub environment_specific: bool,
}

/// Connection string types for different databases and services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    /// Database connections
    Database(DatabaseType),
    /// Message queue connections
    MessageQueue(MessageQueueType),
    /// Cache connections
    Cache(CacheType),
    /// Cloud service connections
    Cloud(CloudServiceType),
    /// Custom connection type
    Custom(String),
}

/// Message queue types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageQueueType {
    RabbitMQ,
    Redis,
    Kafka,
    NATS,
    ActiveMQ,
    AmazonSQS,
    GooglePubSub,
    Custom(String),
}

/// Cache types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheType {
    Redis,
    Memcached,
    Hazelcast,
    Ehcache,
    Custom(String),
}

/// Cloud service types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CloudServiceType {
    AWS,
    GCP,
    Azure,
    DigitalOcean,
    Heroku,
    Vercel,
    Netlify,
    Custom(String),
}

/// Connection string metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMetadata {
    pub connection_type: ConnectionType,
    pub environment: EnvironmentType,
    pub is_pooled: bool,
    pub max_connections: Option<u32>,
    pub connection_timeout: Option<u32>,
    pub read_timeout: Option<u32>,
    pub health_check_interval: Option<u32>,
    pub auto_reconnect: bool,
    pub ssl_required: bool,
    pub connection_tags: Vec<String>,
}

/// Environment set containing multiple related environment variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSet {
    pub name: String,
    pub environment_type: EnvironmentType,
    pub variables: HashMap<String, EnvVariable>,
    pub description: Option<String>,
    pub base_environment: Option<String>, // Can inherit from another environment
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Manager for environment variables and connection strings
pub struct EnvConnectionManager;

impl EnvConnectionManager {
    /// Create a new environment variable entry
    pub fn create_env_variable(
        name: String,
        var_name: String,
        value: String,
        var_type: EnvVarType,
        environment_type: EnvironmentType,
        description: Option<String>,
        tags: Vec<String>,
        sensitive: bool,
    ) -> Result<DecryptedSecretEntry> {
        let env_var = EnvVariable {
            name: var_name.clone(),
            value,
            var_type,
            description: description.clone(),
            required: true,
            sensitive,
            default_value: None,
            validation_pattern: None,
            environment_specific: true,
        };

        let mut variables = HashMap::new();
        variables.insert(var_name, env_var.value.clone());

        let secret_data = SecretData::Configuration {
            format: crate::secrets::ConfigFormat::EnvFile,
            variables,
            template: Some(format!("Environment_{:?}", environment_type)),
        };

        let mut metadata = SecretMetadata::default();
        metadata.environment = Some(format!("{:?}", environment_type));

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

    /// Create a connection string entry
    pub fn create_connection_string(
        name: String,
        connection_type: ConnectionType,
        host: String,
        port: Option<u16>,
        database: String,
        username: String,
        password: String,
        environment_type: EnvironmentType,
        ssl_config: Option<SslConfig>,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let database_type = match &connection_type {
            ConnectionType::Database(db_type) => db_type.clone(),
            _ => DatabaseType::Custom("generic".to_string()),
        };

        let connection_string = Self::build_connection_string(
            &database_type,
            &host,
            port,
            &database,
            &username,
            &password,
            &ssl_config,
        )?;

        let secret_data = SecretData::ConnectionString {
            database_type,
            host,
            port,
            database,
            username,
            password,
            connection_string,
            ssl_config,
        };

        let mut metadata = SecretMetadata::default();
        metadata.environment = Some(format!("{:?}", environment_type));

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: SecretType::ConnectionString,
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

    /// Create an environment set with multiple variables
    pub fn create_environment_set(
        name: String,
        environment_type: EnvironmentType,
        variables: Vec<EnvVariable>,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<DecryptedSecretEntry> {
        let mut var_map = HashMap::new();
        for var in variables {
            var_map.insert(var.name.clone(), var.value.clone());
        }

        let secret_data = SecretData::Configuration {
            format: crate::secrets::ConfigFormat::EnvFile,
            variables: var_map,
            template: Some(format!("EnvironmentSet_{:?}", environment_type)),
        };

        let mut metadata = SecretMetadata::default();
        metadata.environment = Some(format!("{:?}", environment_type));

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

    /// Parse a connection string to extract components
    pub fn parse_connection_string(connection_string: &str) -> Result<ConnectionComponents> {
        let url = Url::parse(connection_string)
            .map_err(|e| Error::Other(format!("Invalid connection string URL: {}", e)))?;

        let scheme = url.scheme();
        let host = url.host_str().unwrap_or("localhost").to_string();
        let port = url.port();
        let username = if url.username().is_empty() {
            None
        } else {
            Some(url.username().to_string())
        };
        let password = url.password().map(|p| p.to_string());
        let database = url.path().trim_start_matches('/').to_string();

        let database_type = Self::detect_database_type_from_scheme(scheme)?;

        Ok(ConnectionComponents {
            database_type,
            host,
            port,
            database,
            username,
            password,
            scheme: scheme.to_string(),
            query_params: url.query_pairs().into_owned().collect(),
        })
    }

    /// Generate environment file content from variables
    pub fn generate_env_file(variables: &HashMap<String, String>) -> String {
        let mut content = String::new();
        content.push_str("# Environment Variables\n");
        content.push_str(&format!("# Generated at: {}\n", Utc::now().to_rfc3339()));
        content.push_str("\n");

        for (key, value) in variables {
            // Quote values that contain spaces or special characters
            if value.contains(' ') || value.contains('$') || value.contains('"') || value.contains('=') {
                content.push_str(&format!("{}=\"{}\"\n", key, value.replace('"', "\\\"")));
            } else {
                content.push_str(&format!("{}={}\n", key, value));
            }
        }

        content
    }

    /// Validate environment variables against rules
    pub fn validate_environment_variables(
        variables: &HashMap<String, String>,
        rules: &HashMap<String, EnvVariable>,
    ) -> Vec<String> {
        let mut errors = Vec::new();

        // Check for required variables
        for (var_name, var_def) in rules {
            if var_def.required && !variables.contains_key(var_name) {
                errors.push(format!("Required environment variable '{}' is missing", var_name));
            }
        }

        // Validate variable formats
        for (var_name, value) in variables {
            if let Some(var_def) = rules.get(var_name) {
                // Type validation
                match var_def.var_type {
                    EnvVarType::Number => {
                        if value.parse::<f64>().is_err() {
                            errors.push(format!("Variable '{}' must be a number", var_name));
                        }
                    }
                    EnvVarType::Boolean => {
                        if !matches!(value.to_lowercase().as_str(), "true" | "false" | "1" | "0" | "yes" | "no") {
                            errors.push(format!("Variable '{}' must be a boolean", var_name));
                        }
                    }
                    EnvVarType::Url => {
                        if Url::parse(value).is_err() {
                            errors.push(format!("Variable '{}' must be a valid URL", var_name));
                        }
                    }
                    _ => {} // Other types don't have strict validation
                }

                // Pattern validation
                if let Some(pattern) = &var_def.validation_pattern {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        if !regex.is_match(value) {
                            errors.push(format!("Variable '{}' does not match required pattern", var_name));
                        }
                    }
                }
            }
        }

        errors
    }

    /// Get environment variable templates for common setups
    pub fn get_environment_templates() -> Vec<EnvironmentTemplate> {
        vec![
            Self::node_js_template(),
            Self::python_django_template(),
            Self::ruby_rails_template(),
            Self::java_spring_template(),
            Self::docker_template(),
            Self::kubernetes_template(),
            Self::aws_template(),
            Self::gcp_template(),
        ]
    }

    /// Test connection string validity
    pub fn test_connection_string(connection_string: &str) -> Result<bool> {
        // Parse the connection string first
        let components = Self::parse_connection_string(connection_string)?;
        
        // Basic validation - in a real implementation, we might try to connect
        if components.host.is_empty() {
            return Err(Error::Other("Host cannot be empty".to_string()));
        }
        
        if components.database.is_empty() {
            return Err(Error::Other("Database name cannot be empty".to_string()));
        }
        
        Ok(true)
    }

    // Helper methods

    fn build_connection_string(
        database_type: &DatabaseType,
        host: &str,
        port: Option<u16>,
        database: &str,
        username: &str,
        password: &str,
        ssl_config: &Option<SslConfig>,
    ) -> Result<String> {
        let default_port = Self::get_default_port(database_type);
        let actual_port = port.unwrap_or(default_port);
        
        let ssl_param = if let Some(ssl) = ssl_config {
            if ssl.enabled {
                match database_type {
                    DatabaseType::PostgreSQL => "?sslmode=require",
                    DatabaseType::MySQL => "?useSSL=true",
                    _ => "",
                }
            } else {
                ""
            }
        } else {
            ""
        };

        let connection_string = match database_type {
            DatabaseType::PostgreSQL => {
                format!("postgresql://{}:{}@{}:{}/{}{}", username, password, host, actual_port, database, ssl_param)
            }
            DatabaseType::MySQL => {
                format!("mysql://{}:{}@{}:{}/{}{}", username, password, host, actual_port, database, ssl_param)
            }
            DatabaseType::SQLite => {
                format!("sqlite:///{}", database)
            }
            DatabaseType::MongoDB => {
                format!("mongodb://{}:{}@{}:{}/{}", username, password, host, actual_port, database)
            }
            DatabaseType::Redis => {
                format!("redis://{}:{}@{}:{}/{}", username, password, host, actual_port, database)
            }
            DatabaseType::Oracle => {
                format!("oracle://{}:{}@{}:{}/{}", username, password, host, actual_port, database)
            }
            DatabaseType::SQLServer => {
                format!("sqlserver://{}:{}@{}:{}/{}", username, password, host, actual_port, database)
            }
            DatabaseType::Custom(name) => {
                format!("{}://{}:{}@{}:{}/{}", name, username, password, host, actual_port, database)
            }
        };

        Ok(connection_string)
    }

    fn get_default_port(database_type: &DatabaseType) -> u16 {
        match database_type {
            DatabaseType::PostgreSQL => 5432,
            DatabaseType::MySQL => 3306,
            DatabaseType::MongoDB => 27017,
            DatabaseType::Redis => 6379,
            DatabaseType::Oracle => 1521,
            DatabaseType::SQLServer => 1433,
            DatabaseType::SQLite => 0, // Not applicable
            DatabaseType::Custom(_) => 0,
        }
    }

    fn detect_database_type_from_scheme(scheme: &str) -> Result<DatabaseType> {
        match scheme {
            "postgresql" | "postgres" => Ok(DatabaseType::PostgreSQL),
            "mysql" => Ok(DatabaseType::MySQL),
            "sqlite" => Ok(DatabaseType::SQLite),
            "mongodb" | "mongo" => Ok(DatabaseType::MongoDB),
            "redis" => Ok(DatabaseType::Redis),
            "oracle" => Ok(DatabaseType::Oracle),
            "sqlserver" | "mssql" => Ok(DatabaseType::SQLServer),
            _ => Ok(DatabaseType::Custom(scheme.to_string())),
        }
    }

    // Environment templates

    fn node_js_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("NODE_ENV".to_string(), EnvVariable {
            name: "NODE_ENV".to_string(),
            value: "production".to_string(),
            var_type: EnvVarType::String,
            description: Some("Node.js environment".to_string()),
            required: true,
            sensitive: false,
            default_value: Some("development".to_string()),
            validation_pattern: Some(r"^(development|production|test)$".to_string()),
            environment_specific: true,
        });
        variables.insert("PORT".to_string(), EnvVariable {
            name: "PORT".to_string(),
            value: "3000".to_string(),
            var_type: EnvVarType::Number,
            description: Some("Application port".to_string()),
            required: false,
            sensitive: false,
            default_value: Some("3000".to_string()),
            validation_pattern: Some(r"^\d+$".to_string()),
            environment_specific: true,
        });

        EnvironmentTemplate {
            name: "Node.js Application".to_string(),
            description: "Standard Node.js application environment variables".to_string(),
            environment_type: EnvironmentType::Development,
            variables,
            tags: vec!["nodejs".to_string(), "javascript".to_string()],
        }
    }

    fn python_django_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("DJANGO_SETTINGS_MODULE".to_string(), EnvVariable {
            name: "DJANGO_SETTINGS_MODULE".to_string(),
            value: "myapp.settings.production".to_string(),
            var_type: EnvVarType::String,
            description: Some("Django settings module".to_string()),
            required: true,
            sensitive: false,
            default_value: Some("myapp.settings.development".to_string()),
            validation_pattern: None,
            environment_specific: true,
        });
        variables.insert("SECRET_KEY".to_string(), EnvVariable {
            name: "SECRET_KEY".to_string(),
            value: "".to_string(),
            var_type: EnvVarType::Secret,
            description: Some("Django secret key".to_string()),
            required: true,
            sensitive: true,
            default_value: None,
            validation_pattern: None,
            environment_specific: false,
        });

        EnvironmentTemplate {
            name: "Django Application".to_string(),
            description: "Django web application environment variables".to_string(),
            environment_type: EnvironmentType::Development,
            variables,
            tags: vec!["python".to_string(), "django".to_string()],
        }
    }

    fn ruby_rails_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("RAILS_ENV".to_string(), EnvVariable {
            name: "RAILS_ENV".to_string(),
            value: "production".to_string(),
            var_type: EnvVarType::String,
            description: Some("Rails environment".to_string()),
            required: true,
            sensitive: false,
            default_value: Some("development".to_string()),
            validation_pattern: Some(r"^(development|production|test)$".to_string()),
            environment_specific: true,
        });

        EnvironmentTemplate {
            name: "Ruby on Rails".to_string(),
            description: "Ruby on Rails application environment variables".to_string(),
            environment_type: EnvironmentType::Development,
            variables,
            tags: vec!["ruby".to_string(), "rails".to_string()],
        }
    }

    fn java_spring_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("SPRING_PROFILES_ACTIVE".to_string(), EnvVariable {
            name: "SPRING_PROFILES_ACTIVE".to_string(),
            value: "production".to_string(),
            var_type: EnvVarType::String,
            description: Some("Active Spring profiles".to_string()),
            required: true,
            sensitive: false,
            default_value: Some("development".to_string()),
            validation_pattern: None,
            environment_specific: true,
        });

        EnvironmentTemplate {
            name: "Spring Boot Application".to_string(),
            description: "Spring Boot Java application environment variables".to_string(),
            environment_type: EnvironmentType::Development,
            variables,
            tags: vec!["java".to_string(), "spring".to_string()],
        }
    }

    fn docker_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("COMPOSE_PROJECT_NAME".to_string(), EnvVariable {
            name: "COMPOSE_PROJECT_NAME".to_string(),
            value: "myapp".to_string(),
            var_type: EnvVarType::String,
            description: Some("Docker Compose project name".to_string()),
            required: false,
            sensitive: false,
            default_value: None,
            validation_pattern: Some(r"^[a-z0-9_-]+$".to_string()),
            environment_specific: false,
        });

        EnvironmentTemplate {
            name: "Docker Compose".to_string(),
            description: "Docker Compose environment variables".to_string(),
            environment_type: EnvironmentType::Development,
            variables,
            tags: vec!["docker".to_string(), "containerization".to_string()],
        }
    }

    fn kubernetes_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("KUBERNETES_NAMESPACE".to_string(), EnvVariable {
            name: "KUBERNETES_NAMESPACE".to_string(),
            value: "default".to_string(),
            var_type: EnvVarType::String,
            description: Some("Kubernetes namespace".to_string()),
            required: false,
            sensitive: false,
            default_value: Some("default".to_string()),
            validation_pattern: Some(r"^[a-z0-9-]+$".to_string()),
            environment_specific: true,
        });

        EnvironmentTemplate {
            name: "Kubernetes Deployment".to_string(),
            description: "Kubernetes deployment environment variables".to_string(),
            environment_type: EnvironmentType::Production,
            variables,
            tags: vec!["kubernetes".to_string(), "k8s".to_string()],
        }
    }

    fn aws_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("AWS_REGION".to_string(), EnvVariable {
            name: "AWS_REGION".to_string(),
            value: "us-east-1".to_string(),
            var_type: EnvVarType::String,
            description: Some("AWS region".to_string()),
            required: true,
            sensitive: false,
            default_value: Some("us-east-1".to_string()),
            validation_pattern: Some(r"^[a-z0-9-]+$".to_string()),
            environment_specific: false,
        });

        EnvironmentTemplate {
            name: "AWS Environment".to_string(),
            description: "Amazon Web Services environment variables".to_string(),
            environment_type: EnvironmentType::Production,
            variables,
            tags: vec!["aws".to_string(), "cloud".to_string()],
        }
    }

    fn gcp_template() -> EnvironmentTemplate {
        let mut variables = HashMap::new();
        variables.insert("GOOGLE_CLOUD_PROJECT".to_string(), EnvVariable {
            name: "GOOGLE_CLOUD_PROJECT".to_string(),
            value: "".to_string(),
            var_type: EnvVarType::String,
            description: Some("Google Cloud project ID".to_string()),
            required: true,
            sensitive: false,
            default_value: None,
            validation_pattern: Some(r"^[a-z0-9-]+$".to_string()),
            environment_specific: false,
        });

        EnvironmentTemplate {
            name: "Google Cloud Environment".to_string(),
            description: "Google Cloud Platform environment variables".to_string(),
            environment_type: EnvironmentType::Production,
            variables,
            tags: vec!["gcp".to_string(), "google".to_string(), "cloud".to_string()],
        }
    }
}

/// Connection string components
#[derive(Debug, Clone)]
pub struct ConnectionComponents {
    pub database_type: DatabaseType,
    pub host: String,
    pub port: Option<u16>,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub scheme: String,
    pub query_params: HashMap<String, String>,
}

/// Environment template for common setups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentTemplate {
    pub name: String,
    pub description: String,
    pub environment_type: EnvironmentType,
    pub variables: HashMap<String, EnvVariable>,
    pub tags: Vec<String>,
}

impl std::fmt::Display for EnvironmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentType::Development => write!(f, "development"),
            EnvironmentType::Staging => write!(f, "staging"),
            EnvironmentType::Testing => write!(f, "testing"),
            EnvironmentType::Production => write!(f, "production"),
            EnvironmentType::Local => write!(f, "local"),
            EnvironmentType::Demo => write!(f, "demo"),
            EnvironmentType::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl std::str::FromStr for EnvironmentType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(EnvironmentType::Development),
            "staging" | "stage" => Ok(EnvironmentType::Staging),
            "testing" | "test" => Ok(EnvironmentType::Testing),
            "production" | "prod" => Ok(EnvironmentType::Production),
            "local" => Ok(EnvironmentType::Local),
            "demo" => Ok(EnvironmentType::Demo),
            s if s.starts_with("custom_") => {
                let name = s.strip_prefix("custom_").unwrap_or(s);
                Ok(EnvironmentType::Custom(name.to_string()))
            }
            _ => Err(Error::Other(format!("Unknown environment type: {}", s))),
        }
    }
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionType::Database(db) => write!(f, "database_{:?}", db),
            ConnectionType::MessageQueue(mq) => write!(f, "messagequeue_{:?}", mq),
            ConnectionType::Cache(cache) => write!(f, "cache_{:?}", cache),
            ConnectionType::Cloud(cloud) => write!(f, "cloud_{:?}", cloud),
            ConnectionType::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_variable_creation() {
        let env_var = EnvConnectionManager::create_env_variable(
            "Development DB URL".to_string(),
            "DATABASE_URL".to_string(),
            "postgresql://user:pass@localhost:5432/mydb".to_string(),
            EnvVarType::Url,
            EnvironmentType::Development,
            Some("Database connection URL".to_string()),
            vec!["database".to_string(), "postgresql".to_string()],
            true,
        ).unwrap();

        assert_eq!(env_var.name, "Development DB URL");
        assert_eq!(env_var.secret_type, SecretType::Configuration);
    }

    #[test]
    fn test_connection_string_creation() {
        let conn = EnvConnectionManager::create_connection_string(
            "Production Database".to_string(),
            ConnectionType::Database(DatabaseType::PostgreSQL),
            "prod.database.com".to_string(),
            Some(5432),
            "production_db".to_string(),
            "admin".to_string(),
            "secret_password".to_string(),
            EnvironmentType::Production,
            None,
            Some("Production PostgreSQL database".to_string()),
            vec!["production".to_string(), "postgresql".to_string()],
        ).unwrap();

        assert_eq!(conn.name, "Production Database");
        assert_eq!(conn.secret_type, SecretType::ConnectionString);
    }

    #[test]
    fn test_connection_string_parsing() {
        let connection_string = "postgresql://user:password@localhost:5432/mydb?sslmode=require";
        let components = EnvConnectionManager::parse_connection_string(connection_string).unwrap();

        assert_eq!(components.host, "localhost");
        assert_eq!(components.port, Some(5432));
        assert_eq!(components.database, "mydb");
        assert_eq!(components.username, Some("user".to_string()));
        assert_eq!(components.password, Some("password".to_string()));
        assert!(matches!(components.database_type, DatabaseType::PostgreSQL));
    }

    #[test]
    fn test_env_file_generation() {
        let mut variables = HashMap::new();
        variables.insert("NODE_ENV".to_string(), "production".to_string());
        variables.insert("DATABASE_URL".to_string(), "postgresql://user:pass@localhost/db".to_string());
        variables.insert("API_KEY".to_string(), "secret key with spaces".to_string());

        let env_content = EnvConnectionManager::generate_env_file(&variables);

        assert!(env_content.contains("NODE_ENV=production"));
        assert!(env_content.contains("DATABASE_URL=postgresql://user:pass@localhost/db"));
        assert!(env_content.contains("API_KEY=\"secret key with spaces\""));
        assert!(env_content.contains("# Environment Variables"));
    }

    #[test]
    fn test_environment_type_parsing() {
        assert_eq!("development".parse::<EnvironmentType>().unwrap(), EnvironmentType::Development);
        assert_eq!("prod".parse::<EnvironmentType>().unwrap(), EnvironmentType::Production);
        assert_eq!("custom_staging".parse::<EnvironmentType>().unwrap(), EnvironmentType::Custom("staging".to_string()));
    }

    #[test]
    fn test_connection_string_building() {
        let conn_str = EnvConnectionManager::build_connection_string(
            &DatabaseType::PostgreSQL,
            "localhost",
            Some(5432),
            "testdb",
            "user",
            "password",
            &None,
        ).unwrap();

        assert_eq!(conn_str, "postgresql://user:password@localhost:5432/testdb");
    }
}