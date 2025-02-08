use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

use crate::database::Database;
use crate::manager;

/// Constants for consistent messaging
mod messages {
    pub const MASTER_PASSWORD_SET: &str = "Master password already set";
    pub const MASTER_PASSWORD_NEEDED: &str = "Master password needs to be set";
}

/// Shared application state with thread-safe mutexes
pub struct AppState {
    pub database_mutex: Mutex<Database>,
    pub encryption_key_mutex: Mutex<Vec<u8>>,
    pub master_key_mutex: Mutex<String>,
}

/// Data structures for API interactions
#[derive(Serialize, Deserialize)]
pub struct PasswordInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct MasterPasswordRequest {
    pub password: String,
}

/// Helper function to safely acquire mutex locks with error handling
fn safe_mutex_lock<'a, T>(
    mutex: &'a Mutex<T>, 
    error_msg: &str
) -> Result<MutexGuard<'a, T>, HttpResponse> {
    mutex.lock().map_err(|_| {
        HttpResponse::InternalServerError().json(error_msg)
    })
}

/// API Endpoint: Check Application Initialization Status
pub async fn initialize(app_state: web::Data<AppState>) -> impl Responder {
    let database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };

    
    match database.get_master_password_data() {
        Ok(Some(_)) => HttpResponse::Ok().json(messages::MASTER_PASSWORD_SET),
        Ok(None) => HttpResponse::Ok().json(messages::MASTER_PASSWORD_NEEDED),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

/// API Endpoint: Set Master Password
pub async fn set_master_password(
    app_state: web::Data<AppState>,
    request_data: web::Json<MasterPasswordRequest>,
) -> impl Responder {
    // Safely acquire locks for database and master key
    let mut database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };
    let mut master_key = match safe_mutex_lock(&app_state.master_key_mutex, "Master key lock error") {
        Ok(mk) => mk,
        Err(err) => return err,
    };

    // Update master key
    *master_key = request_data.password.clone();

    // Create master password and update encryption key
    match manager::create_master_password(false, &mut database, Some(&master_key)) {
        Ok(new_encryption_key) => {
            // Update encryption key in a thread-safe manner
            match app_state.encryption_key_mutex.lock() {
                Ok(mut encryption_key) => {
                    *encryption_key = new_encryption_key;
                    HttpResponse::Ok().json("Master password set successfully")
                },
                Err(_) => HttpResponse::InternalServerError().json("Failed to update encryption key"),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

/// API Endpoint: Verify Master Password
pub async fn verify_master_password(
    app_state: web::Data<AppState>,
    request_data: web::Json<MasterPasswordRequest>,
) -> impl Responder {
    let database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };

    match manager::verify_master_password(&database, &request_data.password) {
        Ok(encryption_key) => {
            // Update encryption key if verification is successful
            if let Ok(mut key) = app_state.encryption_key_mutex.lock() {
                *key = encryption_key;
            }
            HttpResponse::Ok().json("true")
        }
        Err(_) => HttpResponse::Unauthorized().json("false"),
    }
}

/// API Endpoint: Add Password
pub async fn add_password(
    app_state: web::Data<AppState>,
    request_data: web::Json<PasswordInfo>,
) -> impl Responder {
    let mut database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };
    let encryption_key = match safe_mutex_lock(&app_state.encryption_key_mutex, "Key lock error") {
        Ok(key) => key.clone(),
        Err(err) => return err,
    };

    match manager::add_password(
        &mut database,
        &encryption_key,
        &request_data.service,
        &request_data.username,
        &request_data.password,
    ) {
        Ok(_) => HttpResponse::Ok().json("Password added successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

/// API Endpoint: List Passwords
pub async fn list_passwords(app_state: web::Data<AppState>) -> impl Responder {
    let database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };
    let encryption_key = match safe_mutex_lock(&app_state.encryption_key_mutex, "Key lock error") {
        Ok(key) => key.clone(),
        Err(err) => return err,
    };

    match manager::list_passwords(&database, encryption_key.as_slice()) {
        Ok(passwords) => HttpResponse::Ok().json(passwords),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

/// API Endpoint: Get Password
pub async fn get_password(
    app_state: web::Data<AppState>,
    service_name: web::Path<String>,
) -> impl Responder {
    let database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };
    let encryption_key = match safe_mutex_lock(&app_state.encryption_key_mutex, "Key lock error") {
        Ok(key) => key.clone(),
        Err(err) => return err,
    };

    match manager::get_password_info(&database, &encryption_key, &service_name) {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(_) => HttpResponse::NotFound().json("Password not found"),
    }
}

/// API Endpoint: Remove Password
pub async fn remove_password(
    app_state: web::Data<AppState>,
    service_name: web::Path<String>,
) -> impl Responder {
    let mut database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };

    match manager::remove_password(&mut database, &service_name) {
        Ok(_) => HttpResponse::Ok().json("Password removed successfully"),
        Err(_) => HttpResponse::NotFound().json("Password not found"),
    }
}

/// API Endpoint: Clear All Data
pub async fn clear_all_data(app_state: web::Data<AppState>) -> impl Responder {
    let mut database = match safe_mutex_lock(&app_state.database_mutex, "Database lock error") {
        Ok(db) => db,
        Err(err) => return err,
    };

    match manager::clear_all_data(&mut database) {
        Ok(_) => HttpResponse::Ok().json("All data cleared successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

/// Configuration for Cross-Origin Resource Sharing (CORS)
fn create_cors_config() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method()
        .allowed_methods(vec!["GET", "POST", "DELETE"])
        .max_age(3600)
}

/// Start the HTTP server with configured routes
pub async fn start_server() -> std::io::Result<()> {
    // Initialize database
    let database = Database::new("passwords.db").map_err(|e| {
        eprintln!("Database initialization failed: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Database initialization failed")
    })?;

    // Create shared application state
    let app_state = web::Data::new(AppState {
        database_mutex: Mutex::new(database),
        encryption_key_mutex: Mutex::new(Vec::new()),
        master_key_mutex: Mutex::new(String::new()),
    });

    // Configure and start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(create_cors_config())
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/initialize", web::get().to(initialize))
                    .route("/create-master-password", web::post().to(set_master_password))
                    .route("/verify-master-password", web::post().to(verify_master_password))
                    .route("/add-password", web::post().to(add_password))
                    .route("/list-passwords", web::get().to(list_passwords))
                    .route("/get-password/{service}", web::get().to(get_password))
                    .route("/remove-password/{service}", web::delete().to(remove_password))
                    .route("/clear-all-data", web::delete().to(clear_all_data)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}