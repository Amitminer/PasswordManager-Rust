use crate::cli;
use crate::crypto::{decrypt, encrypt, hash_password, verify_password};
use crate::database::{Database, PasswordEntry};
use crate::utils::api::PasswordInfo;
use std::error::Error;
use std::io::{self, Write};

/// Argon2 parameters for master password hashing.
const MASTER_PARAMS: (u32, u32, u32) = (19456, 2, 1);

/// Prompts the user for input with a message.
fn prompt_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    Ok(cli::read_input()?)
}

/// Derives a 32-byte encryption key from a password and salt.
fn derive_encryption_key(
    password: &str,
    stored_salt: Option<&str>,
) -> Result<(String, String, Vec<u8>), Box<dyn Error>> {
    let (hash, salt, key) = hash_password(
        password,
        MASTER_PARAMS.0,
        MASTER_PARAMS.1,
        MASTER_PARAMS.2,
        stored_salt,
    )?;
    Ok((hash, salt, key))
}

/// **Creates a new master password (CLI & API)**
pub fn create_master_password(
    is_cli: bool,
    database: &mut Database,
    password: Option<&str>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let master_password = if is_cli {
        println!("Creating master password:");
        let password = rpassword::prompt_password("Enter new master password: ")?;
        let confirm = rpassword::prompt_password("Confirm master password: ")?;
        if password != confirm {
            return Err("Passwords do not match".into());
        }
        password
    } else {
        match password {
            Some(p) => p.to_string(),
            None => return Err("Password is required.".into()),
        }
    };

    let (hash, salt, key) = derive_encryption_key(&master_password, None)?;
    database.create_master_password(&hash, &salt)?;

    if is_cli {
        println!("✅ Master password created successfully!");
    }
    Ok(key)
}

/// **Verifies the master password and retrieves encryption key (CLI & API)**
pub fn verify_master_password(
    database: &Database,
    password: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let (stored_hash, stored_salt) = database
        .get_master_password_data()?
        .ok_or("Master password not set")?;

    if !verify_password(password, &stored_hash)? {
        return Err("Incorrect master password".into());
    }

    let (_, _, key) = derive_encryption_key(password, Some(&stored_salt))?;
    Ok(key)
}

/// Ensures a master password exists, used by CLI.
pub fn get_or_create_master_key(db: &mut Database) -> Result<Vec<u8>, Box<dyn Error>> {
    if db.get_master_password_data()?.is_some() {
        let password = rpassword::prompt_password("Enter master password: ")?;
        verify_master_password(db, &password)
    } else {
        create_master_password(true, db, None)
    }
}

/// Adds a new password entry.
pub fn add_password(
    db: &mut Database,
    key: &[u8],
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    let encrypted_password = encrypt(password.as_bytes(), key)?;
    db.add_password(&PasswordEntry {
        website: service.to_string(),
        username: username.to_string(),
        password: encrypted_password,
    })?;
    println!("✅ Password added successfully for {}!", service);
    Ok(())
}

/// Removes a password entry.
pub fn remove_password(db: &mut Database, service: &str) -> Result<(), Box<dyn Error>> {
    if db.delete_password(service)? {
        println!("✅ Password for '{}' removed successfully!", service);
        Ok(())
    } else {
        Err(format!("❌ No password found for '{}'", service).into())
    }
}

/// Lists all stored password entries.
pub fn list_passwords(db: &Database, key: &[u8]) -> Result<Vec<PasswordInfo>, Box<dyn Error>> {
    let entries = db.list_passwords()?;
    let mut decrypted_passwords = Vec::new();

    println!("[DEBUG] Retrieved {} encrypted entries", entries.len());

    for (i, entry) in entries.iter().enumerate() {
        if entry.password.len() < 12 {
            println!("[ERROR] Entry {} has invalid encrypted data length", i);
            continue;
        }

        let full_entry = match db.get_password(&entry.website)? {
            Some(e) => e,
            None => {
                println!("[ERROR] Could not retrieve entry for '{}'", entry.website);
                continue;
            }
        };

        match decrypt(&full_entry.password, key) {
            Ok(decrypted_data) => {
                let password_str = String::from_utf8_lossy(&decrypted_data);
                decrypted_passwords.push(PasswordInfo {
                    service: full_entry.website.clone(),
                    username: full_entry.username.clone(),
                    password: password_str.to_string(),
                });
                println!("[DEBUG] Successfully decrypted entry {} for '{}'", i, full_entry.website);
            }
            Err(_) => println!("[ERROR] Decryption failed for '{}'", full_entry.website),
        }
    }

    println!(
        "[DEBUG] Successfully decrypted {} out of {} passwords",
        decrypted_passwords.len(),
        entries.len()
    );
    Ok(decrypted_passwords)
}

/// Retrieves a single password entry.
pub fn get_password_info(
    db: &Database,
    key: &[u8],
    service: &str,
) -> Result<PasswordInfo, Box<dyn Error>> {
    match db.get_password(service)? {
        Some(entry) => match decrypt(&entry.password, key) {
            Ok(decrypted_password) => Ok(PasswordInfo {
                service: entry.website,
                username: entry.username,
                password: String::from_utf8(decrypted_password)?,
            }),
            Err(_) => Err("Decryption failed".into()),
        },
        None => Err(format!("❌ No password found for '{}'", service).into()),
    }
}

/// Clears all stored passwords with user confirmation.
pub fn clear_all_data(db: &mut Database) -> Result<(), Box<dyn Error>> {
    if cli::confirm_action("⚠️ Are you sure you want to clear all data? (y/n): ")? {
        db.clear_all_passwords()?;
        println!("✅ All passwords deleted successfully!");
    } else {
        println!("❌ Operation canceled.");
    }
    Ok(())
}

/// Displays the main menu options.
fn display_menu() {
    println!("\n🔐 Welcome to Password Manager:");
    let options = [
        "Add new password",
        "Remove password",
        "List passwords",
        "Get password info",
        "Clear all data",
        "Exit",
    ];
    for (index, option) in options.iter().enumerate() {
        println!("[{}] {}", index, option);
    }
}

/// Handles menu selection logic.
fn handle_menu_choice(
    choice: &str,
    db: &mut Database,
    master_key: &[u8],
) -> Result<bool, Box<dyn Error>> {
    match choice {
        "0" => cli::add_password(db, master_key)?,
        "1" => cli::remove_password(db)?,
        "2" => cli::list_passwords(db, master_key)?,
        "3" => cli::get_password_info(db, master_key)?,
        "4" => cli::clear_all_data(db)?,
        "5" => return Ok(false), // Exit
        _ => println!("❌ Invalid option! Please try again."),
    }
    Ok(true)
}

/// Starts the CLI.
pub fn start_cli() -> Result<(), Box<dyn Error>> {
    let mut db = Database::new("passwords.db")?;
    let master_key = get_or_create_master_key(&mut db)?;

    loop {
        display_menu();
        let choice = prompt_input("Choose an option: ")?;
        if !handle_menu_choice(&choice, &mut db, &master_key)? {
            break;
        }
    }
    Ok(())
}
