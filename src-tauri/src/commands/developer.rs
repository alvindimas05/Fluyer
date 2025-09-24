use crate::{logger, GLOBAL_APP_HANDLE};
use tauri::Manager;
use crate::utils::toast::{Toast, ToastType};

#[tauri::command]
pub fn developer_save_log() {
    let path = format!(
            "{}/{}",
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .path()
                .home_dir()
                .unwrap()
                .display(),
            logger::get_log_name());
    std::fs::copy(logger::get_log_path(), path.clone()).unwrap();
    Toast::show(format!("Log file saved to {}", path).to_string(), ToastType::Info);
}

#[tauri::command]
pub fn developer_save_mpv_log() {
    let path = format!(
            "{}/{}",
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .path()
                .home_dir()
                .unwrap()
                .display(),
            logger::get_mpv_log_name());
    std::fs::copy(logger::get_mpv_log_path(), path.clone()).unwrap();
    Toast::show(format!("Log MPV file saved to {}", path).to_string(), ToastType::Info);
}
