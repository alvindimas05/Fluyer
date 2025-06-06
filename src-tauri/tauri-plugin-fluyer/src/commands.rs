use crate::models::*;
use crate::FluyerExt;
use crate::Result;
use tauri::{ipc::Channel, AppHandle, Runtime};

#[tauri::command]
pub async fn toast<R: Runtime>(app: tauri::AppHandle<R>, value: String) -> Result<()> {
    app.fluyer().toast(value)
}

#[tauri::command]
pub async fn get_navigation_bar_size<R: Runtime>(app: AppHandle<R>) -> Result<NavigationBarSize> {
    app.fluyer().get_navigation_bar_size()
}

#[tauri::command]
pub async fn get_status_bar_height<R: Runtime>(app: AppHandle<R>) -> Result<StatusBarHeight> {
    app.fluyer().get_status_bar_height()
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.fluyer().check_permissions()
}

#[tauri::command]
#[specta::specta]
pub(crate) async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    permissions: Option<Vec<PermissionType>>,
) -> Result<PermissionStatus> {
    app.fluyer().request_permissions(permissions)
}

#[tauri::command]
#[specta::specta]
pub async fn watch_playlist_change<R: Runtime>(app: AppHandle<R>, channel: Channel) -> Result<WatchPlaylistChangeResponse> {
    app.fluyer().watch_playlist_change_inner(channel)
}

#[tauri::command]
#[specta::specta]
pub async fn restart_app<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.fluyer().restart_app()
}
