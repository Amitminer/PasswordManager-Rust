---
# 🔐 Password Manager  

A secure password manager with **AES-256-GCM encryption**, **Argon2 hashing**, and a **web API**.  
Supports both **CLI** and **Web API** for managing passwords securely.  

---

## 🚀 Features  
✅ **Master Password Protection** (Argon2-based hashing)  
✅ **AES-256-GCM Encryption** (Secure password storage)  
✅ **SQLite Database** (Encrypted storage for passwords)  
✅ **Web API Support** (Actix-Web based API)  
✅ **Zeroization** (Sensitive data is securely erased from memory)  
✅ **Frontend & CLI Support** (Use via Web UI or CLI)  

---

## 📦 Installation  

### **1️⃣ Frontend Setup**  
The frontend is built using **Next.js**.  

#### **Requirements:**  
- **Node.js** (Download from [nodejs.org](https://nodejs.org/))  
- **Git** (Download from [git-scm.com](https://git-scm.com/))  

#### **Install & Run**  
```sh
git clone https://github.com/Amitminer/PasswordManager-Rust.git
cd PasswordManager-Rust
cd website
npm install
npm run dev
```
---

### **2️⃣ Backend Setup**  
The backend is built in **Rust** using Actix-Web.  

#### **Requirements:**  
- **Rust & Cargo** ([Install Rust](https://rustup.rs/))  
- **SQLite** (Bundled with `rusqlite`)  

#### **Install & Run**  
```sh
git clone https://github.com/Amitminer/PasswordManager-Rust.git
cd PasswordManager-Rust
cargo build --release
cargo run -- --api  # OR run compiled binary:
./passwordmanager --api
``

This starts the **Web API** at `http://127.0.0.1:8080/api`.  

---

## 🌍 Web API Usage  

### **Available Endpoints**  
| Method | Endpoint | Description |
|--------|---------|-------------|
| `GET` | `/api/initialize` | Check if the master password is set |
| `POST` | `/api/create-master-password` | Set the master password |
| `POST` | `/api/verify-master-password` | Verify master password |
| `POST` | `/api/add-password` | Add a new password |
| `GET` | `/api/list-passwords` | List stored passwords |
| `GET` | `/api/get-password/{service}` | Retrieve a specific password |
| `DELETE` | `/api/remove-password/{service}` | Remove a password |
| `DELETE` | `/api/clear-all-data` | Delete all stored passwords |

---

## 🐍 API Example (Python Requests)  

### **1️⃣ Install Dependencies**  
```sh
pip install requests
```

### **2️⃣ Sample Python Client**
```python
import requests

BASE_URL = "http://127.0.0.1:8080/api"

def set_master_password(password):
    return requests.post(f"{BASE_URL}/create-master-password", json={"password": password}).json()

def verify_master_password(password):
    return requests.post(f"{BASE_URL}/verify-master-password", json={"password": password}).json()

def add_password(service, username, password):
    return requests.post(f"{BASE_URL}/add-password", json={"service": service, "username": username, "password": password}).json()

def list_passwords():
    return requests.get(f"{BASE_URL}/list-passwords").json()

def get_password(service):
    response = requests.get(f"{BASE_URL}/get-password/{service}")
    return response.json() if response.status_code == 200 else f"Error: {response.text}"

def remove_password(service):
    return requests.delete(f"{BASE_URL}/remove-password/{service}").json()

def clear_all_data():
    return requests.delete(f"{BASE_URL}/clear-all-data").json()

if __name__ == "__main__":
    master_password = "my_secure_master_password"

    print("🔑 Setting Master Password...")
    print(set_master_password(master_password))

    print("\n✅ Verifying Master Password...")
    print(verify_master_password(master_password))

    print("\n🔐 Adding a Password Entry...")
    print(add_password("example.com", "admin", "secure123"))

    print("\n📜 Listing Stored Passwords...")
    print(list_passwords())

    print("\n🔎 Retrieving Password for 'example.com'...")
    print(get_password("example.com"))

    print("\n❌ Removing Password for 'example.com'...")
    print(remove_password("example.com"))

    print("\n⚠️ Clearing All Data...")
    print(clear_all_data())
```

## 🔗 CLI Usage  

### **1️⃣ Running the CLI**  
```sh
cargo run
```
or  
```sh
./passwordmanager
```

### **2️⃣ Available CLI Actions**  
- **Add a new password**  
- **Retrieve stored passwords**  
- **List all stored websites and usernames**  
- **Remove passwords**  
- **Clear all stored data**  

### **3️⃣ Security**  
- **Use a strong master password**  
- **Passwords are encrypted with AES-256-GCM**  
- **Data is erased from memory after use (zeroization)**  

---

## 🔒 Security  

### **1️⃣ Argon2 Password Hashing**  
- **Memory Cost**: 19 MB  
- **Time Cost**: 2 iterations  
- **Parallelism**: 1 thread  

### **2️⃣ AES-256-GCM Encryption**  
- **AES-GCM** ensures password confidentiality & integrity.  
- **Unique nonce per encryption** to prevent attacks.  

---

## 🏗 Why This Project?  
This project was built to learn:  
✅ Secure **password storage & encryption**  
✅ Cryptographic techniques (**Argon2, AES-256-GCM**)  
✅ Database security with **SQLite**  
✅ **Web API Development** in Rust with **Actix-Web**  
✅ **Frontend & Backend Security Best Practices**  

---

## ⚙️ Dependencies  

### **📦 Rust Crates Used:**  
- `actix-web` – Web API framework  
- `rusqlite` – SQLite database  
- `argon2` – Password hashing  
- `aes-gcm` – AES-256-GCM encryption  
- `rpassword` – Secure password input  
- `zeroize` – Secure memory wiping  

### **🌍 Frontend Dependencies:**  
- `Next.js` – React-based web framework  
- `Tailwind CSS` – UI styling  
- `Axios` – API calls  

---

## 📜 License  
This project is **open-source** and licensed under the **MIT License**.  

--- 
```
