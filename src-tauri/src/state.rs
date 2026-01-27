use crate::music::player::MusicPlayer;
use std::sync::{Arc, Mutex, OnceLock};
use tauri::{App, AppHandle, Manager, RunEvent, WebviewWindow, Wry};
use tauri_plugin_store::{Store, StoreExt};

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

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
static MAIN_WINDOW: OnceLock<WebviewWindow> = OnceLock::new();

static STORE_NAME: &str = "store.json";
static APP_STORE: OnceLock<Arc<Store<Wry>>> = OnceLock::new();

/// Get the global application handle
pub fn app_handle() -> &'static AppHandle {
    APP_HANDLE.get().expect("APP_HANDLE not initialized")
}

pub fn try_app_handle() -> Option<&'static AppHandle> {
    APP_HANDLE.get()
}

/// Get the global main window
pub fn main_window() -> &'static WebviewWindow {
    MAIN_WINDOW.get().expect("MAIN_WINDOW not initialized")
}

pub fn set_main_window(window: WebviewWindow) {
    MAIN_WINDOW.set(window).unwrap();
}

pub fn app_store() -> &'static Arc<Store<Wry>> {
    APP_STORE.get().expect("APP_STORE not initialized")
}

/// Initialize global state with app handle
pub fn initialize_globals(app_handle: &AppHandle) {
    APP_HANDLE
        .set(app_handle.clone())
        .expect("Failed to set APP_HANDLE");

    app_handle.manage(Mutex::new(AppState::default()));
}

pub fn initialize_store(app: &mut App) {
    let store = app.store(STORE_NAME).expect("Failed to initialize store.");

    if APP_STORE.set(store).is_err() {
        crate::error!("Failed to set APP_STORE");
    }
}

pub fn initialize_on_ready(app_handle: &AppHandle) {
    initialize_globals(app_handle);

    crate::database::database::initialize_database();
    crate::music::metadata::MusicMetadata::initialize_ffmpeg_paths();

    log_directory_paths(app_handle);
}

/// Log application directory paths for debugging
fn log_directory_paths(app_handle: &AppHandle) {
    crate::debug!(
        "The app data dir is located at: {}",
        app_handle.path().app_data_dir().unwrap().display()
    );

    crate::debug!(
        "The app config dir is located at: {}",
        app_handle.path().app_config_dir().unwrap().display()
    );

    crate::debug!(
        "The app cache dir is located at: {}",
        app_handle.path().app_cache_dir().unwrap().display()
    );
}
