#[tauri::command]
pub fn log_error(message: String) {
    crate::logger::error!("{}", message);
}


#[tauri::command]
pub fn log_info(message: String) {
    crate::logger::info!("{}", message);
}
