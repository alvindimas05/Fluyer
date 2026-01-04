use tauri::ipc::Response;

use crate::{logger, music::metadata::MusicMetadata};

/// Get cover art image from a music file
#[tauri::command]
pub async fn music_get_image(path: String, _size: Option<String>) -> Response {
    let image = MusicMetadata::get_image_from_path(path).await;
    if image.is_err() {
        logger::warn!(image.err().unwrap());
        return Response::new(Vec::new());
    }
    Response::new(image.unwrap())
}

/// Get lyrics from a music file
#[tauri::command]
pub fn music_get_lyrics(path: String) -> Option<String> {
    MusicMetadata::get_lyrics_from_path(path)
}
