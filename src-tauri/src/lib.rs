use crate::store::init_store;
use music::player::MusicPlayer;
use std::sync::{Mutex, OnceLock};
use tauri::AppHandle;
#[allow(unused)]
use tauri::{Manager, RunEvent};

mod commands;
mod file;
mod music;
mod platform;
mod store;

struct AppState {
    music_player: MusicPlayer,
}
impl AppState {
    fn default() -> Self {
        Self {
            music_player: MusicPlayer::spawn(),
        }
    }
}

static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_fluyer::init())
        .setup(|app| {
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            {
                let window = app.get_webview_window("main").unwrap();
                window
                    .set_decorations(false)
                    .expect("Failed to set decorations on MacOS");
            }

            app.manage(Mutex::new(AppState::default()));
            init_store(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::music::music_controller,
            commands::music::music_position_set,
            commands::music::music_get_all,
            commands::music::music_playlist_add,
            commands::music::music_get_info,
            #[cfg(desktop)]
            commands::music::music_request_dir,
            commands::log::log_error,
            #[cfg(mobile)]
            commands::log::toast,
            #[cfg(mobile)]
            commands::mobile::request_read_audio_permission,
            #[cfg(mobile)]
            commands::mobile::get_navigation_bar_height,
            #[cfg(mobile)]
            commands::mobile::get_navigation_bar_size,
            #[cfg(mobile)]
            commands::mobile::get_status_bar_height,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready => {
                GLOBAL_APP_HANDLE
                    .set(app_handle.clone())
                    .expect("Failed to set GLOBAL_APP_HANDLE");
            }
            _ => {}
        });
}
