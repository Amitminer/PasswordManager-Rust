use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, ParamsBuilder,
};
use thiserror::Error;
use zeroize::Zeroize;

/// Custom error type for cryptographic operations.
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionError,
    #[error("Decryption failed")]
    DecryptionError,
    #[error("Password hashing failed")]
    HashingError,
}

/// Hashes a password using Argon2 and derives a key.
///
/// # Arguments
/// - `password`: The password to hash.
/// - `memory_cost`: Memory cost parameter for Argon2 (in kibibytes).
/// - `time_cost`: Time cost parameter for Argon2 (number of iterations).
/// - `parallelism`: Parallelism parameter for Argon2 (number of threads).
///
/// # Returns
/// - A tuple containing:
///   - The Argon2 hash string (for storage).
///   - A 32-byte key derived from the hash (for encryption).
///
/// # Errors
/// - Returns `CryptoError::HashingError` if hashing fails.
pub fn hash_password(
    password: &str,
    memory_cost: u32,
    time_cost: u32,
    parallelism: u32,
) -> Result<(String, Vec<u8>), CryptoError> {
    // Generate a random salt for Argon2 hashing.
    let salt = SaltString::generate(&mut OsRng);

    // Configure Argon2 parameters.
    let params = ParamsBuilder::new()
        .m_cost(memory_cost) // Memory cost
        .t_cost(time_cost)   // Time cost
        .p_cost(parallelism) // Parallelism
        .build()
        .map_err(|_| CryptoError::HashingError)?;

    // Create an Argon2 instance with the specified parameters.
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    // Hash the password using Argon2.
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| CryptoError::HashingError)?;

    // Extract the hash string and derive a 32-byte key from it.
    let hash_str = password_hash.to_string();
    let key = password_hash
        .hash
        .expect("Hash should exist")
        .as_bytes()[..32]
        .to_vec();

    Ok((hash_str, key))
}

/// Verifies a password against a stored Argon2 hash.
///
/// # Arguments
/// - `password`: The password to verify.
/// - `hash`: The stored Argon2 hash string.
///
/// # Returns
/// - `true` if the password matches the hash, otherwise `false`.
///
/// # Errors
/// - Returns `CryptoError::HashingError` if hash parsing fails.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, CryptoError> {
    // Parse the stored hash string into a `PasswordHash` object.
    let parsed_hash = PasswordHash::new(hash).map_err(|_| CryptoError::HashingError)?;

    // Verify the password against the parsed hash.
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Encrypts data using AES-256-GCM.
///
/// # Arguments
/// - `data`: The data to encrypt.
/// - `key`: The encryption key (must be 32 bytes).
///
/// # Returns
/// - A concatenated vector containing the nonce and encrypted data.
///
/// # Errors
/// - Returns `CryptoError::EncryptionError` if encryption fails.
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Create a copy of the key and initialize the AES-256-GCM cipher.
    let mut key_copy = key.to_vec();
    let cipher = Aes256Gcm::new_from_slice(&key_copy).map_err(|_| CryptoError::EncryptionError)?;

    // Generate a random nonce for encryption.
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Encrypt the data using the cipher and nonce.
    let encrypted_data = cipher
        .encrypt(&nonce, data)
        .map_err(|_| CryptoError::EncryptionError)?;

    // Securely clear the key copy from memory.
    key_copy.zeroize();

    // Concatenate the nonce and encrypted data for storage.
    Ok([nonce.to_vec(), encrypted_data].concat())
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
    // Create a copy of the key and initialize the AES-256-GCM cipher.
    let mut key_copy = key.to_vec();
    let cipher = Aes256Gcm::new_from_slice(&key_copy).map_err(|_| CryptoError::DecryptionError)?;

    // Split the encrypted data into nonce and ciphertext.
    let nonce = Nonce::from_slice(&encrypted_data[..12]); // Nonce is always 12 bytes.
    let ciphertext = &encrypted_data[12..];

    // Decrypt the data using the cipher and nonce.
    let decrypted_data = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionError)?;

    // Securely clear the key copy from memory.
    key_copy.zeroize();

    Ok(decrypted_data)
}