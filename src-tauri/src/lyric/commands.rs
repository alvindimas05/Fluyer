use crate::lyric::{cache, queue, request, types::*};
use std::fs;

/// Get lyrics for a track - returns synced lyrics text
#[tauri::command]
pub async fn lyric_get(query: LyricQuery) -> Option<String> {
    if query.title.is_empty() {
        crate::warn!("No title provided for lyric search");
        return None;
    }

    let cache_key = request::generate_cache_key(&query);
    let cache_path = format!("{}/{}", cache::get_cache_directory(), cache_key);

    // Check if there's an existing queue entry
    if let Some(queue_item) = queue::get_queue(&cache_key) {
        match queue_item.status {
            LyricRequestStatus::Loaded => {
                // Already fetched, return from cache
                if let Ok(lyrics) = fs::read_to_string(&cache_path) {
                    return Some(lyrics);
                }
                crate::warn!("Lyrics cache file not found: {}", cache_key);
                return None;
            }
            LyricRequestStatus::Failed => {
                // Previous attempt failed
                crate::warn!("Lyrics fetch previously failed: {}", cache_key);
                return None;
            }
            LyricRequestStatus::Pending => {
                // Another request is in progress, wait for it
                let status = queue::wait_for_result(&cache_key).await;
                match status {
                    LyricRequestStatus::Loaded => {
                        if let Ok(lyrics) = fs::read_to_string(&cache_path) {
                            return Some(lyrics);
                        }
                        crate::warn!("Lyrics cache file not found: {}", cache_key);
                        return None;
                    }
                    _ => {
                        crate::warn!("Lyrics fetch failed: {}", cache_key);
                        return None;
                    }
                }
            }
        }
    }

    // No queue entry - we're the first request
    // Try cache first
    if let Ok(lyrics) = fs::read_to_string(&cache_path) {
        queue::set_status(cache_key.clone(), LyricRequestStatus::Loaded);
        return Some(lyrics);
    }

    // Mark as pending before starting the fetch
    queue::set_status(cache_key.clone(), LyricRequestStatus::Pending);

    // Request from API
    let lyrics = request::request_lyrics(query).await;

    if lyrics.is_none() {
        queue::set_status(cache_key.clone(), LyricRequestStatus::Failed);
        crate::warn!("Failed to get lyrics for: {}", cache_key);
        return None;
    }

    queue::set_status(cache_key.clone(), LyricRequestStatus::Loaded);
    lyrics
}
