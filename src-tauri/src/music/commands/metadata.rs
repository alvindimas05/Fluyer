use fast_image_resize::{Resizer, images::Image};
use futures::io::BufWriter;
use tauri::ipc::Response;

use crate::music::metadata::MusicMetadata;

/// Get cover art image from a music file
#[tauri::command]
pub async fn music_get_image(path: String, size: Option<u32>) -> Response {
    #[cfg(not(target_os = "android"))]
    let image = MusicMetadata::get_image_from_path(path).await;

    #[cfg(target_os = "android")]
    let image = MusicMetadata::get_image_from_path_android(path).await;
    if image.is_err() {
        crate::warn!("{}", image.err().unwrap());
        return Response::new(Vec::new());
    }

    if let Some(size) = size {
        let image_bytes = image.unwrap();
        match crate::utils::image::compress_image(&image_bytes, size) {
            Ok(compressed) => return Response::new(compressed),
            Err(e) => {
                crate::warn!("{}", e);
                return Response::new(Vec::new());
            }
        }
    }

    Response::new(image.unwrap())
}

// #[tauri::command]
// pub fn music_get_default_cover_art() -> Response {
//     Response::new(MusicMetadata::get_default_cover_art().unwrap())
// }

/// Get lyrics from a music file
#[tauri::command]
pub fn music_get_lyrics(path: String) -> Option<String> {
    MusicMetadata::get_lyrics_from_path(path)
}
