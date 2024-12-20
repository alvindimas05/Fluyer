use tauri::{AppHandle, Runtime};

use crate::models::*;
use crate::FluyerExt;
use crate::Result;

#[cfg(mobile)]
#[tauri::command]
pub async fn toast<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    value: String,
) -> std::result::Result<(), String> {
    app.fluyer().toast(value);
    Ok(())
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