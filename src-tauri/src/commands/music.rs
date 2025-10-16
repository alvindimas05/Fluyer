use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use tauri::path::BaseDirectory;
#[cfg(desktop)]
use tauri::Emitter;
#[cfg(desktop)]
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fluyer::FluyerExt;
#[cfg(desktop)]
use crate::store::GLOBAL_APP_STORE;

use crate::GLOBAL_APP_HANDLE;

use crate::music::player::MusicPlayer;
use crate::{logger, music::metadata::MusicMetadata, AppState};

use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
pub fn music_playlist_goto_desktop(state: State<'_, Mutex<AppState>>, music: MusicMetadata) {
    let mut state = state.lock().unwrap();
    state.music_player.goto_playlist_desktop(music);
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
pub async fn music_get_image(path: String, size: Option<String>) -> Option<String> {
    MusicMetadata::get_image_from_path_async(path, size).await
}

#[tauri::command]
pub async fn music_get_visualizer_buffer(app_handle: AppHandle, path: String) -> Option<Vec<u8>> {
    tokio::task::spawn_blocking(move || {
        #[cfg(desktop)]
        let dir = std::env::temp_dir();

        #[cfg(mobile)]
        let dir = app_handle.path().app_cache_dir().ok().unwrap();

        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let tmp_file: PathBuf = dir.join(format!("converted_{}.mp3", unique_id));

        #[cfg(desktop)]
        let music_path = &path;
        #[cfg(target_os = "android")]
        let music_path = format!("\"{}\"", &path);

        let mut args = vec![];
        #[cfg(desktop)]
        args.push("-y"); // overwrite
        args.extend([
            "-i", music_path.as_str(), // input
            "-ac", "1", // mono
            "-ar", "44100", // fixed sample rate
            "-b:a", "192k", // fixed bitrate
            "-q:a", "5", // ~45-55 kbps
            // "-c:a", "libmp3lame", // mp3 encoder
            "-map", "0:a", // only audio
            tmp_file.to_str().unwrap(),
        ]);

        #[cfg(mobile)]{
            let result = app_handle.fluyer().visualizer_get_buffer(args.join(" "));
            if result.is_err() {
                println!(
                    "Failed to convert audio to mp3: {}",
                    result.unwrap_err()
                );
                return std::fs::read(path).ok();
            }

            if !result.unwrap().value {
                println!("Failed to convert audio to mp3");
                return std::fs::read(path).ok();
            }

            return std::fs::read(tmp_file).ok();
        }
        #[cfg(desktop)]{
            // Get the bundled ffmpeg path
            let ffmpeg_binary = if cfg!(target_os = "windows") {
                "libs/ffmpeg/bin/ffmpeg.exe"
            } else {
                "libs/ffmpeg/bin/ffmpeg"
            };

            let ffmpeg_path = app_handle.path()
                .resolve(ffmpeg_binary, BaseDirectory::Resource)
                .unwrap();

            #[cfg(any(target_os = "linux", target_os = "macos"))]{
                // Make sure ffmpeg is executable
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = std::fs::metadata(&ffmpeg_path) {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o755); // rwxr-xr-x
                    let _ = std::fs::set_permissions(&ffmpeg_path, permissions);
                }
            }

            let ffmpeg_status = {
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    Command::new(&ffmpeg_path)
                        .args(&args)
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW on Windows
                        .status()
                }
                #[cfg(not(windows))]
                {
                    Command::new(&ffmpeg_path).args(&args).status()
                }
            };

            if ffmpeg_status.is_err() {
                logger::error!(
                    "Failed to convert audio to mp3: {}",
                    ffmpeg_status.unwrap_err()
                );
                return std::fs::read(path).ok();
            }

            if !tmp_file.exists() {
                logger::error!("Failed to convert audio to mp3");
                return std::fs::read(path).ok();
            }

            let data = std::fs::read(&tmp_file).ok();

            let _ = std::fs::remove_file(tmp_file);

            return data;
        }
    }).await.ok().flatten()
}

#[tauri::command]
pub fn music_get_current_duration(state: State<'_, Mutex<AppState>>) -> Option<u128> {
    Some(state.lock().unwrap().music_player.get_current_duration())
}

#[tauri::command]
pub fn music_request_sync() {
    MusicPlayer::emit_sync(false);
}


#[tauri::command]
pub fn music_get_lyrics(path: String) -> Option<String> {
    MusicMetadata::get_lyrics_from_path(path)
}