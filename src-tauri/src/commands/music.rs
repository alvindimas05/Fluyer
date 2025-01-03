use std::sync::Mutex;
use tauri::State;

#[cfg(desktop)]
use tauri::{AppHandle, Emitter};

#[cfg(desktop)]
use tauri_plugin_dialog::DialogExt;

#[cfg(desktop)]
use crate::{store::GLOBAL_APP_STORE, GLOBAL_APP_HANDLE};

use crate::{music::metadata::MusicMetadata, AppState};

pub static STORE_PATH_NAME: &str = "music-path";

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
pub fn music_get_all() -> Option<Vec<MusicMetadata>> {
    crate::file::get_all_music()
}

#[tauri::command]
pub fn music_playlist_add(state: State<'_, Mutex<AppState>>, playlist: Vec<String>) {
    let mut state = state.lock().unwrap();
    state
        .music_player
        .add_playlist(playlist)
        .expect("Can't set playlist.");
}

#[cfg(desktop)]
#[tauri::command]
pub fn music_request_directory(app: AppHandle) {
    app.dialog().file().pick_folder(|dir_path| {
        let dir = dir_path
            .unwrap()
            .into_path()
            .expect("Failed to get music dir path.")
            .into_os_string()
            .into_string()
            .expect("Failed to get music dir path.");

        GLOBAL_APP_STORE
            .get()
            .expect("Failed to get GLOBAL_APP_STORE")
            .set(STORE_PATH_NAME, dir);

        GLOBAL_APP_HANDLE
            .get()
            .unwrap()
            .emit(crate::commands::route::MUSIC_REQUEST_DIR, ())
            .expect(format!("Failed to emit {}", crate::commands::route::MUSIC_REQUEST_DIR).as_str())
    });
}
