use tauri::State;

use crate::{music::metadata::MusicMetadata, state::AppState};

#[tauri::command]
pub fn music_queue_add(state: State<AppState>, playlist: Vec<MusicMetadata>) {
    state.music_player.add_playlist(playlist);
}

#[tauri::command]
pub fn music_queue_remove(state: State<AppState>, index: usize) {
    state.music_player.remove_playlist(index);
}

#[tauri::command]
pub fn music_queue_goto(state: State<AppState>, index: usize) {
    state.music_player.goto_playlist(index);
}

#[tauri::command]
pub fn music_queue_moveto(state: State<AppState>, from: usize, to: usize) {
    state.music_player.moveto_playlist(from, to);
}
