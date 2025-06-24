#[cfg(mobile)]
use crate::GLOBAL_APP_HANDLE;
#[cfg(mobile)]
use tauri_plugin_fluyer::FluyerExt;

#[cfg(target_os = "android")]
#[tauri::command]
pub fn check_read_audio_permission() -> bool {
    use tauri::plugin::PermissionState;

    let app = GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE");
    let permissions = app.fluyer().check_permissions().unwrap();
    permissions.audio == PermissionState::Granted ||
        permissions.storage == PermissionState::Granted
}

#[cfg(target_os = "android")]
#[tauri::command]
pub fn request_read_audio_permission() -> bool {
    use tauri::plugin::PermissionState;
    use tauri_plugin_fluyer::models::PermissionType;

    let app = GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE");
    if !check_read_audio_permission() {
        let permissions = app
            .fluyer()
            .request_permissions(Some(vec![PermissionType::Audio, PermissionType::Storage]))
            .unwrap();
        return permissions.audio == PermissionState::Granted ||
               permissions.storage == PermissionState::Granted;
    }
    true
}

#[cfg(mobile)]
#[tauri::command]
pub fn get_navigation_bar_height() -> u8 {
    let app = GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE");
    app.fluyer().get_navigation_bar_height().unwrap().value
}

#[cfg(mobile)]
#[tauri::command]
pub fn get_status_bar_height() -> u8 {
    let app = GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE");
    app.fluyer().get_status_bar_height().unwrap().value
}
