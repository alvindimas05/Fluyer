#[cfg(mobile)]
use crate::state::GLOBAL_APP_HANDLE;
#[cfg(mobile)]
use tauri::Emitter;
#[cfg(mobile)]
use tauri_plugin_fluyer::FluyerExt;
#[cfg(mobile)]
use crate::state::app_handle;

#[cfg(target_os = "android")]
#[tauri::command]
pub fn check_read_audio_permission() -> bool {
    use tauri::plugin::PermissionState;

    let permissions_result = app_handle().fluyer().check_permissions().unwrap();
    if permissions_result.is_none() {
        return false;
    }

    let permissions = permissions_result.unwrap();
    permissions.audio == PermissionState::Granted || permissions.storage == PermissionState::Granted
}

#[cfg(target_os = "android")]
#[tauri::command]
pub fn request_read_audio_permission() -> bool {
    use tauri::plugin::PermissionState;
    use tauri_plugin_fluyer::models::PermissionType;

    if !check_read_audio_permission() {
        let permissions = app_handle().fluyer()
            .request_permissions(Some(vec![
                if app.fluyer().get_sdk_version().unwrap().value >= 33 {
                    PermissionType::Audio
                } else {
                    PermissionType::Storage
                },
            ]))
            .unwrap();
        return permissions.audio == PermissionState::Granted
            || permissions.storage == PermissionState::Granted;
    }
    true
}

#[cfg(mobile)]
#[tauri::command]
pub fn get_navigation_bar_height() -> u8 {
    app_handle().fluyer().get_navigation_bar_height().unwrap().value
}

#[cfg(mobile)]
#[tauri::command]
pub fn get_status_bar_height() -> u8 {
    app_handle().fluyer().get_status_bar_height().unwrap().value
}

#[cfg(mobile)]
#[tauri::command]
pub fn set_navigation_bar_visibility(visible: bool) {
    app_handle().fluyer().set_navigation_bar_visibility(visible).unwrap();
}

#[cfg(target_os = "android")]
#[tauri::command]
pub fn android_request_directory() {
    app_handle().fluyer()
        .android_pick_folder(|payload| {
            if let Some(dir) = payload.value {
                app_handle
                    .emit(crate::commands::route::ANDROID_REQUEST_DIRECTORY, dir)
                    .unwrap();
            }
        })
        .ok();
}
