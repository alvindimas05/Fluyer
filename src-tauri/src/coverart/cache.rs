use crate::state::app_handle;
use tauri::Manager;

/// Get the cover art cache directory path
pub fn get_cache_directory() -> String {
    let dir = format!(
        "{}/coverarts",
        app_handle().path().app_cache_dir().unwrap().display()
    );
    std::fs::create_dir_all(dir.clone()).expect("Failed to create cover arts cache directory.");
    dir
}
