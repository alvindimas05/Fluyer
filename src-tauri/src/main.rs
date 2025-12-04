// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod platform;

fn main() {
    // Initialize platform-specific configurations
    platform::init();

    // Initialize logging for desktop platforms
    #[cfg(desktop)]
    logger::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Run the application
    fluyer_lib::run()
}