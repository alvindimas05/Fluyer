use crate::logger;
use crate::state::app_handle;
use crate::utils::toast::{Toast, ToastType};
use tauri::Manager;

/// Log an error message
#[tauri::command]
pub fn log_error(message: String) {
    logger::error!("{}", message);
}

/// Log an info message
#[tauri::command]
pub fn log_info(message: String) {
    logger::info!("{}", message);
}

/// Show a toast message (Android only)
#[cfg(target_os = "android")]
#[tauri::command]
pub fn toast(message: String) {
    Toast::show(message, ToastType::Info);
}

/// Save application log file to home directory
#[tauri::command]
pub fn developer_save_log() {
    let path = format!(
        "{}/{}",
        app_handle().path().home_dir().unwrap().display(),
        logger::get_log_name()
    );
    std::fs::copy(logger::get_log_path(), path.clone()).unwrap();
    Toast::show(
        format!("Log file saved to {}", path).to_string(),
        ToastType::Info,
    );
}

/// Save MPV log file to home directory
#[tauri::command]
pub fn developer_save_mpv_log() {
    let path = format!(
        "{}/{}",
        app_handle().path().home_dir().unwrap().display(),
        logger::get_mpv_log_name()
    );
    std::fs::copy(logger::get_mpv_log_path(), path.clone()).unwrap();
    Toast::show(
        format!("Log MPV file saved to {}", path).to_string(),
        ToastType::Info,
    );
}
