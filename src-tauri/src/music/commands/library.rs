use crate::music::metadata::MusicMetadata;

/// Get all music files from the library
#[tauri::command]
pub fn music_get_all() -> Option<Vec<MusicMetadata>> {
    crate::file::get_all_music()
}
