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

    // Check if there's an existing queue entry
    if let Some(queue_item) = queue::get_queue(&name) {
        match queue_item.status {
            CoverArtRequestStatus::Loaded => {
                // Already fetched, return from cache
                if let Some(image) = get_image_bytes(&file_path) {
                    return Ok(image);
                }
                return Err(format!("Cover art file not found: {}", name));
            }
            CoverArtRequestStatus::Failed => {
                // Previous attempt failed
                return Err(format!("Cover art fetch previously failed: {}", name));
            }
            CoverArtRequestStatus::Pending => {
                // Another request is in progress, wait for it
                let status = queue::wait_for_result(&name).await;
                match status {
                    CoverArtRequestStatus::Loaded => {
                        if let Some(image) = get_image_bytes(&file_path) {
                            return Ok(image);
                        }
                        return Err(format!("Cover art file not found: {}", name));
                    }
                    _ => {
                        return Err(format!("Cover art fetch failed: {}", name));
                    }
                }
            }
        }
    }

    // No queue entry - we're the first request
    // Try cache first
    if let Some(image) = get_image_bytes(&file_path) {
        queue::set_status(name.clone(), CoverArtRequestStatus::Loaded);
        return Ok(image);
    }

    // Mark as pending before starting the fetch
    queue::set_status(name.clone(), CoverArtRequestStatus::Pending);

    // Request from API
    let cover_art = request::request_cover_art(query).await;

    if cover_art.is_none() {
        queue::set_status(name.clone(), CoverArtRequestStatus::Failed);
        return Err(format!("Failed to get cover art for: {}", name));
    }

    // Read the saved file
    if let Some(image) = get_image_bytes(&file_path) {
        queue::set_status(name.clone(), CoverArtRequestStatus::Loaded);
        return Ok(image);
    }

    queue::set_status(name.clone(), CoverArtRequestStatus::Failed);
    Err(format!("Cover art file not found: {}", name))
}

fn get_image_bytes(file_path: &str) -> Option<Vec<u8>> {
    fs::read(file_path).ok()
}
