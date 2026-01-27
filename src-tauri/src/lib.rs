// Core modules
pub mod animated_background;
mod api;
pub(crate) mod commands;
mod coverart;
mod database;
mod folder;
pub mod logger;
mod lyric;
mod music;
mod playlist;
mod system;
mod utils;
mod wgpu_renderer;

// Application modules
mod app_setup;
mod events;
pub(crate) mod state;

// Re-export platform module from main
pub mod platform;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(app_setup::prevent_default_plugin())
        .plugin(tauri_plugin_fluyer::init())
        .setup(app_setup::setup_application)
        .invoke_handler(commands::COMMAND_HANDLERS)
        .on_window_event(events::handle_window_events)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(events::handle_app_events);
}
