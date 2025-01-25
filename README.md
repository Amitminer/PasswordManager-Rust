---

# Password Manager

This is a **Password Manager** project built for learning purposes. It demonstrates secure password storage, encryption, and database management using modern cryptographic techniques.

---

## Features

- **Master Password Protection**: Uses Argon2 for secure password hashing and verification.
- **Encryption**: Passwords are encrypted using AES-256-GCM before storage.
- **Database**: SQLite is used to store encrypted passwords and the master password hash.
- **Zeroization**: Sensitive data (e.g., keys, passwords) is securely erased from memory after use.
- **Command-Line Interface (CLI)**: Easy-to-use menu for managing passwords.

---

## How It Works

### 1. **Master Password**
- The master password is hashed using **Argon2**, a modern password hashing algorithm designed to resist brute-force and side-channel attacks.
- The hash is stored in the SQLite database.
- During login, the entered password is verified against the stored hash.

### 2. **Encryption**
- Passwords are encrypted using **AES-256-GCM**, a secure symmetric encryption algorithm.
- A unique **nonce** is generated for each encryption operation to ensure ciphertext uniqueness.
- The encryption key is derived from the master password using Argon2.

### 3. **Database**
- SQLite is used to store:
  - The master password hash.
  - Encrypted passwords (website, username, and encrypted password).
- The database is initialized with two tables:
  - `master_password`: Stores the Argon2 hash of the master password.
  - `passwords`: Stores encrypted password entries.

### 4. **Security**
- **Argon2** is used for key derivation and password hashing, with configurable parameters:
  - Memory cost: 19 MB
  - Time cost: 2 iterations
  - Parallelism: 1 thread
- **AES-256-GCM** ensures confidentiality and integrity of encrypted passwords.
- Sensitive data (e.g., keys, passwords) is zeroized from memory after use to prevent leaks.

---

## Getting Started

### Prerequisites
- Rust (install from [rustup.rs](https://rustup.rs/))
- SQLite (for database storage, already bundled with crate)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Amitminer/PasswordManager-Rust.git
   cd PasswordManager-Rust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the password manager:
   ```bash
   cargo run --release
   ```

---

## Usage

1. **First Run**:
   - You will be prompted to create a master password.
   - The master password hash will be stored in the database.

2. **Main Menu**:
   - Add new passwords.
   - Retrieve stored passwords.
   - List all stored websites and usernames.
   - Remove passwords.
   - Clear all data.

3. **Security**:
   - Always use a strong master password.
   - Do not share your master password.

---

## Why I Built This

This project was created as a learning exercise to understand:
- Secure password storage and encryption.
- Cryptographic algorithms like Argon2 and AES-256-GCM.
- Database management with SQLite.
- Memory safety and zeroization in Rust.

---

## Dependencies

- **Rust Crates**:
  - `aes-gcm`: For AES-256-GCM encryption.
  - `argon2`: For password hashing and key derivation.
  - `rusqlite`: For SQLite database operations.
  - `rpassword`: For secure password input.
  - `zeroize`: For securely erasing sensitive data from memory.

---
