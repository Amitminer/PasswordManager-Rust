/// A struct containing all SQL queries used in the application.
pub struct SqlQueries;

impl SqlQueries {
    // Database initialization queries
    /// Creates the `passwords` table if it doesn't already exist.
    /// Columns:
    /// - `website`: TEXT (Primary Key)
    /// - `username`: TEXT (Not Null)
    /// - `password`: BLOB (Not Null)
    pub const CREATE_PASSWORDS_TABLE: &str = "
        CREATE TABLE IF NOT EXISTS passwords (
            website TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            password BLOB NOT NULL
        )";

    /// Creates the `master_password` table if it doesn't already exist.
    /// Columns:
    /// - `id`: INTEGER (Primary Key, Always 1)
    /// - `hash`: TEXT (Not Null)
    pub const CREATE_MASTER_PASSWORD_TABLE: &str = "
        CREATE TABLE IF NOT EXISTS master_password (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            hash TEXT NOT NULL
        )";

    // Master password operations
    /// Inserts or replaces the master password hash in the `master_password` table.
    /// Parameters:
    /// - `?1`: The Argon2 hash of the master password.
    pub const INSERT_MASTER_PASSWORD: &str = "
        INSERT OR REPLACE INTO master_password (id, hash) VALUES (1, ?1)";

    /// Retrieves the master password hash from the `master_password` table.
    pub const GET_MASTER_PASSWORD_HASH: &str = "
        SELECT hash FROM master_password WHERE id = 1";

    // Password operations
    /// Inserts a new password entry into the `passwords` table.
    /// Parameters:
    /// - `?1`: The website.
    /// - `?2`: The username.
    /// - `?3`: The encrypted password.
    pub const INSERT_PASSWORD: &str = "
        INSERT INTO passwords (website, username, password) VALUES (?1, ?2, ?3)";

    /// Retrieves a password entry from the `passwords` table by website.
    /// Parameters:
    /// - `?1`: The website to search for.
    pub const GET_PASSWORD: &str = "
        SELECT website, username, password FROM passwords WHERE website = ?1";

    /// Lists all password entries in the `passwords` table.
    pub const LIST_PASSWORDS: &str = "
        SELECT website, username, password FROM passwords";

    /// Deletes a password entry from the `passwords` table by website.
    /// Parameters:
    /// - `?1`: The website to delete.
    pub const DELETE_PASSWORD: &str = "
        DELETE FROM passwords WHERE website = ?1";

    /// Clears all password entries from the `passwords` table.
    pub const CLEAR_ALL_PASSWORDS: &str = "
        DELETE FROM passwords";
}