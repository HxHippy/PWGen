use std::path::{Path, PathBuf};
use std::fs;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

use crate::{Result, Error};
use crate::crypto::MasterKey;
use crate::secrets::{SecretData, SecretMetadata};

/// Document types supported by the storage system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentType {
    /// Generic document
    Document,
    /// Digital certificate
    Certificate,
    /// Configuration file
    Configuration,
    /// License file
    License,
    /// Key file (non-SSH)
    KeyFile,
    /// Backup file
    Backup,
    /// Image file
    Image,
    /// Archive file
    Archive,
    /// Custom document type
    Custom(String),
}

/// Document metadata and information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub filename: String,
    pub content_type: String,
    pub file_size: u64,
    pub checksum_sha256: String,
    pub document_type: DocumentType,
    pub encoding: DocumentEncoding,
    pub compression: Option<CompressionType>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Document encoding formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentEncoding {
    /// Plain text (UTF-8)
    Text,
    /// Binary data
    Binary,
    /// Base64 encoded
    Base64,
}

/// Compression types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionType {
    /// No compression
    None,
    /// Gzip compression
    Gzip,
    /// Zlib compression
    Zlib,
    /// Deflate compression
    Deflate,
}

/// Document storage manager
pub struct DocumentManager;

impl DocumentManager {
    /// Import a file as a secure document
    pub fn import_file<P: AsRef<Path>>(
        file_path: P,
        document_type: DocumentType,
        description: Option<String>,
        compress: bool,
    ) -> Result<(SecretData, DocumentInfo)> {
        let path = file_path.as_ref();
        
        if !path.exists() {
            return Err(Error::Other(format!("File does not exist: {}", path.display())));
        }
        
        if !path.is_file() {
            return Err(Error::Other(format!("Path is not a file: {}", path.display())));
        }
        
        // Read file content
        let content = fs::read(path)
            .map_err(|e| Error::Other(format!("Failed to read file: {}", e)))?;
        
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Calculate checksum
        let checksum = Self::calculate_checksum(&content);
        
        // Determine content type
        let content_type = Self::detect_content_type(&content, &filename);
        
        // Determine encoding
        let encoding = if Self::is_text_content(&content) {
            DocumentEncoding::Text
        } else {
            DocumentEncoding::Binary
        };
        
        // Compress if requested and beneficial
        let (final_content, compression) = if compress && content.len() > 1024 {
            match Self::compress_content(&content) {
                Ok(compressed) if compressed.len() < content.len() => {
                    (compressed, Some(CompressionType::Gzip))
                }
                _ => (content, None),
            }
        } else {
            (content, None)
        };
        
        // Get file metadata
        let metadata = fs::metadata(path)
            .map_err(|e| Error::Other(format!("Failed to read file metadata: {}", e)))?;
        
        let last_modified = metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
            .flatten()
            .map(|dt| dt.with_timezone(&chrono::Utc));
        
        // Create document info
        let doc_info = DocumentInfo {
            filename: filename.clone(),
            content_type,
            file_size: metadata.len(),
            checksum_sha256: checksum,
            document_type,
            encoding,
            compression,
            created_at: chrono::Utc::now(),
            last_modified,
        };
        
        // Create secret data
        let secret_data = SecretData::Document {
            filename,
            content_type: doc_info.content_type.clone(),
            content: final_content,
            checksum: doc_info.checksum_sha256.clone(),
        };
        
        Ok((secret_data, doc_info))
    }
    
    /// Export a document to a file
    pub fn export_document(
        secret_data: &SecretData,
        output_path: &Path,
        verify_checksum: bool,
    ) -> Result<()> {
        if let SecretData::Document { filename: _, content_type: _, content, checksum } = secret_data {
            // Verify checksum if requested
            if verify_checksum {
                let calculated_checksum = Self::calculate_checksum(content);
                if calculated_checksum != *checksum {
                    return Err(Error::Other("Document checksum verification failed".to_string()));
                }
            }
            
            // Create parent directory if it doesn't exist
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| Error::Other(format!("Failed to create directory: {}", e)))?;
            }
            
            // Write content to file
            fs::write(output_path, content)
                .map_err(|e| Error::Other(format!("Failed to write file: {}", e)))?;
            
            println!("Document exported to: {}", output_path.display());
            Ok(())
        } else {
            Err(Error::Other("Secret is not a document".to_string()))
        }
    }
    
    /// Get document information from secret data
    pub fn get_document_info(secret_data: &SecretData) -> Result<DocumentInfo> {
        if let SecretData::Document { filename, content_type, content, checksum } = secret_data {
            let file_size = content.len() as u64;
            let encoding = if Self::is_text_content(content) {
                DocumentEncoding::Text
            } else {
                DocumentEncoding::Binary
            };
            
            Ok(DocumentInfo {
                filename: filename.clone(),
                content_type: content_type.clone(),
                file_size,
                checksum_sha256: checksum.clone(),
                document_type: Self::detect_document_type(filename, content_type),
                encoding,
                compression: None, // This would need to be stored separately
                created_at: chrono::Utc::now(),
                last_modified: None,
            })
        } else {
            Err(Error::Other("Secret is not a document".to_string()))
        }
    }
    
    /// Verify document integrity
    pub fn verify_document(secret_data: &SecretData) -> Result<bool> {
        if let SecretData::Document { content, checksum, .. } = secret_data {
            let calculated_checksum = Self::calculate_checksum(content);
            Ok(calculated_checksum == *checksum)
        } else {
            Err(Error::Other("Secret is not a document".to_string()))
        }
    }
    
    /// Create document from text content
    pub fn create_text_document(
        filename: String,
        text_content: String,
        document_type: DocumentType,
    ) -> Result<SecretData> {
        let content = text_content.into_bytes();
        let checksum = Self::calculate_checksum(&content);
        let content_type = "text/plain".to_string();
        
        Ok(SecretData::Document {
            filename,
            content_type,
            content,
            checksum,
        })
    }
    
    /// Extract text content from document (if it's text)
    pub fn extract_text_content(secret_data: &SecretData) -> Result<String> {
        if let SecretData::Document { content, .. } = secret_data {
            if Self::is_text_content(content) {
                String::from_utf8(content.clone())
                    .map_err(|e| Error::Other(format!("Failed to decode text: {}", e)))
            } else {
                Err(Error::Other("Document is not text content".to_string()))
            }
        } else {
            Err(Error::Other("Secret is not a document".to_string()))
        }
    }
    
    /// List supported file extensions and their types
    pub fn supported_extensions() -> Vec<(&'static str, DocumentType)> {
        vec![
            // Certificates
            (".pem", DocumentType::Certificate),
            (".crt", DocumentType::Certificate),
            (".cer", DocumentType::Certificate),
            (".p12", DocumentType::Certificate),
            (".pfx", DocumentType::Certificate),
            (".jks", DocumentType::Certificate),
            
            // Configuration files
            (".json", DocumentType::Configuration),
            (".yaml", DocumentType::Configuration),
            (".yml", DocumentType::Configuration),
            (".toml", DocumentType::Configuration),
            (".ini", DocumentType::Configuration),
            (".conf", DocumentType::Configuration),
            (".config", DocumentType::Configuration),
            (".xml", DocumentType::Configuration),
            
            // Key files
            (".key", DocumentType::KeyFile),
            (".priv", DocumentType::KeyFile),
            (".pub", DocumentType::KeyFile),
            
            // License files
            (".license", DocumentType::License),
            (".lic", DocumentType::License),
            
            // Archives
            (".zip", DocumentType::Archive),
            (".tar", DocumentType::Archive),
            (".gz", DocumentType::Archive),
            (".bz2", DocumentType::Archive),
            (".7z", DocumentType::Archive),
            
            // Images
            (".png", DocumentType::Image),
            (".jpg", DocumentType::Image),
            (".jpeg", DocumentType::Image),
            (".gif", DocumentType::Image),
            (".bmp", DocumentType::Image),
            (".svg", DocumentType::Image),
            
            // Documents
            (".txt", DocumentType::Document),
            (".md", DocumentType::Document),
            (".pdf", DocumentType::Document),
            (".doc", DocumentType::Document),
            (".docx", DocumentType::Document),
        ]
    }
    
    // Helper methods
    
    fn calculate_checksum(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        hex::encode(result)
    }
    
    fn detect_content_type(content: &[u8], filename: &str) -> String {
        // Simple content type detection based on file extension and magic bytes
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "txt" | "md" => "text/plain",
            "json" => "application/json",
            "xml" => "application/xml",
            "yaml" | "yml" => "application/x-yaml",
            "toml" => "application/toml",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "pem" | "crt" | "cer" => "application/x-pem-file",
            "p12" | "pfx" => "application/x-pkcs12",
            _ => {
                // Check magic bytes for common formats
                if content.len() >= 4 {
                    match &content[0..4] {
                        [0x50, 0x4B, 0x03, 0x04] => "application/zip",
                        [0x25, 0x50, 0x44, 0x46] => "application/pdf",
                        [0x89, 0x50, 0x4E, 0x47] => "image/png",
                        [0xFF, 0xD8, 0xFF, _] => "image/jpeg",
                        _ => "application/octet-stream",
                    }
                } else {
                    "application/octet-stream"
                }
            }
        }.to_string()
    }
    
    fn detect_document_type(filename: &str, content_type: &str) -> DocumentType {
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Check extension first
        for (ext, doc_type) in Self::supported_extensions() {
            if extension == ext.trim_start_matches('.') {
                return doc_type;
            }
        }
        
        // Fall back to content type
        match content_type {
            t if t.starts_with("image/") => DocumentType::Image,
            "application/zip" | "application/x-tar" | "application/gzip" => DocumentType::Archive,
            "application/x-pem-file" | "application/x-pkcs12" => DocumentType::Certificate,
            "application/json" | "application/xml" | "application/x-yaml" | "application/toml" => DocumentType::Configuration,
            _ => DocumentType::Document,
        }
    }
    
    fn is_text_content(content: &[u8]) -> bool {
        // Simple heuristic to determine if content is text
        if content.is_empty() {
            return true;
        }
        
        // Check for null bytes (binary indicator)
        if content.contains(&0) {
            return false;
        }
        
        // Check if content is valid UTF-8
        if String::from_utf8(content.to_vec()).is_err() {
            return false;
        }
        
        // Check the ratio of printable ASCII characters
        let printable_count = content.iter()
            .filter(|&&b| (b >= 32 && b <= 126) || b == 9 || b == 10 || b == 13)
            .count();
        
        let ratio = printable_count as f64 / content.len() as f64;
        ratio > 0.7 // If more than 70% are printable, consider it text
    }
    
    fn compress_content(content: &[u8]) -> Result<Vec<u8>> {
        use std::io::Write;
        
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(content)
            .map_err(|e| Error::Other(format!("Compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| Error::Other(format!("Compression failed: {}", e)))
    }
    
    fn decompress_content(compressed: &[u8]) -> Result<Vec<u8>> {
        use std::io::Read;
        
        let mut decoder = flate2::read::GzDecoder::new(compressed);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| Error::Other(format!("Decompression failed: {}", e)))?;
        
        Ok(decompressed)
    }
}

/// Document attachment utilities
pub struct DocumentAttachment;

impl DocumentAttachment {
    /// Create a document attachment from file path
    pub fn from_file<P: AsRef<Path>>(
        file_path: P,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
        compress: bool,
    ) -> Result<crate::secrets::DecryptedSecretEntry> {
        let path = file_path.as_ref();
        let document_type = Self::guess_document_type(path);
        
        let (secret_data, _doc_info) = DocumentManager::import_file(
            path,
            document_type,
            description.clone(),
            compress,
        )?;
        
        Ok(crate::secrets::DecryptedSecretEntry {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: crate::secrets::SecretType::Document,
            data: secret_data,
            metadata: crate::secrets::SecretMetadata::default(),
            tags,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_accessed: None,
            expires_at: None,
            favorite: false,
        })
    }
    
    /// Guess document type from file path
    pub fn guess_document_type(path: &Path) -> DocumentType {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        for (ext, doc_type) in DocumentManager::supported_extensions() {
            if extension == ext.trim_start_matches('.') {
                return doc_type;
            }
        }
        
        DocumentType::Document
    }
    
    /// Create a text document attachment
    pub fn from_text(
        name: String,
        filename: String,
        text_content: String,
        description: Option<String>,
        tags: Vec<String>,
        document_type: DocumentType,
    ) -> Result<crate::secrets::DecryptedSecretEntry> {
        let secret_data = DocumentManager::create_text_document(
            filename,
            text_content,
            document_type,
        )?;
        
        Ok(crate::secrets::DecryptedSecretEntry {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            secret_type: crate::secrets::SecretType::Document,
            data: secret_data,
            metadata: crate::secrets::SecretMetadata::default(),
            tags,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_accessed: None,
            expires_at: None,
            favorite: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    
    #[test]
    fn test_content_type_detection() {
        assert_eq!(DocumentManager::detect_content_type(b"Hello world", "test.txt"), "text/plain");
        assert_eq!(DocumentManager::detect_content_type(b"{\"key\": \"value\"}", "config.json"), "application/json");
        assert_eq!(DocumentManager::detect_content_type(&[0x89, 0x50, 0x4E, 0x47], "image.png"), "image/png");
    }
    
    #[test]
    fn test_text_detection() {
        assert!(DocumentManager::is_text_content(b"Hello, world!"));
        assert!(DocumentManager::is_text_content(b""));
        assert!(!DocumentManager::is_text_content(&[0x00, 0x01, 0x02, 0x03]));
    }
    
    #[test]
    fn test_checksum_calculation() {
        let content = b"test content";
        let checksum = DocumentManager::calculate_checksum(content);
        assert_eq!(checksum.len(), 64); // SHA256 hex length
    }
    
    #[test]
    fn test_document_type_detection() {
        assert_eq!(DocumentManager::detect_document_type("cert.pem", "application/x-pem-file"), DocumentType::Certificate);
        assert_eq!(DocumentManager::detect_document_type("config.json", "application/json"), DocumentType::Configuration);
        assert_eq!(DocumentManager::detect_document_type("image.png", "image/png"), DocumentType::Image);
    }
}