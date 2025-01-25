use crate::database::{Database, PasswordEntry};
use crate::crypto::{hash_password, verify_password, encrypt, decrypt};
use std::io::{self, Write};

/// Helper function to read input from the user with error handling.
///
/// # Returns
/// - The trimmed input as a `String`.
///
/// # Errors
/// - Returns `io::Error` if reading from stdin fails.
pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Argon2 parameters for master password hashing.
/// - Memory cost: 19 MB
/// - Time cost: 2 iterations
/// - Parallelism: 1 thread
const MASTER_PARAMS: (u32, u32, u32) = (19456, 2, 1);

/// Creates a new master password and stores its hash in the database.
///
/// # Arguments
/// - `db`: A mutable reference to the `Database`.
///
/// # Returns
/// - A 32-byte encryption key derived from the master password.
///
/// # Errors
/// - Returns an error if the passwords don't match or if database operations fail.
pub fn create_master_password(db: &mut Database) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("Creating master password:");
    let password = rpassword::prompt_password("Enter new master password: ")?;
    let confirm = rpassword::prompt_password("Confirm master password: ")?;

    // Ensure the passwords match.
    if password != confirm {
        return Err("Passwords do not match".into());
    }

    // Hash the master password and derive an encryption key.
    let (hash, key) = hash_password(
        &password,
        MASTER_PARAMS.0,
        MASTER_PARAMS.1,
        MASTER_PARAMS.2,
    )?;

    // Store the hash in the database.
    db.create_master_password(&hash)?;
    Ok(key)
}

/// Verifies the master password and derives an encryption key.
///
/// # Arguments
/// - `db`: A reference to the `Database`.
///
/// # Returns
/// - A 32-byte encryption key derived from the master password.
///
/// # Errors
/// - Returns an error if the master password is incorrect or if database operations fail.
pub fn verify_master_password(db: &Database) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Retrieve the stored master password hash.
    let stored_hash = db.get_master_password_hash()?
        .ok_or("Master password not set")?;

    // Prompt the user to enter their master password.
    let password = rpassword::prompt_password("Enter master password: ")?;

    // Verify the entered password against the stored hash.
    if !verify_password(&password, &stored_hash)? {
        return Err("Incorrect master password".into());
    }

    // Derive an encryption key from the master password.
    let (_, key) = hash_password(
        &password,
        MASTER_PARAMS.0,
        MASTER_PARAMS.1,
        MASTER_PARAMS.2,
    )?;

    Ok(key)
}

/// Adds a new password entry to the database.
///
/// # Arguments
/// - `db`: A mutable reference to the `Database`.
/// - `key`: The encryption key used to encrypt the password.
///
/// # Errors
/// - Returns an error if input reading, encryption, or database operations fail.
pub fn add_password(db: &mut Database, key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for website, username, and password.
    print!("Enter the website: ");
    io::stdout().flush()?;
    let website = read_input()?;

    print!("Enter the username: ");
    io::stdout().flush()?;
    let username = read_input()?;

    let password = rpassword::prompt_password("Enter the password: ")?;

    // Encrypt the password before storing it.
    let encrypted_password = encrypt(password.as_bytes(), key)?;

    // Create a new password entry.
    let entry = PasswordEntry {
        website,
        username,
        password: encrypted_password,
    };

    // Add the entry to the database.
    db.add_password(&entry)?;
    println!("Password added successfully!");
    Ok(())
}

/// Removes a password entry from the database.
///
/// # Arguments
/// - `db`: A mutable reference to the `Database`.
///
/// # Errors
/// - Returns an error if input reading or database operations fail.
pub fn remove_password(db: &mut Database) -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for the website to remove.
    print!("Enter the website to remove: ");
    io::stdout().flush()?;
    let website = read_input()?;

    // Delete the password entry.
    if db.delete_password(&website)? {
        println!("Password for {} removed successfully!", website);
    } else {
        println!("No password found for {}.", website);
    }
    Ok(())
}

/// Lists all password entries in the database.
///
/// # Arguments
/// - `db`: A reference to the `Database`.
///
/// # Errors
/// - Returns an error if database operations fail.
pub fn list_passwords(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve all password entries.
    let passwords = db.list_passwords()?;

    // Display the results.
    if passwords.is_empty() {
        println!("No passwords found.");
    } else {
        println!("Passwords:");
        for entry in passwords {
            println!("- Website: {}, Username: {}", entry.website, entry.username);
        }
    }
    Ok(())
}

/// Retrieves and decrypts a password entry from the database.
///
/// # Arguments
/// - `db`: A reference to the `Database`.
/// - `key`: The encryption key used to decrypt the password.
///
/// # Errors
/// - Returns an error if input reading, decryption, or database operations fail.
pub fn get_password_info(db: &Database, key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for the website.
    print!("Enter the website: ");
    io::stdout().flush()?;
    let website = read_input()?;

    // Retrieve the password entry.
    if let Some(entry) = db.get_password(&website)? {
        // Decrypt the password.
        let decrypted_password = decrypt(&entry.password, key)?;
        println!(
            "Website: {}, Username: {}, Password: {}",
            entry.website,
            entry.username,
            String::from_utf8(decrypted_password)?
        );
    } else {
        println!("No password found for {}.", website);
    }
    Ok(())
}

/// Clears all password entries from the database.
///
/// # Arguments
/// - `db`: A mutable reference to the `Database`.
///
/// # Errors
/// - Returns an error if database operations fail.
pub fn clear_all_data(db: &mut Database) -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for confirmation.
    print!("Are you sure you want to clear all data? (y/n): ");
    io::stdout().flush()?;
    let confirmation = read_input()?;

    // Clear all data if confirmed.
    if confirmation.to_lowercase() == "y" || confirmation.to_lowercase() == "yes" {
        db.clear_all_passwords()?;
        println!("All data cleared successfully!");
    } else {
        println!("Operation canceled.");
    }
    Ok(())
}