use tauri::ipc::Response;

use crate::music::metadata::MusicMetadata;

/// Get cover art image from a music file
#[tauri::command]
pub async fn music_get_image(path: String, _size: Option<String>) -> Response {
    #[cfg(not(target_os = "android"))]
    let image = MusicMetadata::get_image_from_path(path).await;

    #[cfg(target_os = "android")]
    let image = MusicMetadata::get_image_from_path_android(path).await;
    if image.is_err() {
        crate::warn!("{}", image.err().unwrap());
        return Response::new(Vec::new());
    }
    Response::new(image.unwrap())
}

#[tauri::command]
pub fn music_get_default_cover_art() -> Response {
    Response::new(MusicMetadata::get_default_cover_art().unwrap())
}

/// Get lyrics from a music file
#[tauri::command]
pub fn music_get_lyrics(path: String) -> Option<String> {
    MusicMetadata::get_lyrics_from_path(path)
}
