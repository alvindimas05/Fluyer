use std::sync::Mutex;
use tauri::State;

#[cfg(desktop)]
use tauri::{AppHandle, Emitter};

#[cfg(desktop)]
use tauri_plugin_dialog::DialogExt;

#[cfg(desktop)]
use crate::{store::GLOBAL_APP_STORE, GLOBAL_APP_HANDLE};

use crate::{
    music::{metadata::MusicMetadata, player::MusicPlayer},
    AppState,
};

pub static STORE_PATH_NAME: &str = "music-path";

#[tauri::command]
pub fn music_controller(state: State<'_, Mutex<AppState>>, command: String) {
    let mut state = state.lock().unwrap();
    state.music_player.send_command(command.clone());
}

#[tauri::command]
pub fn music_position_set(state: State<'_, Mutex<AppState>>, position: u64) {
    let mut state = state.lock().unwrap();
    state.music_player.set_pos(position);
}

#[tauri::command]
pub fn music_get_info() -> crate::music::player::MusicPlayerInfo {
    MusicPlayer::get_info()
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
        .add_playlist(playlist);
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
            .emit(crate::commands::route::MUSIC_REQUEST_DIRECTORY, ())
            .unwrap_or_else(|_| {
                eprintln!(
                    "Failed to emit {}",
                    crate::commands::route::MUSIC_REQUEST_DIRECTORY
                )
            })
    });
}

#[tauri::command]
pub fn music_playlist_remove(state: State<'_, Mutex<AppState>>, index: usize) {
    let mut state = state.lock().unwrap();
    state.music_player.remove_playlist(index);
}

#[tauri::command]
pub fn music_set_volume(state: State<'_, Mutex<AppState>>, volume: f32) {
    let mut state = state.lock().unwrap();
    state.music_player.set_volume(volume);
}
