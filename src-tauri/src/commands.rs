use std::sync::Mutex;
use tauri::State;

use crate::{music::metadata::MusicMetadata, AppState};

#[tauri::command]
pub fn music_controller(state: State<'_, Mutex<AppState>>, command: String, message: String){
    let mut state = state.lock().unwrap();
    state.music_player.set_path(String::from("test-music.flac")).expect("Can't change music player name");
    if command == "play" {
        state.music_player.play().expect("Can't play music player.");
    } else if command == "pause" || command == "stop" {
        state.music_player.pause().expect("Can't pause music player.");
    }
}

#[tauri::command]
pub fn music_set_position(state: State<'_, Mutex<AppState>>, position: u64){
    let mut state = state.lock().unwrap();
    state.music_player.set_pos(position).expect("Can't set music player position");
}

#[tauri::command]
pub fn music_get_all() -> Vec<MusicMetadata> {
    let musics = crate::file::get_all_music();
    musics
}