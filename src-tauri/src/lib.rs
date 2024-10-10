use std::sync::Mutex;
use tauri::{Builder, Manager, State};
use music::MusicPlayer;

mod music;

#[tauri::command]
fn music_controller(state: State<'_, Mutex<AppState>>, command: String){
    let mut state = state.lock().unwrap();
    if command == "play" {
        state.music_player.play();
    } else if command == "pause" || command == "stop" {
        state.music_player.pause();
    }
}

struct AppState {
    music_player: MusicPlayer
}
impl AppState {
    fn default() -> Self {
        let mut music_player = MusicPlayer::spawn();
        music_player.set_music_path(String::from("../test-music.flac"));
            
        Self { music_player }
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
        .invoke_handler(tauri::generate_handler![music_controller])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
