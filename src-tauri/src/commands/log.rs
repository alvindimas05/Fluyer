#[tauri::command]
pub fn log_error(message: String) {
    log::error!("{}", message);
}

#[cfg(mobile)]
#[tauri::command]
pub fn toast(message: String) {
    use crate::GLOBAL_APP_HANDLE;
    use tauri_plugin_fluyer::FluyerExt;

    GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE")
        .fluyer()
        .toast(message)
        .unwrap();
}
