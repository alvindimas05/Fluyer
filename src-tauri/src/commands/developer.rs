use tauri::Manager;
use crate::{logger, GLOBAL_APP_HANDLE};

#[tauri::command]
pub fn developer_save_log() {
    std::fs::copy(logger::get_log_path(),
        format!("{}/{}", GLOBAL_APP_HANDLE.get().unwrap()
          .path().download_dir().unwrap().display(), logger::get_log_name())).unwrap();
}

#[tauri::command]
pub fn developer_save_mpv_log() {
    std::fs::copy(logger::get_mpv_log_path(),
        format!("{}/{}", GLOBAL_APP_HANDLE.get().unwrap()
          .path().download_dir().unwrap().display(), logger::get_mpv_log_name())).unwrap();
}