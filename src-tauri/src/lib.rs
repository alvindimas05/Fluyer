// Core modules
mod api;
mod commands;
mod database;
mod file;
mod music;
mod playlist;
mod store;
mod utils;
mod logger;

// Application modules
mod app_setup;
mod events;
mod state;

// Re-export platform module from main
pub mod platform;

use app_setup::setup_application;
use events::handle_window_events;
use crate::commands::COMMAND_HANDLERS;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(app_setup::prevent_default_plugin())
        .plugin(tauri_plugin_fluyer::init())
        .setup(setup_application)
        .invoke_handler(COMMAND_HANDLERS)
        .on_window_event(handle_window_events)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(state::handle_app_events);
}