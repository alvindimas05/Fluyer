use crate::lyric::{cache, queue, request, types::*};
use crate::music::metadata::MusicMetadata;
use std::fs;
use std::path::Path;

/// Get lyrics for a track - returns synced lyrics text
/// Priority: .lrc file → embedded metadata → cached → LrcLib API
#[tauri::command]
pub async fn lyric_get(query: LyricQuery) -> Option<String> {
    if query.title.is_empty() {
        crate::warn!("No title provided for lyric search");
        return None;
    }

    // 1. Try .lrc file first (highest priority)
    let lrc_path = Path::new(&query.path).with_extension("lrc");
    if let Ok(lyrics) = fs::read_to_string(&lrc_path) {
        crate::info!("Loaded lyrics from .lrc file: {:?}", lrc_path);
        return Some(lyrics);
    }

    // 2. Try embedded lyrics from audio file metadata
    if let Some(lyrics) = MusicMetadata::get_embedded_lyrics_from_path(&query.path) {
        crate::info!("Loaded embedded lyrics from: {}", query.path);
        return Some(lyrics);
    }

    // 3. Try cache/API with queue system
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
