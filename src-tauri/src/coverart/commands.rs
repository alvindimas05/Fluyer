use crate::coverart::{cache, queue, request, types::*};
use std::fs;

/// Get cover art for an album or track - returns raw image bytes
#[tauri::command]
pub async fn cover_art_get(query: CoverArtQuery) -> Result<Vec<u8>, String> {
    if query.album.is_none() && query.title.is_none() {
        return Err("No album or title provided".to_string());
    }

    let mut name = String::from("");
    let mut folder_name = String::from("");
    if query.album.is_some() {
        name = format!("{} {}", query.artist, query.album.clone().unwrap());
        folder_name = "album".to_string();
    }
    if query.title.is_some() {
        name = format!("{} {}", query.artist, query.title.clone().unwrap());
        folder_name = "music".to_string();
    }

    let file_path = format!("{}/{}/{}", cache::get_cache_directory(), folder_name, name);
    let queue_item = queue::get_queue(name.clone());

    if queue_item.is_none() {
        // Try to get from cache first
        if let Some(image) = get_image_bytes(&file_path) {
            queue::set_status(name.clone(), CoverArtRequestStatus::Loaded);
            return Ok(image);
        }

        // Request from API
        let cover_art = request::request_cover_art(query).await;

        if cover_art.is_none() {
            queue::set_status(name.clone(), CoverArtRequestStatus::Failed);
            return Err(format!("Failed to get cover art for: {}", name));
        }

        // Read the saved file
        if let Some(image) = get_image_bytes(&file_path) {
            return Ok(image);
        }

        return Err(format!("Cover art file not found: {}", name));
    }

    if queue_item.unwrap().status == CoverArtRequestStatus::Loaded {
        if let Some(image) = get_image_bytes(&file_path) {
            return Ok(image);
        }
    }

    Err(format!("Cover art not available: {}", name))
}

fn get_image_bytes(file_path: &str) -> Option<Vec<u8>> {
    fs::read(file_path).ok()
}
