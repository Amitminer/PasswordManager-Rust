use std::error::Error;
use std::io::{self, Write};
use clap::{Parser, Subcommand};
use crate::crypto::{decrypt, encrypt};
use crate::database::{Database, PasswordEntry};

/// Command line interface for the password manager.
#[derive(Parser)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new password entry.
    Add {
        /// Website for the password.
        website: String,
        /// Username for the website.
        username: String,
    },
    /// Get a password entry.
    Get {
        /// Website to retrieve the password for.
        website: String,
    },
    /// List all password entries.
    List,
    /// Delete a password entry.
    Delete {
        /// Website to delete the password for.
        website: String,
    },
}

/// Reads input from the user and trims whitespace.
pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Prints a message and reads user input.
pub fn prompt_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;  // Flush to ensure the prompt appears before input
    read_input()
}

/// Asks for confirmation (y/n) before proceeding.
pub fn confirm_action(prompt: &str) -> io::Result<bool> {
    let response = prompt_input(prompt)?;
    Ok(response.eq_ignore_ascii_case("y") || response.eq_ignore_ascii_case("yes"))
}

/// Adds a new password entry after encrypting it.
pub fn add_password(db: &mut Database, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let website = prompt_input("Enter the website: ")?;
    let username = prompt_input("Enter the username: ")?;
    let password = rpassword::prompt_password("Enter the password: ")?;

    let encrypted_password = encrypt(password.as_bytes(), key)?;

    db.add_password(&PasswordEntry {
        website,
        username,
        password: encrypted_password,
    })?;

    println!("✅ Password added successfully!");
    Ok(())
}

/// Lists all saved password entries.
pub fn list_passwords(db: &Database, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let passwords = db.list_passwords()?;

    if passwords.is_empty() {
        println!("🔍 No passwords found.");
    } else {
        println!("🔐 Saved Passwords:");
        for entry in passwords {
            match decrypt(&entry.password, key) {
                Ok(decrypted_password) => {
                    println!(
                        "- 🌐 Website: {}, 👤 Username: {}, 🔑 Password: {}",
                        entry.website,
                        entry.username,
                        String::from_utf8_lossy(&decrypted_password)
                    );
                }
                Err(_) => println!("❌ Failed to decrypt password for {}", entry.website),
            }
        }
    }
    Ok(())
}

/// Retrieves and decrypts a specific password entry.
pub fn get_password_info(db: &Database, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let website = prompt_input("Enter the website: ")?;

    match db.get_password(&website)? {
        Some(entry) => match decrypt(&entry.password, key) {
            Ok(decrypted_password) => println!(
                "🔓 Website: {}, 👤 Username: {}, 🔑 Password: {}",
                entry.website,
                entry.username,
                String::from_utf8_lossy(&decrypted_password)
            ),
            Err(_) => println!("❌ Failed to decrypt password."),
        },
        None => println!("❌ No password found for {}", website),
    }
    Ok(())
}

/// Removes a stored password entry.
pub fn remove_password(db: &mut Database) -> Result<(), Box<dyn Error>> {
    let website = prompt_input("Enter the website to remove: ")?;

    if db.delete_password(&website)? {
        println!("✅ Password for {} removed successfully!", website);
    } else {
        println!("❌ No password found for {}.", website);
    }
    Ok(())
}

/// Clears all stored passwords with confirmation.
pub fn clear_all_data(db: &mut Database) -> Result<(), Box<dyn Error>> {
    if confirm_action("⚠️ Are you sure you want to clear all data? (y/n): ")? {
        db.clear_all_passwords()?;
        println!("✅ All passwords have been deleted!");
    } else {
        println!("❌ Operation canceled.");
    }
    Ok(())
}
