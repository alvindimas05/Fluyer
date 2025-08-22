use std::sync::Mutex;
use tauri::{Manager, State};

#[cfg(desktop)]
use tauri::{AppHandle, Emitter};
use tauri::path::BaseDirectory;
#[cfg(desktop)]
use tauri_plugin_dialog::DialogExt;

#[cfg(desktop)]
use crate::{store::GLOBAL_APP_STORE};

use crate::GLOBAL_APP_HANDLE;

use crate::{logger, music::metadata::MusicMetadata, AppState};
use crate::music::player::MusicPlayer;

pub static MUSIC_STORE_PATH_NAME: &str = "music-path";

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

// #[tauri::command]
// pub fn music_get_info() -> crate::music::player::MusicPlayerSync {
//     MusicPlayer::get_sync_info()
// }

#[tauri::command]
pub fn music_get_all() -> Option<Vec<MusicMetadata>> {
    crate::file::get_all_music()
}

#[tauri::command]
pub fn music_playlist_add(state: State<'_, Mutex<AppState>>, playlist: Vec<MusicMetadata>) {
    let mut state = state.lock().unwrap();
    state.music_player.add_playlist(playlist);
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
            .set(MUSIC_STORE_PATH_NAME, dir);

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

#[tauri::command]
pub fn music_playlist_goto(state: State<'_, Mutex<AppState>>, index: usize) {
    let mut state = state.lock().unwrap();
    state.music_player.goto_playlist(index);
}

#[tauri::command]
pub fn music_playlist_moveto(state: State<'_, Mutex<AppState>>, from: usize, to: usize) {
    let mut state = state.lock().unwrap();
    state.music_player.moveto_playlist(from, to);
}

#[tauri::command]
pub fn music_equalizer(state: State<'_, Mutex<AppState>>, values: Vec<f32>) {
    let state = state.lock().unwrap();
    state.music_player.equalizer(values);
}

#[tauri::command]
pub fn music_get_image(path: String) -> Option<String> {
    MusicMetadata::get_image_from_path(path)
}

#[tauri::command]
pub fn music_get_buffer(path: String) -> Option<Vec<u8>> {
    if cfg!(mobile) {
        // FIXME: Mobile platforms do not support ffmpeg conversion
        return std::fs::read(path).ok();
    }

    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::path::PathBuf;

    // Get the bundled ffmpeg path
    let ffmpeg_binary = if cfg!(target_os = "windows") {
        "libs/ffmpeg/bin/ffmpeg.exe"
    } else {
        "libs/ffmpeg/bin/ffmpeg"
    };

    let ffmpeg_path = GLOBAL_APP_HANDLE.get().unwrap().path()
        .resolve(ffmpeg_binary, BaseDirectory::Resource).unwrap();

    let tmp_dir = std::env::temp_dir();
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let tmp_file: PathBuf = tmp_dir.join(format!("converted_{}.mp3", unique_id));

    // Convert to very small mp3
    let ffmpeg_status = Command::new(&ffmpeg_path)
        .args([
            "-y", // overwrite
            "-i", &path, // input
            "-ac", "1",  // mono
            "-ar", "44100", // lower the sample rate
            "-b:a", "192k", // lower the bitrate
            "-c:a", "libmp3lame", // MP3 encoder
            "-map", "0:a", // only audio
            tmp_file.to_str().unwrap(),
        ])
        .status();

    if ffmpeg_status.is_err() {
        logger::error!("Failed to convert audio to mp3: {}", ffmpeg_status.unwrap_err());
        return std::fs::read(path).ok();
    }

    if !tmp_file.exists() {
        logger::error!("Failed to convert audio to mp3");
        return std::fs::read(path).ok();
    }

    let data = std::fs::read(&tmp_file).ok();

    let _ = std::fs::remove_file(tmp_file);

    data
}

#[tauri::command]
pub fn music_get_current_duration(state: State<'_, Mutex<AppState>>) -> Option<u128> {
    Some(state.lock().unwrap().music_player.get_current_duration())
}

#[tauri::command]
pub fn music_request_sync() {
    MusicPlayer::emit_sync(false);
}