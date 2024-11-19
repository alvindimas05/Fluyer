#[tauri::command]
pub fn log_error(message: String){
    log::error!("{}", message);
}