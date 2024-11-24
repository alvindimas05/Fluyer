use std::sync::Mutex;
use tauri::State;

use crate::{music::metadata::MusicMetadata, AppState};

#[tauri::command]
pub fn music_controller(state: State<'_, Mutex<AppState>>, command: String){
    let mut state = state.lock().unwrap();
    if command == "play" {
        state.music_player.play().expect("Can't play music player.");
    } else if command == "pause" || command == "stop" {
        state.music_player.pause().expect("Can't pause music player.");
    } else if command == "next" {
        state.music_player.next().expect("Can't next music player.");
    }
}

#[tauri::command]
pub fn music_position_set(state: State<'_, Mutex<AppState>>, position: u64){
    let mut state = state.lock().unwrap();
    state.music_player.set_pos(position).expect("Can't set music player position.");
}


#[tauri::command]
pub fn music_get_info(state: State<'_, Mutex<AppState>>){
    let mut state = state.lock().unwrap();
    state.music_player.get_info().expect("Can't get music player position.");
}

#[tauri::command]
pub fn music_get_all() -> Vec<MusicMetadata> {
    let musics = crate::file::get_all_music();
    musics
}

#[tauri::command]
pub fn music_playlist_add(state: State<'_, Mutex<AppState>>, playlist: Vec<String>){
    let mut state = state.lock().unwrap();
    state.music_player.add_playlist(playlist).expect("Can't set playlist.");
}