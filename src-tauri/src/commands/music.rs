use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

use crate::{music::metadata::MusicMetadata, AppState, GLOBAL_APP_HANDLE};

#[tauri::command]
pub fn music_controller(state: State<'_, Mutex<AppState>>, command: String) {
    let mut state = state.lock().unwrap();
    if command == "play" {
        state.music_player.play().expect("Can't play music player.");
    } else if command == "pause" || command == "stop" {
        state
            .music_player
            .pause()
            .expect("Can't pause music player.");
    } else if command == "next" {
        state.music_player.next().expect("Can't next music player.");
    }
}

#[tauri::command]
pub fn music_position_set(state: State<'_, Mutex<AppState>>, position: u64) {
    let mut state = state.lock().unwrap();
    state
        .music_player
        .set_pos(position)
        .expect("Can't set music player position.");
}

#[tauri::command]
pub fn music_get_info(state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state
        .music_player
        .get_info()
        .expect("Can't get music player position.");
}

#[tauri::command]
pub fn music_get_all() -> Vec<MusicMetadata> {
    let musics = crate::file::get_all_music();
    musics
}

#[tauri::command]
pub fn music_playlist_add(state: State<'_, Mutex<AppState>>, playlist: Vec<String>) {
    let mut state = state.lock().unwrap();
    state
        .music_player
        .add_playlist(playlist)
        .expect("Can't set playlist.");
}

#[tauri::command]
pub fn music_request_dir(app: AppHandle) {
    app.dialog().file().pick_folder(|dir_path| {
        if let Some(app_handle) = GLOBAL_APP_HANDLE.get() {
            let dir = dir_path
                .unwrap()
                .into_path()
                .expect("Failed to get music dir path.")
                .into_os_string()
                .into_string()
                .expect("Failed to get music dir path.");
            
            app_handle
                .emit("music_request_dir", ())
                .expect("Can't emit music_get_dir_path");
        } else {
            log::error!("Failed to get GLOBAL_APP_HANDLE");
        }
    });
}
