use crate::logger;

#[tauri::command]
pub fn log_error(message: String) {
    logger::error!("{}", message);
}


#[tauri::command]
pub fn log_info(message: String) {
    logger::info!("{}", message);
}

#[cfg(target_os = "android")]
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
