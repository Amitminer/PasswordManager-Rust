mod cli;
mod crypto;
mod database;
mod manager;
pub mod utils;

use database::Database;
use manager::{
    add_password, clear_all_data, get_password_info, list_passwords, read_input, remove_password,
};
use std::io::{self, Write};
use zeroize::Zeroize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the database connection.
    let mut db = Database::new("passwords.db")?;

    // Check if a master password exists.
    // If it does, verify the user's input. If not, create a new master password.
    let key = if db.get_master_password_hash()?.is_some() {
        manager::verify_master_password(&db)?
    } else {
        manager::create_master_password(&mut db)?
    };

    // Create a copy of the key for secure zeroization later.
    let mut key_copy = key.clone();

    // Main application loop.
    loop {
        // Display the main menu options.
        println!("\nWelcome to Password Manager:");
        let options = [
            "Add new password",
            "Remove password",
            "List passwords",
            "Get password info",
            "Clear all data",
            "Exit",
        ];

        // Print each option with its corresponding index.
        for (index, option) in options.iter().enumerate() {
            println!("[{}] {}", index, option);
        }

        // Prompt the user to choose an option.
        print!("Choose an option: ");
        io::stdout().flush()?;

        // Read the user's choice.
        let choice = read_input()?;

        // Handle the user's choice.
        match choice.as_str() {
            "0" => add_password(&mut db, &key)?, // Add a new password.
            "1" => remove_password(&mut db)?,    // Remove a password.
            "2" => list_passwords(&db)?,         // List all passwords.
            "3" => get_password_info(&db, &key)?, // Get details for a specific password.
            "4" => clear_all_data(&mut db)?,     // Clear all stored passwords.
            "5" => break,                        // Exit the application.
            _ => println!("Invalid option! Please try again."), // Handle invalid input.
        }
    }

    // Securely clear the key copy from memory.
    key_copy.zeroize();

    Ok(())
}