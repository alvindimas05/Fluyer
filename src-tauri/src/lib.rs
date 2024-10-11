use std::sync::Mutex;
use tauri::{Builder, Manager, State};
use music::MusicPlayer;

mod music;

#[tauri::command]
fn music_controller(state: State<'_, Mutex<AppState>>, command: String){
    let mut state = state.lock().unwrap();
    state.music_player.set_path(String::from("test-music.flac")).expect("Can't change music player name");
    if command == "play" {
        state.music_player.play().expect("Can't play music player.");
    } else if command == "pause" || command == "stop" {
        state.music_player.pause().expect("Can't pause music player.");
    }
}

#[tauri::command]
fn music_set_position(state: State<'_, Mutex<AppState>>, position: u64){
    let mut state = state.lock().unwrap();
    state.music_player.set_pos(position).expect("Can't set music player position");
}

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
            music_controller, music_set_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
