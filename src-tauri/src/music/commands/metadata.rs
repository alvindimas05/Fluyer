use tauri::ipc::Response;

use crate::music::image_cache::{CacheStatus, ImageCache};
use crate::music::metadata::MusicMetadata;

#[tauri::command]
pub async fn music_image_get(path: String, size: Option<u32>) -> Response {
    let cache_key = {
        let meta = MusicMetadata::get(path.clone()).await.ok();
        let artist = meta.as_ref().and_then(|m| m.artist.as_deref());
        let album = meta.as_ref().and_then(|m| m.album.as_deref());
        ImageCache::get_cache_key(artist, album, &path)
    };

    if size.is_none() {
        #[cfg(not(target_os = "android"))]
        let raw = MusicMetadata::get_image_from_path(path).await;
        #[cfg(target_os = "android")]
        let raw = MusicMetadata::get_image_from_path_android(path).await;
        return Response::new(raw.unwrap_or(Vec::new()));
    }

    let base_size = ImageCache::base_cover_size();

    match ImageCache::queue_get(&cache_key) {
        Some(CacheStatus::Pending) => {
            let status = ImageCache::queue_wait(&cache_key).await;
            if status == CacheStatus::Loaded {
                return serve_from_cache(&cache_key, size, base_size).await;
            } else {
                return Response::new(Vec::new());
            }
        }
        Some(CacheStatus::Loaded) => {
            return serve_from_cache(&cache_key, size, base_size).await;
        }
        Some(CacheStatus::Failed) => {
            return Response::new(Vec::new());
        }
        None => {}
    }

    if let Some(cached) = ImageCache::read_cache(&cache_key) {
        ImageCache::queue_set(&cache_key, CacheStatus::Loaded);
        return serve_bytes(cached, size, base_size).await;
    }

    ImageCache::queue_set(&cache_key, CacheStatus::Pending);

    #[cfg(not(target_os = "android"))]
    let raw = MusicMetadata::get_image_from_path(path).await;
    #[cfg(target_os = "android")]
    let raw = MusicMetadata::get_image_from_path_android(path).await;

    let raw_bytes = match raw {
        Ok(b) => b,
        Err(e) => {
            crate::warn!("{}", e);
            ImageCache::queue_set(&cache_key, CacheStatus::Failed);
            return Response::new(Vec::new());
        }
    };

    let sem = ImageCache::resize_semaphore();
    let cached_bytes = {
        let _permit = sem.acquire().await.expect("Semaphore closed");
        match crate::utils::image::compress_image(&raw_bytes, base_size) {
            Ok(resized) => {
                ImageCache::write_cache(&cache_key, &resized);
                ImageCache::queue_set(&cache_key, CacheStatus::Loaded);
                resized
            }
            Err(e) => {
                crate::warn!("Failed to resize to base size: {}", e);
                ImageCache::write_cache(&cache_key, &raw_bytes);
                ImageCache::queue_set(&cache_key, CacheStatus::Loaded);
                raw_bytes
            }
        }
    };

    if let Some(req_size) = size {
        if req_size < base_size {
            let _permit = sem.acquire().await.expect("Semaphore closed");
            match crate::utils::image::compress_image(&cached_bytes, req_size) {
                Ok(small) => return Response::new(small),
                Err(e) => crate::warn!("Failed to downscale to {}: {}", req_size, e),
            }
        }
    }

    Response::new(cached_bytes)
}

async fn serve_from_cache(key: &str, size: Option<u32>, base_size: u32) -> Response {
    match ImageCache::read_cache(key) {
        Some(bytes) => serve_bytes(bytes, size, base_size).await,
        None => {
            crate::warn!("Cache entry missing for key: {}", key);
            Response::new(Vec::new())
        }
    }
}

async fn serve_bytes(bytes: Vec<u8>, size: Option<u32>, base_size: u32) -> Response {
    if let Some(req_size) = size {
        if req_size < base_size {
            let sem = ImageCache::resize_semaphore();
            let _permit = sem.acquire().await.expect("Semaphore closed");
            match crate::utils::image::compress_image(&bytes, req_size) {
                Ok(small) => return Response::new(small),
                Err(e) => crate::warn!("Failed to downscale to {}: {}", req_size, e),
            }
        }
    }
    Response::new(bytes)
}

#[tauri::command]
pub fn music_lyrics_get(path: String) -> Option<String> {
    MusicMetadata::get_lyrics_from_path(path)
}
