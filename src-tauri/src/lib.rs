use std::fs::File;

use music::MusicState;

mod music;

#[tauri::command]
fn play(){
    let music_player = music::spawn(MusicState::Paused);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
