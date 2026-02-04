use crate::coverart::{cache, queue, request, types::*};
use std::fs;

/// Get cover art for an album or track - returns raw image bytes
#[tauri::command]
pub async fn cover_art_get(query: CoverArtQuery) -> Option<Vec<u8>> {
    if query.album.is_none() && query.title.is_none() {
        crate::warn!("No album or title provided");
        return None;
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
                    return Some(image);
                }
                crate::warn!("Cover art file not found: {}", name);
                return None;
            }
            CoverArtRequestStatus::Failed => {
                // Previous attempt failed
                crate::warn!("Cover art fetch previously failed: {}", name);
                return None;
            }
            CoverArtRequestStatus::Pending => {
                // Another request is in progress, wait for it
                let status = queue::wait_for_result(&name).await;
                match status {
                    CoverArtRequestStatus::Loaded => {
                        if let Some(image) = get_image_bytes(&file_path) {
                            return Some(image);
                        }
                        crate::warn!("Cover art file not found: {}", name);
                        return None;
                    }
                    _ => {
                        crate::warn!("Cover art fetch failed: {}", name);
                        return None;
                    }
                }
            }
        }
    }

    // No queue entry - we're the first request
    // Try cache first
    if let Some(image) = get_image_bytes(&file_path) {
        queue::set_status(name.clone(), CoverArtRequestStatus::Loaded);
        return Some(image);
    }

    // Mark as pending before starting the fetch
    queue::set_status(name.clone(), CoverArtRequestStatus::Pending);

    // Request from API
    let cover_art = request::request_cover_art(query).await;

    match cover_art {
        Ok(Some(bytes)) => {
            queue::set_status(name.clone(), CoverArtRequestStatus::Loaded);
            return Some(bytes);
        }
        Ok(None) => {
            queue::set_status(name.clone(), CoverArtRequestStatus::Failed);
            crate::warn!("Failed to get cover art for: {}", name);
            return None;
        }
        Err(e) => {
            queue::set_status(name.clone(), CoverArtRequestStatus::Failed);
            crate::warn!("Failed to get cover art for: {} - {}", name, e);
            return None;
        }
    }
}

fn get_image_bytes(file_path: &str) -> Option<Vec<u8>> {
    fs::read(file_path).ok()
}
