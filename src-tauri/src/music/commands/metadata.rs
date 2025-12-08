use crate::music::metadata::MusicMetadata;

/// Get cover art image from a music file
#[tauri::command]
pub async fn music_get_image(path: String, size: Option<String>) -> Option<String> {
    MusicMetadata::get_image_from_path_async(path, size).await
}

/// Get lyrics from a music file
#[tauri::command]
pub fn music_get_lyrics(path: String) -> Option<String> {
    MusicMetadata::get_lyrics_from_path(path)
}
