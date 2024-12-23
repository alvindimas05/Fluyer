#[cfg(mobile)]
use crate::models::*;
#[cfg(mobile)]
use crate::FluyerExt;
#[cfg(mobile)]
use crate::Result;
#[cfg(mobile)]
use tauri::{AppHandle, Runtime};

#[cfg(mobile)]
#[tauri::command]
pub async fn toast<R: Runtime>(app: tauri::AppHandle<R>, value: String) -> Result<()> {
    app.fluyer().toast(value).unwrap();
    Ok(())
}

#[cfg(mobile)]
#[tauri::command]
pub async fn get_navigation_bar_size<R: Runtime>(app: AppHandle<R>) -> Result<NavigationBarSize> {
    app.fluyer().get_navigation_bar_size()
}

#[cfg(mobile)]
#[tauri::command]
pub async fn get_status_bar_height<R: Runtime>(app: AppHandle<R>) -> Result<StatusBarHeight> {
    app.fluyer().get_status_bar_height()
}

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.fluyer().check_permissions()
}

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub(crate) async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    permissions: Option<Vec<PermissionType>>,
) -> Result<PermissionStatus> {
    app.fluyer().request_permissions(permissions)
}


// #[cfg(mobile)]
// #[tauri::command]
// pub async fn watch_android_state<R: Runtime, F: Fn(AppState) + Send + Sync + 'static>(app: AppHandle<R>, callback: F) -> Result<()> {
//     app.fluyer().watch_state(callback)
// }