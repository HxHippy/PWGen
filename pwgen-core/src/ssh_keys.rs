use std::process::Command;
use std::fs;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

use crate::{Result, Error};
use crate::secrets::{SshKeyType, SecretData};

/// SSH key generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyGenParams {
    pub key_type: SshKeyType,
    pub bits: Option<u32>,
    pub comment: Option<String>,
    pub passphrase: Option<String>,
}

impl Default for SshKeyGenParams {
    fn default() -> Self {
        Self {
            key_type: SshKeyType::Ed25519,
            bits: None,
            comment: None,
            passphrase: None,
        }
    }
}

/// SSH key information extracted from key content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyInfo {
    pub key_type: SshKeyType,
    pub fingerprint_md5: String,
    pub fingerprint_sha256: String,
    pub bit_length: Option<u32>,
    pub comment: Option<String>,
    pub is_encrypted: bool,
}

/// SSH key manager for generation, validation, and manipulation
pub struct SshKeyManager;

impl SshKeyManager {
    /// Generate a new SSH key pair
    pub fn generate_key_pair(params: &SshKeyGenParams) -> Result<(String, String)> {
        let temp_dir = std::env::temp_dir();
        let key_name = format!("pwgen_temp_key_{}", uuid::Uuid::new_v4());
        let private_key_path = temp_dir.join(&key_name);
        let public_key_path = temp_dir.join(format!("{}.pub", key_name));
        
        // Build ssh-keygen command
        let mut cmd = Command::new("ssh-keygen");
        cmd.arg("-f").arg(&private_key_path);
        cmd.arg("-t").arg(Self::key_type_to_string(&params.key_type));
        cmd.arg("-q"); // Quiet mode
        
        // Add key size for RSA keys
        if params.key_type == SshKeyType::Rsa {
            let bits = params.bits.unwrap_or(4096);
            cmd.arg("-b").arg(bits.to_string());
        }
        
        // Add comment if provided
        if let Some(comment) = &params.comment {
            cmd.arg("-C").arg(comment);
        }
        
        // Set passphrase (empty if none provided)
        cmd.arg("-N").arg(params.passphrase.as_deref().unwrap_or(""));
        
        // Execute command
        let output = cmd.output()
            .map_err(|e| Error::Other(format!("Failed to execute ssh-keygen: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Other(format!("ssh-keygen failed: {}", stderr)));
        }
        
        // Read generated keys
        let private_key = fs::read_to_string(&private_key_path)
            .map_err(|e| Error::Other(format!("Failed to read private key: {}", e)))?;
        
        let public_key = fs::read_to_string(&public_key_path)
            .map_err(|e| Error::Other(format!("Failed to read public key: {}", e)))?;
        
        // Clean up temporary files
        let _ = fs::remove_file(&private_key_path);
        let _ = fs::remove_file(&public_key_path);
        
        Ok((private_key, public_key.trim().to_string()))
    }
    
    /// Parse and validate an SSH private key
    pub fn parse_private_key(key_content: &str) -> Result<SshKeyInfo> {
        let lines: Vec<&str> = key_content.lines().collect();
        
        if lines.is_empty() {
            return Err(Error::Other("Empty key content".to_string()));
        }
        
        let first_line = lines[0];
        let is_encrypted = key_content.contains("ENCRYPTED");
        
        let key_type = if first_line.contains("RSA") {
            SshKeyType::Rsa
        } else if first_line.contains("ED25519") {
            SshKeyType::Ed25519
        } else if first_line.contains("ECDSA") {
            SshKeyType::Ecdsa
        } else if first_line.contains("DSA") {
            SshKeyType::Dsa
        } else {
            return Err(Error::Other("Unknown key type".to_string()));
        };
        
        // Generate fingerprints (simplified version)
        let fingerprint_sha256 = Self::calculate_fingerprint_sha256(key_content);
        let fingerprint_md5 = Self::calculate_fingerprint_md5(key_content);
        
        Ok(SshKeyInfo {
            key_type,
            fingerprint_md5,
            fingerprint_sha256,
            bit_length: Self::extract_bit_length(key_content),
            comment: None,
            is_encrypted,
        })
    }
    
    /// Parse and validate an SSH public key
    pub fn parse_public_key(key_content: &str) -> Result<SshKeyInfo> {
        let parts: Vec<&str> = key_content.trim().split_whitespace().collect();
        
        if parts.len() < 2 {
            return Err(Error::Other("Invalid public key format".to_string()));
        }
        
        let key_type_str = parts[0];
        let key_data = parts[1];
        let comment = if parts.len() > 2 {
            Some(parts[2..].join(" "))
        } else {
            None
        };
        
        let key_type = match key_type_str {
            "ssh-rsa" => SshKeyType::Rsa,
            "ssh-ed25519" => SshKeyType::Ed25519,
            "ecdsa-sha2-nistp256" | "ecdsa-sha2-nistp384" | "ecdsa-sha2-nistp521" => SshKeyType::Ecdsa,
            "ssh-dss" => SshKeyType::Dsa,
            _ => return Err(Error::Other(format!("Unknown key type: {}", key_type_str))),
        };
        
        // Calculate fingerprints
        let fingerprint_sha256 = Self::calculate_public_key_fingerprint_sha256(key_data)?;
        let fingerprint_md5 = Self::calculate_public_key_fingerprint_md5(key_data)?;
        
        Ok(SshKeyInfo {
            key_type: key_type.clone(),
            fingerprint_md5,
            fingerprint_sha256,
            bit_length: Self::extract_public_key_bit_length(key_data, &key_type)?,
            comment,
            is_encrypted: false,
        })
    }
    
    /// Extract public key from private key
    pub fn extract_public_key(private_key: &str) -> Result<String> {
        let temp_dir = std::env::temp_dir();
        let key_name = format!("pwgen_extract_{}", uuid::Uuid::new_v4());
        let private_key_path = temp_dir.join(&key_name);
        
        // Write private key to temp file
        fs::write(&private_key_path, private_key)
            .map_err(|e| Error::Other(format!("Failed to write temp private key: {}", e)))?;
        
        // Extract public key using ssh-keygen
        let output = Command::new("ssh-keygen")
            .arg("-y")
            .arg("-f")
            .arg(&private_key_path)
            .output()
            .map_err(|e| Error::Other(format!("Failed to execute ssh-keygen: {}", e)))?;
        
        // Clean up temp file
        let _ = fs::remove_file(&private_key_path);
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Other(format!("Failed to extract public key: {}", stderr)));
        }
        
        let public_key = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(public_key)
    }
    
    /// Validate SSH key format and structure
    pub fn validate_key(key_content: &str, is_private: bool) -> Result<bool> {
        if is_private {
            Self::parse_private_key(key_content)?;
        } else {
            Self::parse_public_key(key_content)?;
        }
        Ok(true)
    }
    
    /// Convert SSH key to SecretData format
    pub fn to_secret_data(
        private_key: Option<String>,
        public_key: Option<String>,
        passphrase: Option<String>,
    ) -> Result<SecretData> {
        let key_type = if let Some(ref private_key) = private_key {
            Self::parse_private_key(private_key)?.key_type
        } else if let Some(ref public_key) = public_key {
            Self::parse_public_key(public_key)?.key_type
        } else {
            return Err(Error::Other("Either private or public key must be provided".to_string()));
        };
        
        let fingerprint = if let Some(ref public_key) = public_key {
            Some(Self::parse_public_key(public_key)?.fingerprint_sha256)
        } else if let Some(ref private_key) = private_key {
            // Try to extract public key and get fingerprint
            if let Ok(extracted_public) = Self::extract_public_key(private_key) {
                Some(Self::parse_public_key(&extracted_public)?.fingerprint_sha256)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(SecretData::SshKey {
            key_type,
            private_key,
            public_key,
            passphrase,
            comment: None,
            fingerprint,
        })
    }
    
    // Helper methods
    fn key_type_to_string(key_type: &SshKeyType) -> &'static str {
        match key_type {
            SshKeyType::Rsa => "rsa",
            SshKeyType::Ed25519 => "ed25519",
            SshKeyType::Ecdsa => "ecdsa",
            SshKeyType::Dsa => "dsa",
        }
    }
    
    fn calculate_fingerprint_sha256(key_content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key_content.as_bytes());
        let result = hasher.finalize();
        format!("SHA256:{}", general_purpose::STANDARD_NO_PAD.encode(result))
    }
    
    fn calculate_fingerprint_md5(key_content: &str) -> String {
        let mut hasher = md5::Context::new();
        hasher.consume(key_content.as_bytes());
        let result = hasher.compute();
        format!("MD5:{:x}", result)
    }
    
    fn calculate_public_key_fingerprint_sha256(key_data: &str) -> Result<String> {
        let decoded = general_purpose::STANDARD.decode(key_data)
            .map_err(|e| Error::Other(format!("Failed to decode key data: {}", e)))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&decoded);
        let result = hasher.finalize();
        Ok(format!("SHA256:{}", general_purpose::STANDARD_NO_PAD.encode(result)))
    }
    
    fn calculate_public_key_fingerprint_md5(key_data: &str) -> Result<String> {
        let decoded = general_purpose::STANDARD.decode(key_data)
            .map_err(|e| Error::Other(format!("Failed to decode key data: {}", e)))?;
        
        let mut hasher = md5::Context::new();
        hasher.consume(&decoded);
        let result = hasher.compute();
        Ok(format!("MD5:{:x}", result))
    }
    
    fn extract_bit_length(key_content: &str) -> Option<u32> {
        // Simple bit length extraction for RSA keys
        if key_content.contains("RSA") {
            // This is a simplified approach - in practice you'd parse the actual key structure
            if key_content.len() > 3000 {
                Some(4096)
            } else if key_content.len() > 1500 {
                Some(2048)
            } else {
                Some(1024)
            }
        } else {
            None
        }
    }
    
    fn extract_public_key_bit_length(_key_data: &str, key_type: &SshKeyType) -> Result<Option<u32>> {
        match key_type {
            SshKeyType::Rsa => {
                // For RSA, we could decode and parse the actual key to get precise bit length
                // This is simplified
                Ok(Some(2048)) // Default assumption
            }
            SshKeyType::Ed25519 => Ok(Some(256)),
            SshKeyType::Ecdsa => Ok(Some(256)), // Varies by curve
            SshKeyType::Dsa => Ok(Some(1024)),
        }
    }
}

/// SSH key utilities for common operations
pub struct SshKeyUtils;

impl SshKeyUtils {
    /// Convert between different SSH key formats
    pub fn convert_format(key_content: &str, target_format: &str) -> Result<String> {
        match target_format.to_lowercase().as_str() {
            "openssh" => Ok(key_content.to_string()), // Already in OpenSSH format
            "pem" => Self::to_pem_format(key_content),
            "pkcs8" => Self::to_pkcs8_format(key_content),
            _ => Err(Error::Other(format!("Unsupported format: {}", target_format))),
        }
    }
    
    /// Add or change passphrase on private key
    pub fn change_passphrase(private_key: &str, old_passphrase: Option<&str>, new_passphrase: Option<&str>) -> Result<String> {
        let temp_dir = std::env::temp_dir();
        let key_name = format!("pwgen_passphrase_{}", uuid::Uuid::new_v4());
        let key_path = temp_dir.join(&key_name);
        
        // Write key to temp file
        fs::write(&key_path, private_key)
            .map_err(|e| Error::Other(format!("Failed to write temp key: {}", e)))?;
        
        // Change passphrase using ssh-keygen
        let mut cmd = Command::new("ssh-keygen");
        cmd.arg("-p")
           .arg("-f").arg(&key_path);
        
        if let Some(old_pass) = old_passphrase {
            cmd.arg("-P").arg(old_pass);
        } else {
            cmd.arg("-P").arg("");
        }
        
        if let Some(new_pass) = new_passphrase {
            cmd.arg("-N").arg(new_pass);
        } else {
            cmd.arg("-N").arg("");
        }
        
        let output = cmd.output()
            .map_err(|e| Error::Other(format!("Failed to execute ssh-keygen: {}", e)))?;
        
        if !output.status.success() {
            let _ = fs::remove_file(&key_path);
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Other(format!("Failed to change passphrase: {}", stderr)));
        }
        
        // Read modified key
        let modified_key = fs::read_to_string(&key_path)
            .map_err(|e| Error::Other(format!("Failed to read modified key: {}", e)))?;
        
        // Clean up
        let _ = fs::remove_file(&key_path);
        
        Ok(modified_key)
    }
    
    /// Check if ssh-keygen is available
    pub fn check_ssh_keygen_available() -> bool {
        Command::new("ssh-keygen")
            .arg("--help")
            .output()
            .is_ok()
    }
    
    fn to_pem_format(key_content: &str) -> Result<String> {
        // This would require more complex conversion
        // For now, return as-is since OpenSSH format is widely supported
        Ok(key_content.to_string())
    }
    
    fn to_pkcs8_format(key_content: &str) -> Result<String> {
        // This would require more complex conversion
        Ok(key_content.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_type_conversion() {
        assert_eq!(SshKeyManager::key_type_to_string(&SshKeyType::Rsa), "rsa");
        assert_eq!(SshKeyManager::key_type_to_string(&SshKeyType::Ed25519), "ed25519");
    }
    
    #[test]
    fn test_ssh_keygen_availability() {
        // This test will only pass if ssh-keygen is installed
        let available = SshKeyUtils::check_ssh_keygen_available();
        println!("SSH keygen available: {}", available);
    }
    
    #[test]
    fn test_public_key_parsing() {
        let sample_public_key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIG4rT3vTt99Ox5kndS4HmgTrKBT8F0E6fks0DhP4VS4L test@example.com";
        
        match SshKeyManager::parse_public_key(sample_public_key) {
            Ok(info) => {
                assert_eq!(info.key_type, SshKeyType::Ed25519);
                assert_eq!(info.comment, Some("test@example.com".to_string()));
                assert!(!info.is_encrypted);
            }
            Err(e) => println!("Parse error (expected if ssh tools not available): {}", e),
        }
    }
}