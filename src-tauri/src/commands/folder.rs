use crate::file::FolderItem;

#[tauri::command]
pub fn folder_get_items(path: String) -> Vec<FolderItem> {
    crate::file::get_folder_items(path.as_str())
}

#[tauri::command]
pub fn folder_get_image(path: String) -> Option<String> {
    crate::file::get_folder_image(path.as_str())
}