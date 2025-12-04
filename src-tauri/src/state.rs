use crate::music::player::MusicPlayer;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Manager, RunEvent, WebviewWindow};
use crate::logger;

/// Global application state
pub struct AppState {
    pub music_player: MusicPlayer,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            music_player: MusicPlayer::spawn(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global application handle - initialized once at startup
pub static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

/// Global main window reference - initialized once at startup
pub static GLOBAL_MAIN_WINDOW: OnceLock<WebviewWindow> = OnceLock::new();

/// Get the global application handle
pub fn app_handle() -> &'static AppHandle {
    GLOBAL_APP_HANDLE
        .get()
        .expect("GLOBAL_APP_HANDLE not initialized")
}

/// Get the global main window
pub fn main_window() -> &'static WebviewWindow {
    GLOBAL_MAIN_WINDOW
        .get()
        .expect("GLOBAL_MAIN_WINDOW not initialized")
}

/// Initialize global state with app handle
pub fn initialize_globals(app_handle: &AppHandle) {
    GLOBAL_APP_HANDLE
        .set(app_handle.clone())
        .expect("Failed to set GLOBAL_APP_HANDLE");

    app_handle.manage(Mutex::new(AppState::default()));
}

/// Handle application runtime events
pub fn handle_app_events(app_handle: &AppHandle, event: RunEvent) {
    if let RunEvent::Ready = event {
        initialize_globals(app_handle);

        crate::database::database::initialize_database();

        log_directory_paths(app_handle);
    }
}

/// Log application directory paths for debugging
fn log_directory_paths(app_handle: &AppHandle) {
    logger::debug!(
        "The app data dir is located at: {}",
        app_handle.path().app_data_dir().unwrap().display()
    );

    logger::debug!(
        "The app config dir is located at: {}",
        app_handle.path().app_config_dir().unwrap().display()
    );

    logger::debug!(
        "The app cache dir is located at: {}",
        app_handle.path().app_cache_dir().unwrap().display()
    );
}