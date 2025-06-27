use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    password_hash::{rand_core::RngCore, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Error, Result};

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey {
    key: Vec<u8>,
}

impl MasterKey {
    pub fn derive_from_password(password: &str, salt: &[u8]) -> Result<Self> {
        let argon2 = Argon2::default();
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| Error::Encryption(format!("Invalid salt: {}", e)))?;
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| Error::Encryption(format!("Failed to hash password: {}", e)))?;
        
        let hash = password_hash.hash.ok_or_else(|| {
            Error::Encryption("Failed to extract hash from password".to_string())
        })?;
        
        let mut key = vec![0u8; 32];
        key.copy_from_slice(&hash.as_bytes()[..32]);
        
        Ok(Self { key })
    }
    
    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| Error::Decryption(format!("Invalid password hash: {}", e)))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    pub fn generate_salt() -> Vec<u8> {
        let mut salt = vec![0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut salt);
        salt
    }
    
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.key));
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| Error::Encryption(format!("Encryption failed: {}", e)))?;
        
        let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < NONCE_SIZE {
            return Err(Error::Decryption("Ciphertext too short".to_string()));
        }
        
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.key));
        
        cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| Error::Decryption(format!("Decryption failed: {}", e)))
    }
    
    pub fn hash_password_for_storage(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Error::Encryption(format!("Failed to hash password: {}", e)))?;
        
        Ok(password_hash.to_string())
    }
}

pub fn hash_entry_id(site: &str, username: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(site.as_bytes());
    hasher.update(b":");
    hasher.update(username.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let password = "test_password123!";
        let salt = MasterKey::generate_salt();
        let master_key = MasterKey::derive_from_password(password, &salt).unwrap();
        
        let plaintext = b"Hello, World!";
        let encrypted = master_key.encrypt(plaintext).unwrap();
        let decrypted = master_key.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
    
    #[test]
    fn test_password_verification() {
        let password = "secure_password123!";
        let hash = MasterKey::hash_password_for_storage(password).unwrap();
        
        assert!(MasterKey::verify_password(password, &hash).unwrap());
        assert!(!MasterKey::verify_password("wrong_password", &hash).unwrap());
    }
}