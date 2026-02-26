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
pub fn playlist_save_image(image_data: Vec<u8>, filename: String) -> Result<String, String> {
    Playlist::save_image(image_data, filename)
}
