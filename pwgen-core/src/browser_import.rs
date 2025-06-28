use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use csv::ReaderBuilder;
use uuid::Uuid;

use crate::{Result, Error};
use crate::secrets::{SecretData, DecryptedSecretEntry, SecretType, SecretMetadata};
use crate::models::DecryptedPasswordEntry;

/// Supported browser types for import
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrowserType {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Opera,
    Brave,
    Vivaldi,
    Custom(String),
}

/// Browser import formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportFormat {
    /// CSV export from browser
    Csv,
    /// JSON export (some browsers/extensions)
    Json,
    /// Browser database file (SQLite)
    Database,
    /// 1Password 1PIF format
    OnePasswordPif,
    /// KeePass XML
    KeePassXml,
    /// LastPass CSV
    LastPassCsv,
    /// Bitwarden JSON
    BitwardenJson,
}

/// Imported password entry before conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedPassword {
    pub name: String,
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub notes: Option<String>,
    pub folder: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub totp_secret: Option<String>,
    pub favorite: bool,
    pub tags: Vec<String>,
}

/// Import statistics and results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub total_entries: usize,
    pub successful_imports: usize,
    pub failed_imports: usize,
    pub skipped_entries: usize,
    pub duplicate_entries: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub imported_entries: Vec<String>, // Entry IDs
}

/// Browser import configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportConfig {
    pub browser_type: BrowserType,
    pub format: ImportFormat,
    pub skip_duplicates: bool,
    pub merge_duplicates: bool,
    pub import_folders_as_tags: bool,
    pub default_tags: Vec<String>,
    pub password_strength_check: bool,
    pub cleanup_urls: bool,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            browser_type: BrowserType::Chrome,
            format: ImportFormat::Csv,
            skip_duplicates: true,
            merge_duplicates: false,
            import_folders_as_tags: true,
            default_tags: vec!["imported".to_string()],
            password_strength_check: false,
            cleanup_urls: true,
        }
    }
}

/// Manager for browser import functionality
pub struct BrowserImporter;

impl BrowserImporter {
    /// Import passwords from a file with the given configuration
    pub fn import_from_file<P: AsRef<Path>>(
        file_path: P,
        config: ImportConfig,
    ) -> Result<(Vec<ImportedPassword>, ImportResult)> {
        let file_path = file_path.as_ref();
        
        let passwords = match config.format {
            ImportFormat::Csv => Self::import_csv(file_path, &config)?,
            ImportFormat::Json => Self::import_json(file_path, &config)?,
            ImportFormat::Database => Self::import_database(file_path, &config)?,
            ImportFormat::OnePasswordPif => Self::import_1password_pif(file_path, &config)?,
            ImportFormat::KeePassXml => Self::import_keepass_xml(file_path, &config)?,
            ImportFormat::LastPassCsv => Self::import_lastpass_csv(file_path, &config)?,
            ImportFormat::BitwardenJson => Self::import_bitwarden_json(file_path, &config)?,
        };

        let result = Self::process_imported_passwords(&passwords, &config)?;
        
        Ok((passwords, result))
    }

    /// Convert imported passwords to password manager entries
    pub fn convert_to_entries(
        passwords: Vec<ImportedPassword>,
        config: &ImportConfig,
    ) -> Result<Vec<DecryptedPasswordEntry>> {
        let mut entries = Vec::new();
        
        for imported in passwords {
            let entry = Self::convert_imported_password(imported, config)?;
            entries.push(entry);
        }
        
        Ok(entries)
    }

    /// Convert imported passwords to secret entries (new format)
    pub fn convert_to_secret_entries(
        passwords: Vec<ImportedPassword>,
        config: &ImportConfig,
    ) -> Result<Vec<DecryptedSecretEntry>> {
        let mut entries = Vec::new();
        
        for imported in passwords {
            let entry = Self::convert_imported_to_secret(imported, config)?;
            entries.push(entry);
        }
        
        Ok(entries)
    }

    /// Get browser-specific default paths for password databases/exports
    pub fn get_default_browser_paths(browser: &BrowserType) -> Vec<PathBuf> {
        match browser {
            BrowserType::Chrome => Self::get_chrome_paths(),
            BrowserType::Firefox => Self::get_firefox_paths(),
            BrowserType::Safari => Self::get_safari_paths(),
            BrowserType::Edge => Self::get_edge_paths(),
            BrowserType::Opera => Self::get_opera_paths(),
            BrowserType::Brave => Self::get_brave_paths(),
            BrowserType::Vivaldi => Self::get_vivaldi_paths(),
            BrowserType::Custom(_) => vec![],
        }
    }

    /// Detect browser type from file content or path
    pub fn detect_browser_type<P: AsRef<Path>>(file_path: P) -> Result<BrowserType> {
        let file_path = file_path.as_ref();
        let path_str = file_path.to_string_lossy().to_lowercase();
        
        if path_str.contains("chrome") {
            Ok(BrowserType::Chrome)
        } else if path_str.contains("firefox") || path_str.contains("mozilla") {
            Ok(BrowserType::Firefox)
        } else if path_str.contains("safari") {
            Ok(BrowserType::Safari)
        } else if path_str.contains("edge") || path_str.contains("microsoftedge") {
            Ok(BrowserType::Edge)
        } else if path_str.contains("opera") {
            Ok(BrowserType::Opera)
        } else if path_str.contains("brave") {
            Ok(BrowserType::Brave)
        } else if path_str.contains("vivaldi") {
            Ok(BrowserType::Vivaldi)
        } else {
            // Try to detect from file content
            Self::detect_from_content(file_path)
        }
    }

    /// Import from CSV format (Chrome, Edge, most browsers)
    fn import_csv<P: AsRef<Path>>(
        file_path: P,
        config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        let content = std::fs::read_to_string(file_path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());

        let mut passwords = Vec::new();
        let headers = reader.headers()?.clone();
        
        for result in reader.records() {
            let record = result?;
            let password = Self::parse_csv_record(&record, &headers, config)?;
            if let Some(password) = password {
                passwords.push(password);
            }
        }

        Ok(passwords)
    }

    /// Import from JSON format
    fn import_json<P: AsRef<Path>>(
        file_path: P,
        _config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        let content = std::fs::read_to_string(file_path)?;
        let json_value: serde_json::Value = serde_json::from_str(&content)?;
        
        match json_value {
            serde_json::Value::Array(entries) => {
                let mut passwords = Vec::new();
                for entry in entries {
                    if let Some(password) = Self::parse_json_entry(&entry)? {
                        passwords.push(password);
                    }
                }
                Ok(passwords)
            }
            serde_json::Value::Object(ref obj) => {
                // Check if it's a Bitwarden export with nested structure
                if let Some(items) = obj.get("items") {
                    if let serde_json::Value::Array(entries) = items {
                        let mut passwords = Vec::new();
                        for entry in entries {
                            if let Some(password) = Self::parse_bitwarden_entry(entry)? {
                                passwords.push(password);
                            }
                        }
                        return Ok(passwords);
                    }
                }
                
                // Single entry JSON
                if let Some(password) = Self::parse_json_entry(&json_value)? {
                    Ok(vec![password])
                } else {
                    Ok(vec![])
                }
            }
            _ => Err(Error::Other("Invalid JSON format for password import".to_string())),
        }
    }

    /// Import from browser database (SQLite)
    fn import_database<P: AsRef<Path>>(
        _file_path: P,
        _config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        // Note: Direct database import would require SQLite access and is browser-specific
        // For now, we'll recommend users export to CSV first
        Err(Error::Other(
            "Direct database import not yet supported. Please export to CSV format first.".to_string()
        ))
    }

    /// Import from 1Password 1PIF format
    fn import_1password_pif<P: AsRef<Path>>(
        file_path: P,
        _config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        let content = std::fs::read_to_string(file_path)?;
        let mut passwords = Vec::new();
        
        for line in content.lines() {
            if line.trim().is_empty() || !line.starts_with("{") {
                continue;
            }
            
            if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(password) = Self::parse_1password_entry(&entry)? {
                    passwords.push(password);
                }
            }
        }
        
        Ok(passwords)
    }

    /// Import from KeePass XML format
    fn import_keepass_xml<P: AsRef<Path>>(
        _file_path: P,
        _config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        // KeePass XML parsing would require XML parser
        Err(Error::Other(
            "KeePass XML import not yet implemented. Please export to CSV format.".to_string()
        ))
    }

    /// Import from LastPass CSV format
    fn import_lastpass_csv<P: AsRef<Path>>(
        file_path: P,
        config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        // LastPass CSV has specific column order: url,username,password,extra,name,grouping,fav
        let content = std::fs::read_to_string(file_path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());

        let mut passwords = Vec::new();
        
        for result in reader.records() {
            let record = result?;
            let password = Self::parse_lastpass_record(&record, config)?;
            if let Some(password) = password {
                passwords.push(password);
            }
        }

        Ok(passwords)
    }

    /// Import from Bitwarden JSON format
    fn import_bitwarden_json<P: AsRef<Path>>(
        file_path: P,
        config: &ImportConfig,
    ) -> Result<Vec<ImportedPassword>> {
        // Bitwarden JSON is handled in import_json with special parsing
        Self::import_json(file_path, config)
    }

    // Helper methods for parsing different formats

    fn parse_csv_record(
        record: &csv::StringRecord,
        headers: &csv::StringRecord,
        config: &ImportConfig,
    ) -> Result<Option<ImportedPassword>> {
        let mut password = ImportedPassword {
            name: String::new(),
            url: None,
            username: None,
            password: None,
            notes: None,
            folder: None,
            created_at: None,
            updated_at: None,
            totp_secret: None,
            favorite: false,
            tags: config.default_tags.clone(),
        };

        // Map common column names to fields
        for (i, header) in headers.iter().enumerate() {
            if let Some(value) = record.get(i) {
                if value.trim().is_empty() {
                    continue;
                }
                
                match header.to_lowercase().as_str() {
                    "name" | "title" | "site" => password.name = value.to_string(),
                    "url" | "website" | "site_url" => {
                        password.url = Some(if config.cleanup_urls {
                            Self::cleanup_url(value)
                        } else {
                            value.to_string()
                        });
                    },
                    "username" | "user" | "login" => password.username = Some(value.to_string()),
                    "password" | "pass" => password.password = Some(value.to_string()),
                    "notes" | "note" | "comment" => password.notes = Some(value.to_string()),
                    "folder" | "group" | "category" => {
                        password.folder = Some(value.to_string());
                        if config.import_folders_as_tags && !value.is_empty() {
                            password.tags.push(value.to_string());
                        }
                    },
                    "totp" | "otp" | "2fa" => password.totp_secret = Some(value.to_string()),
                    "favorite" | "fav" => password.favorite = value.to_lowercase() == "true" || value == "1",
                    _ => {}
                }
            }
        }

        // Generate name if empty
        if password.name.is_empty() {
            password.name = password.url.as_ref()
                .map(|u| Self::extract_domain(u))
                .or_else(|| password.username.clone())
                .unwrap_or_else(|| "Imported Entry".to_string());
        }

        // Skip entries without password
        if password.password.is_none() || password.password.as_ref().unwrap().is_empty() {
            return Ok(None);
        }

        Ok(Some(password))
    }

    fn parse_json_entry(entry: &serde_json::Value) -> Result<Option<ImportedPassword>> {
        if !entry.is_object() {
            return Ok(None);
        }

        let obj = entry.as_object().unwrap();
        
        let name = obj.get("name")
            .or_else(|| obj.get("title"))
            .or_else(|| obj.get("site"))
            .and_then(|v| v.as_str())
            .unwrap_or("Imported Entry")
            .to_string();

        let url = obj.get("url")
            .or_else(|| obj.get("website"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let username = obj.get("username")
            .or_else(|| obj.get("user"))
            .or_else(|| obj.get("login"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let password = obj.get("password")
            .or_else(|| obj.get("pass"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Skip entries without password
        if password.is_none() || password.as_ref().unwrap().is_empty() {
            return Ok(None);
        }

        let notes = obj.get("notes")
            .or_else(|| obj.get("note"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let folder = obj.get("folder")
            .or_else(|| obj.get("group"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let favorite = obj.get("favorite")
            .or_else(|| obj.get("fav"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(Some(ImportedPassword {
            name,
            url,
            username,
            password,
            notes,
            folder,
            created_at: None,
            updated_at: None,
            totp_secret: None,
            favorite,
            tags: vec!["imported".to_string()],
        }))
    }

    fn parse_bitwarden_entry(entry: &serde_json::Value) -> Result<Option<ImportedPassword>> {
        let obj = entry.as_object().ok_or_else(|| {
            Error::Other("Invalid Bitwarden entry format".to_string())
        })?;

        // Skip non-login items
        if let Some(item_type) = obj.get("type").and_then(|v| v.as_u64()) {
            if item_type != 1 { // 1 = login type
                return Ok(None);
            }
        }

        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Imported Entry")
            .to_string();

        let login = obj.get("login").and_then(|v| v.as_object());
        
        let (username, password, totp_secret) = if let Some(login_obj) = login {
            let username = login_obj.get("username")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            let password = login_obj.get("password")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            let totp = login_obj.get("totp")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            (username, password, totp)
        } else {
            (None, None, None)
        };

        // Skip entries without password
        if password.is_none() || password.as_ref().unwrap().is_empty() {
            return Ok(None);
        }

        let notes = obj.get("notes")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let folder_id = obj.get("folderId")
            .and_then(|v| v.as_str());

        let favorite = obj.get("favorite")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Extract URL from URIs array
        let url = if let Some(login_obj) = login {
            login_obj.get("uris")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|uri| uri.get("uri"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        };

        Ok(Some(ImportedPassword {
            name,
            url,
            username,
            password,
            notes,
            folder: folder_id.map(|s| s.to_string()),
            created_at: None,
            updated_at: None,
            totp_secret,
            favorite,
            tags: vec!["imported".to_string(), "bitwarden".to_string()],
        }))
    }

    fn parse_1password_entry(entry: &serde_json::Value) -> Result<Option<ImportedPassword>> {
        let obj = entry.as_object().ok_or_else(|| {
            Error::Other("Invalid 1Password entry format".to_string())
        })?;

        // Skip non-login items
        if let Some(category) = obj.get("category").and_then(|v| v.as_str()) {
            if category != "001" { // 001 = login category
                return Ok(None);
            }
        }

        let name = obj.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Imported Entry")
            .to_string();

        let location = obj.get("location")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Parse secure contents
        let (username, password) = if let Some(contents) = obj.get("secureContents") {
            let username = contents.get("username")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            let password = contents.get("password")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            (username, password)
        } else {
            (None, None)
        };

        // Skip entries without password
        if password.is_none() || password.as_ref().unwrap().is_empty() {
            return Ok(None);
        }

        let notes = obj.get("secureContents")
            .and_then(|v| v.get("notesPlain"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Some(ImportedPassword {
            name,
            url: location,
            username,
            password,
            notes,
            folder: None,
            created_at: None,
            updated_at: None,
            totp_secret: None,
            favorite: false,
            tags: vec!["imported".to_string(), "1password".to_string()],
        }))
    }

    fn parse_lastpass_record(
        record: &csv::StringRecord,
        config: &ImportConfig,
    ) -> Result<Option<ImportedPassword>> {
        // LastPass CSV format: url,username,password,extra,name,grouping,fav
        if record.len() < 5 {
            return Ok(None);
        }

        let url = record.get(0).unwrap_or("").trim();
        let username = record.get(1).unwrap_or("").trim();
        let password = record.get(2).unwrap_or("").trim();
        let extra = record.get(3).unwrap_or("").trim();
        let name = record.get(4).unwrap_or("").trim();
        let grouping = record.get(5).unwrap_or("").trim();
        let fav = record.get(6).unwrap_or("").trim();

        // Skip entries without password
        if password.is_empty() {
            return Ok(None);
        }

        let mut tags = config.default_tags.clone();
        tags.push("lastpass".to_string());

        if config.import_folders_as_tags && !grouping.is_empty() {
            tags.push(grouping.to_string());
        }

        let entry_name = if name.is_empty() {
            if !url.is_empty() {
                Self::extract_domain(url)
            } else {
                "Imported Entry".to_string()
            }
        } else {
            name.to_string()
        };

        Ok(Some(ImportedPassword {
            name: entry_name,
            url: if url.is_empty() { None } else { 
                Some(if config.cleanup_urls {
                    Self::cleanup_url(url)
                } else {
                    url.to_string()
                })
            },
            username: if username.is_empty() { None } else { Some(username.to_string()) },
            password: Some(password.to_string()),
            notes: if extra.is_empty() { None } else { Some(extra.to_string()) },
            folder: if grouping.is_empty() { None } else { Some(grouping.to_string()) },
            created_at: None,
            updated_at: None,
            totp_secret: None,
            favorite: fav == "1" || fav.to_lowercase() == "true",
            tags,
        }))
    }

    // Conversion methods

    fn convert_imported_password(
        imported: ImportedPassword,
        _config: &ImportConfig,
    ) -> Result<DecryptedPasswordEntry> {
        let now = Utc::now();
        Ok(DecryptedPasswordEntry {
            id: Uuid::new_v4().to_string(),
            site: imported.url.unwrap_or_else(|| imported.name.clone()),
            username: imported.username.unwrap_or_default(),
            password: imported.password.unwrap_or_default(),
            notes: imported.notes,
            tags: imported.tags,
            favorite: imported.favorite,
            created_at: imported.created_at.unwrap_or(now),
            updated_at: imported.updated_at.unwrap_or(now),
            last_used: None,
            password_changed_at: imported.updated_at.unwrap_or(now),
        })
    }

    fn convert_imported_to_secret(
        imported: ImportedPassword,
        _config: &ImportConfig,
    ) -> Result<DecryptedSecretEntry> {
        let secret_data = SecretData::Password {
            username: imported.username.unwrap_or_default(),
            password: imported.password.unwrap_or_default(),
            url: imported.url,
            notes: imported.notes,
        };

        Ok(DecryptedSecretEntry {
            id: Uuid::new_v4().to_string(),
            name: imported.name,
            description: imported.folder,
            secret_type: SecretType::Password,
            data: secret_data,
            metadata: SecretMetadata::default(),
            tags: imported.tags,
            created_at: imported.created_at.unwrap_or_else(Utc::now),
            updated_at: imported.updated_at.unwrap_or_else(Utc::now),
            last_accessed: None,
            expires_at: None,
            favorite: imported.favorite,
        })
    }

    // Utility methods

    fn process_imported_passwords(
        passwords: &[ImportedPassword],
        _config: &ImportConfig,
    ) -> Result<ImportResult> {
        let total_entries = passwords.len();
        let successful_imports = passwords.iter()
            .filter(|p| p.password.is_some() && !p.password.as_ref().unwrap().is_empty())
            .count();
        let failed_imports = total_entries - successful_imports;

        Ok(ImportResult {
            total_entries,
            successful_imports,
            failed_imports,
            skipped_entries: 0,
            duplicate_entries: 0,
            errors: vec![],
            warnings: vec![],
            imported_entries: passwords.iter()
                .filter(|p| p.password.is_some() && !p.password.as_ref().unwrap().is_empty())
                .map(|p| p.name.clone())
                .collect(),
        })
    }

    fn cleanup_url(url: &str) -> String {
        let url = url.trim();
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else if url.contains('.') {
            format!("https://{}", url)
        } else {
            url.to_string()
        }
    }

    fn extract_domain(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.host_str().unwrap_or(url).to_string()
        } else {
            // If URL parsing fails, try to extract domain manually
            url.split('/').next().unwrap_or(url)
                .split('.')
                .rev()
                .take(2)
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(".")
        }
    }

    fn detect_from_content<P: AsRef<Path>>(file_path: P) -> Result<BrowserType> {
        let content = std::fs::read_to_string(file_path)?;
        
        // Check for CSV headers that indicate specific browsers
        let first_line = content.lines().next().unwrap_or("");
        let lower_line = first_line.to_lowercase();
        
        if lower_line.contains("lastpass") {
            return Ok(BrowserType::Custom("LastPass".to_string()));
        }
        
        if lower_line.contains("bitwarden") {
            return Ok(BrowserType::Custom("Bitwarden".to_string()));
        }
        
        // Default to Chrome format for standard CSV
        if lower_line.contains("name") && lower_line.contains("url") && lower_line.contains("username") {
            return Ok(BrowserType::Chrome);
        }
        
        // Check for JSON format indicators
        if content.trim_start().starts_with('{') || content.trim_start().starts_with('[') {
            if content.contains("\"encrypted\"") && content.contains("\"folders\"") {
                return Ok(BrowserType::Custom("Bitwarden".to_string()));
            }
            if content.contains("\"category\"") && content.contains("\"secureContents\"") {
                return Ok(BrowserType::Custom("1Password".to_string()));
            }
        }
        
        Ok(BrowserType::Chrome) // Default fallback
    }

    // Browser-specific path helpers

    fn get_chrome_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                paths.push(PathBuf::from(local_app_data).join("Google/Chrome/User Data/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/Google/Chrome/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".config/google-chrome/Default/Login Data"));
            }
        }
        
        paths
    }

    fn get_firefox_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Some(app_data) = std::env::var_os("APPDATA") {
                paths.push(PathBuf::from(app_data).join("Mozilla/Firefox/Profiles"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/Firefox/Profiles"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".mozilla/firefox"));
            }
        }
        
        paths
    }

    fn get_safari_paths() -> Vec<PathBuf> {
        let paths = Vec::new();
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Safari/Passwords.plist"));
            }
        }
        
        paths
    }

    fn get_edge_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                paths.push(PathBuf::from(local_app_data).join("Microsoft/Edge/User Data/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/Microsoft Edge/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".config/microsoft-edge/Default/Login Data"));
            }
        }
        
        paths
    }

    fn get_opera_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Some(app_data) = std::env::var_os("APPDATA") {
                paths.push(PathBuf::from(app_data).join("Opera Software/Opera Stable/Login Data"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/com.operasoftware.Opera/Login Data"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".config/opera/Login Data"));
            }
        }
        
        paths
    }

    fn get_brave_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                paths.push(PathBuf::from(local_app_data).join("BraveSoftware/Brave-Browser/User Data/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/BraveSoftware/Brave-Browser/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".config/BraveSoftware/Brave-Browser/Default/Login Data"));
            }
        }
        
        paths
    }

    fn get_vivaldi_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                paths.push(PathBuf::from(local_app_data).join("Vivaldi/User Data/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/Vivaldi/Default/Login Data"));
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Some(home) = std::env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".config/vivaldi/Default/Login Data"));
            }
        }
        
        paths
    }
}

impl std::fmt::Display for BrowserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrowserType::Chrome => write!(f, "Google Chrome"),
            BrowserType::Firefox => write!(f, "Mozilla Firefox"),
            BrowserType::Safari => write!(f, "Safari"),
            BrowserType::Edge => write!(f, "Microsoft Edge"),
            BrowserType::Opera => write!(f, "Opera"),
            BrowserType::Brave => write!(f, "Brave"),
            BrowserType::Vivaldi => write!(f, "Vivaldi"),
            BrowserType::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::str::FromStr for BrowserType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "chrome" | "google chrome" => Ok(BrowserType::Chrome),
            "firefox" | "mozilla firefox" => Ok(BrowserType::Firefox),
            "safari" => Ok(BrowserType::Safari),
            "edge" | "microsoft edge" => Ok(BrowserType::Edge),
            "opera" => Ok(BrowserType::Opera),
            "brave" => Ok(BrowserType::Brave),
            "vivaldi" => Ok(BrowserType::Vivaldi),
            s => Ok(BrowserType::Custom(s.to_string())),
        }
    }
}

impl std::fmt::Display for ImportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportFormat::Csv => write!(f, "CSV"),
            ImportFormat::Json => write!(f, "JSON"),
            ImportFormat::Database => write!(f, "Database"),
            ImportFormat::OnePasswordPif => write!(f, "1Password PIF"),
            ImportFormat::KeePassXml => write!(f, "KeePass XML"),
            ImportFormat::LastPassCsv => write!(f, "LastPass CSV"),
            ImportFormat::BitwardenJson => write!(f, "Bitwarden JSON"),
        }
    }
}

impl std::str::FromStr for ImportFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(ImportFormat::Csv),
            "json" => Ok(ImportFormat::Json),
            "database" | "db" | "sqlite" => Ok(ImportFormat::Database),
            "1password" | "1pif" => Ok(ImportFormat::OnePasswordPif),
            "keepass" | "xml" => Ok(ImportFormat::KeePassXml),
            "lastpass" => Ok(ImportFormat::LastPassCsv),
            "bitwarden" => Ok(ImportFormat::BitwardenJson),
            _ => Err(Error::Other(format!("Unknown import format: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_parse_chrome_csv() {
        let csv_content = "name,url,username,password\nTest Site,https://example.com,testuser,testpass123\n";
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(csv_content.as_bytes()).unwrap();
        
        let config = ImportConfig::default();
        let result = BrowserImporter::import_csv(temp_file.path(), &config).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Test Site");
        assert_eq!(result[0].url, Some("https://example.com".to_string()));
        assert_eq!(result[0].username, Some("testuser".to_string()));
        assert_eq!(result[0].password, Some("testpass123".to_string()));
    }

    #[test]
    fn test_parse_lastpass_csv() {
        let csv_content = "url,username,password,extra,name,grouping,fav\nhttps://example.com,user,pass,notes,Example,Work,0\n";
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(csv_content.as_bytes()).unwrap();
        
        let config = ImportConfig {
            format: ImportFormat::LastPassCsv,
            ..Default::default()
        };
        let result = BrowserImporter::import_lastpass_csv(temp_file.path(), &config).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Example");
        assert_eq!(result[0].folder, Some("Work".to_string()));
        assert!(!result[0].favorite);
    }

    #[test]
    fn test_cleanup_url() {
        assert_eq!(BrowserImporter::cleanup_url("example.com"), "https://example.com");
        assert_eq!(BrowserImporter::cleanup_url("https://example.com"), "https://example.com");
        assert_eq!(BrowserImporter::cleanup_url("http://example.com"), "http://example.com");
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(BrowserImporter::extract_domain("https://www.example.com/path"), "www.example.com");
        assert_eq!(BrowserImporter::extract_domain("example.com"), "example.com");
    }

    #[test]
    fn test_browser_type_parsing() {
        assert_eq!("chrome".parse::<BrowserType>().unwrap(), BrowserType::Chrome);
        assert_eq!("firefox".parse::<BrowserType>().unwrap(), BrowserType::Firefox);
        assert_eq!("custom".parse::<BrowserType>().unwrap(), BrowserType::Custom("custom".to_string()));
    }
}