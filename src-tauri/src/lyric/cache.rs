use crate::state::app_handle;
use tauri::Manager;

/// Get the lyrics cache directory path
pub fn get_cache_directory() -> String {
    let dir = format!(
        "{}/lyrics",
        app_handle().path().app_cache_dir().unwrap().display()
    );
    std::fs::create_dir_all(dir.clone()).expect("Failed to create lyrics cache directory.");
    dir
}
