use crate::store::init_store;
use music::player::MusicPlayer;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, WebviewWindow, WindowEvent};
#[allow(unused)]
use tauri::{Manager, RunEvent};
#[cfg(target_os = "macos")]
use tauri_plugin_decorum::WebviewWindowExt;

mod api;
mod commands;
mod file;
mod logger;
mod music;
mod platform;
mod store;
mod database;

#[cfg(target_os = "macos")]
const MACOS_TRAFFIC_LIGHTS_INSET_X: f32 = 12.0;
#[cfg(target_os = "macos")]
const MACOS_TRAFFIC_LIGHTS_INSET_Y: f32 = 20.0;

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

pub static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
static GLOBAL_MAIN_WINDOW: OnceLock<WebviewWindow> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(prevent_default())
        .plugin(tauri_plugin_fluyer::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(any(windows, target_os = "linux"))]
            {
                main_window.set_decorations(false).unwrap();
                main_window.set_shadow(false).unwrap();
            }
            #[cfg(target_os = "macos")]
            {
                main_window.make_transparent().unwrap();
                main_window
                    .set_traffic_lights_inset(
                        MACOS_TRAFFIC_LIGHTS_INSET_X,
                        MACOS_TRAFFIC_LIGHTS_INSET_Y,
                    )
                    .unwrap();
            }
            #[cfg(all(desktop, not(windows)))]
            main_window.maximize().unwrap();

            GLOBAL_MAIN_WINDOW
                .set(main_window)
                .expect("Failed to set GLOBAL_APP_WINDOW");
            init_store(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::music::music_controller,
            commands::music::music_position_set,
            commands::music::music_get_all,
            commands::music::music_playlist_add,
            // commands::music::music_get_info,
            commands::music::music_playlist_remove,
            commands::music::music_set_volume,
            commands::music::music_playlist_goto,
            commands::music::music_playlist_moveto,
            commands::music::music_get_buffer,
            #[cfg(desktop)]
            commands::music::music_request_directory,
            #[cfg(desktop)]
            commands::music::music_equalizer,
            commands::music::music_get_image,
            commands::music::music_get_current_duration,
            commands::music::music_request_sync,
            commands::log::log_error,
            #[cfg(target_os = "android")]
            commands::log::toast,
            #[cfg(target_os = "android")]
            commands::mobile::request_read_audio_permission,
            #[cfg(mobile)]
            commands::mobile::get_navigation_bar_height,
            #[cfg(mobile)]
            commands::mobile::get_status_bar_height,
            commands::coverart::cover_art_get,
            #[cfg(windows)]
            commands::decorum::decorum_show_snap_overlay,
            #[cfg(mobile)]
            commands::mobile::set_navigation_bar_visibility,
            #[cfg(target_os = "android")]
            commands::mobile::android_request_directory,
            commands::developer::developer_save_log,
            commands::developer::developer_save_mpv_log,
        ])
        .on_window_event(|_, event| match event {
            WindowEvent::Resized(_) => {
                #[cfg(target_os = "macos")]
                GLOBAL_MAIN_WINDOW
                    .get()
                    .unwrap()
                    .set_traffic_lights_inset(
                        MACOS_TRAFFIC_LIGHTS_INSET_X,
                        MACOS_TRAFFIC_LIGHTS_INSET_Y,
                    )
                    .unwrap();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Ready = event {
                GLOBAL_APP_HANDLE
                    .set(app_handle.clone())
                    .expect("Failed to set GLOBAL_APP_HANDLE");
                app_handle.manage(Mutex::new(AppState::default()));

                database::database::initialize_database();

                debug!(
                    "The app data dir is located at: {}",
                    app_handle.path().app_data_dir().unwrap().display()
                );

                debug!(
                    "The app config dir is located at: {}",
                    app_handle.path().app_config_dir().unwrap().display()
                );

                debug!(
                    "The app cache dir is located at: {}",
                    app_handle.path().app_cache_dir().unwrap().display()
                );
            }
        });
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::debug())
        .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}
