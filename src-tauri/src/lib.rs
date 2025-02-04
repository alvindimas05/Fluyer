use crate::store::init_store;
use music::player::MusicPlayer;
#[cfg(desktop)]
use tauri_plugin_decorum::WebviewWindowExt;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, WebviewWindow};
#[allow(unused)]
use tauri::{Manager, RunEvent};

mod commands;
mod file;
mod music;
mod platform;
mod store;
mod logger;

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
static GLOBAL_MAIN_WINDOW: OnceLock<WebviewWindow> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fluyer::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(windows)]
            main_window.set_decorations(false).unwrap();
            #[cfg(target_os = "macos")]
      		main_window.make_transparent().unwrap();
            
            GLOBAL_MAIN_WINDOW.set(main_window).expect("Failed to set GLOBAL_APP_WINDOW");
            init_store(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::music::music_controller,
            commands::music::music_position_set,
            commands::music::music_get_all,
            commands::music::music_playlist_add,
            commands::music::music_get_info,
            commands::music::music_playlist_remove,
            commands::music::music_set_volume,
            #[cfg(desktop)]
            commands::music::music_request_directory,
            commands::log::log_error,
            #[cfg(target_os = "android")]
            commands::log::toast,
            #[cfg(target_os = "android")]
            commands::mobile::request_read_audio_permission,
            #[cfg(mobile)]
            commands::mobile::get_navigation_bar_height,
            #[cfg(mobile)]
            commands::mobile::get_status_bar_height,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Ready = event {
                GLOBAL_APP_HANDLE
                    .set(app_handle.clone())
                    .expect("Failed to set GLOBAL_APP_HANDLE");
                app_handle.manage(Mutex::new(AppState::default()));
                crate::music::player::handle_music_player_background();
                #[cfg(target_os = "android")]
                {
                    crate::music::player::handle_headset_change();
                }
            }
        });
}
