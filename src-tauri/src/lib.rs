use std::sync::Mutex;
use tauri::{Builder, Manager};
use music::player::MusicPlayer;

mod music;
mod commands;
mod file;

struct AppState {
    music_player: MusicPlayer
}
impl AppState {
    fn default() -> Self {
        Self { music_player: MusicPlayer::spawn() }
    }
}

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
