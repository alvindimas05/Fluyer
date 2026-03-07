use crate::playlist::playlist::Playlist;

#[tauri::command]
pub fn playlist_get_all() -> Vec<Playlist> {
    Playlist::get_all()
}

#[tauri::command]
pub fn playlist_create(playlist: Playlist) -> Result<i64, String> {
    Playlist::create(playlist).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn playlist_delete(id: i64) -> Result<(), String> {
    Playlist::delete(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn playlist_upload_image() -> Result<String, String> {
    Playlist::upload_image().await
}

#[tauri::command]
pub async fn playlist_read_image(id: String) -> Result<Vec<u8>, String> {
    Playlist::read_image(id).await
}
