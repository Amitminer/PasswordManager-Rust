use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, PasswordHash, SaltString, PasswordVerifier},
    Argon2, ParamsBuilder,
};
use thiserror::Error;
use zeroize::Zeroize;

/// Custom error type for cryptographic operations.
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    #[error("Password hashing failed: {0}")]
    HashingError(String),
}

pub fn hash_password(
    password: &str,
    memory_cost: u32,
    time_cost: u32,
    parallelism: u32,
    existing_salt: Option<&str>,
) -> Result<(String, String, Vec<u8>), CryptoError> {
    let salt = match existing_salt {
        Some(s) => SaltString::from_b64(s).map_err(|_| CryptoError::HashingError("Invalid salt".to_string()))?,
        None => SaltString::generate(&mut OsRng),
    };

    let params = ParamsBuilder::new()
        .m_cost(memory_cost)
        .t_cost(time_cost)
        .p_cost(parallelism)
        .build()
        .map_err(|_| CryptoError::HashingError("Hashing failed".to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| CryptoError::HashingError("Hashing failed".to_string()))?;

    let hash_str = password_hash.to_string();
    let key = password_hash
        .hash
        .expect("Hash should exist")
        .as_bytes()[..32]
        .to_vec();

    Ok((hash_str, salt.to_string(), key))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, CryptoError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| CryptoError::HashingError("Hashing failed".to_string()))?;

    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(result)
}

/// Encrypts data using AES-256-GCM.
///
/// # Arguments
/// - `data`: The data to encrypt.
/// - `key`: The encryption key (must be 32 bytes).
///
/// # Returns
/// - A vector containing the nonce (12 bytes) followed by the encrypted data.
///
/// # Errors
/// - Returns `CryptoError::EncryptionError` if encryption fails.
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut encryption_key_copy = key.to_vec();
    let cipher = Aes256Gcm::new_from_slice(&encryption_key_copy)
        .map_err(|_| CryptoError::EncryptionError("Failed to initialize AES-256-GCM cipher".to_string()))?;

    let encryption_nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&encryption_nonce, data)
        .map_err(|_| CryptoError::EncryptionError("Encryption failed".to_string()))?;

    encryption_key_copy.zeroize();

    let result = [encryption_nonce.to_vec(), ciphertext].concat();

    Ok(result)
}

/// Decrypts data using AES-256-GCM.
///
/// # Arguments
/// - `encrypted_data`: The encrypted data (nonce + ciphertext).
/// - `key`: The encryption key (must be 32 bytes).
///
/// # Returns
/// - The decrypted data as a byte vector.
///
/// # Errors
/// - Returns `CryptoError::DecryptionError` if decryption fails.
pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if encrypted_data.len() < 12 {
        return Err(CryptoError::DecryptionError("Encrypted data too short".to_string()));
    }
    
    if key.len() != 32 {
        return Err(CryptoError::DecryptionError("Invalid key length".to_string()));
    }
    
    let mut encryption_key_copy = key.to_vec();
    let cipher = Aes256Gcm::new_from_slice(&encryption_key_copy)
        .map_err(|_| CryptoError::DecryptionError("Failed to create cipher".to_string()))?;
    
    let encryption_nonce = Nonce::from_slice(&encrypted_data[..12]);
    let ciphertext = &encrypted_data[12..];
    
    let decrypted_data = cipher
        .decrypt(encryption_nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionError("Decryption failed".to_string()))?;
    
    encryption_key_copy.zeroize();
    Ok(decrypted_data)
}
