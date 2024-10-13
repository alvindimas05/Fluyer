use std::sync::Mutex;
use file::MusicMetadata;
use tauri::{Builder, Manager};
use music::MusicPlayer;

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
    println!("{:?}", MusicMetadata::new().get());
    // Builder::default()
    //     .setup(|app| {
    //         app.manage(Mutex::new(AppState::default()));
    //         Ok(())
    //     })
    //     .plugin(tauri_plugin_shell::init())
    //     .invoke_handler(tauri::generate_handler![
    //         commands::music_controller,
    //         commands::music_set_position
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
