mod cli;
mod crypto;
mod database;
mod manager;
pub mod utils;

use std::env;
use manager::start_cli;
use utils::api::start_server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--api".to_string()) {
        // Start the API server
        println!("Starting password manager API server on http://127.0.0.1:8080");
        start_server().await?;
    } else {
        // Start the CLI
        start_cli()?;
    }

    Ok(())
}