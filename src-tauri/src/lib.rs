use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Builder, Manager};
use music::player::MusicPlayer;

mod music;
mod commands;
mod file;
mod tests;

struct AppState {
    music_player: MusicPlayer
}
impl AppState {
    fn default() -> Self {
        Self { music_player: MusicPlayer::spawn() }
    }
}

static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::music::music_controller,
            commands::music::music_position_set,
            commands::music::music_get_all,
            commands::music::music_playlist_add,
            commands::music::music_get_info,
            
            commands::log::log_error,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready => { 
                GLOBAL_APP_HANDLE.set(app_handle.clone()).expect("Failed to set GLOBAL_APP_HANDLE");
            }
            _ => {}
        });
}
