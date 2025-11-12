use crate::file::FolderItem;

#[tauri::command]
pub fn folder_get_items(path: String) -> Vec<FolderItem> {
    crate::file::get_folder_items(path.as_str())
}

#[tauri::command]
pub fn folder_get_first_music_path(path: String, size: Option<String>) -> Option<String> {
    crate::file::get_folder_first_music_path(path.as_str(), size)
}
