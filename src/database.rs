use rusqlite::{Connection, params, Result, OptionalExtension};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::utils::queries::SqlQueries as Queries;

/// Represents a password entry in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub website: String,
    pub username: String,
    pub password: Vec<u8>,
}

/// Manages SQLite database operations for password storage.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Creates or opens a SQLite database at the specified path.
    ///
    /// # Arguments
    /// - `path`: The path to the SQLite database file.
    ///
    /// # Returns
    /// - A `Database` instance if successful.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database cannot be opened or initialized.
    pub fn new(path: &str) -> Result<Self> {
        // Open or create the SQLite database file.
        let conn = Connection::open(Path::new(path))?;

        // Initialize the required tables if they don't already exist.
        conn.execute(Queries::CREATE_PASSWORDS_TABLE, [])?;
        conn.execute(Queries::CREATE_MASTER_PASSWORD_TABLE, [])?;

        Ok(Database { conn })
    }

    /// Stores the master password hash in the database.
    ///
    /// # Arguments
    /// - `hash`: The Argon2 hash of the master password.
    ///
    /// # Returns
    /// - `Ok(())` if the hash is stored successfully.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn create_master_password(&self, hash: &str) -> Result<(), rusqlite::Error> {
        self.conn
            .execute(Queries::INSERT_MASTER_PASSWORD, params![hash])
            .map(|_| ())
    }

    /// Retrieves the stored master password hash from the database.
    ///
    /// # Returns
    /// - `Some(hash)` if a master password hash exists.
    /// - `None` if no master password hash is found.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn get_master_password_hash(&self) -> Result<Option<String>, rusqlite::Error> {
        self.conn
            .query_row(Queries::GET_MASTER_PASSWORD_HASH, params![], |row| row.get(0))
            .optional()
    }

    /// Adds a new password entry to the database.
    ///
    /// # Arguments
    /// - `entry`: The `PasswordEntry` to store.
    ///
    /// # Returns
    /// - `Ok(())` if the entry is stored successfully.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn add_password(&mut self, entry: &PasswordEntry) -> Result<(), rusqlite::Error> {
        self.conn
            .execute(
                Queries::INSERT_PASSWORD,
                params![entry.website, entry.username, entry.password],
            )
            .map(|_| ())
    }

    /// Retrieves a password entry from the database by website.
    ///
    /// # Arguments
    /// - `website`: The website associated with the password entry.
    ///
    /// # Returns
    /// - `Some(PasswordEntry)` if the entry is found.
    /// - `None` if no entry is found.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn get_password(&self, website: &str) -> Result<Option<PasswordEntry>> {
        self.conn
            .query_row(
                Queries::GET_PASSWORD,
                params![website],
                |row| {
                    Ok(PasswordEntry {
                        website: row.get(0)?,
                        username: row.get(1)?,
                        password: row.get(2)?,
                    })
                },
            )
            .optional()
    }

    /// Lists all password entries in the database.
    ///
    /// # Returns
    /// - A vector of `PasswordEntry` objects.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn list_passwords(&self) -> Result<Vec<PasswordEntry>> {
        // Prepare the SQL query to list all passwords.
        let mut stmt = self.conn.prepare(Queries::LIST_PASSWORDS)?;

        // Execute the query and map the results to `PasswordEntry` objects.
        let rows = stmt.query_map([], |row| {
            Ok(PasswordEntry {
                website: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })?;

        // Collect the results into a vector.
        let result: Vec<PasswordEntry> = rows.collect::<Result<_, _>>()?;
        Ok(result)
    }

    /// Deletes a password entry from the database by website.
    ///
    /// # Arguments
    /// - `website`: The website associated with the password entry to delete.
    ///
    /// # Returns
    /// - `true` if the entry was deleted.
    /// - `false` if no entry was found.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn delete_password(&self, website: &str) -> Result<bool> {
        let rows_affected = self.conn.execute(Queries::DELETE_PASSWORD, params![website])?;
        Ok(rows_affected > 0)
    }

    /// Clears all password entries from the database.
    ///
    /// # Returns
    /// - `Ok(())` if the operation is successful.
    ///
    /// # Errors
    /// - Returns `rusqlite::Error` if the database operation fails.
    pub fn clear_all_passwords(&self) -> Result<(), rusqlite::Error> {
        self.conn
            .execute(Queries::CLEAR_ALL_PASSWORDS, [])
            .map(|_| ())
    }
}