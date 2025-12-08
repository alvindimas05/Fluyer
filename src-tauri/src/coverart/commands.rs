use crate::coverart::{cache, queue, request, types::*};
use crate::music::image::ImageHandler;
use image::ImageReader;
use std::io::Cursor;
use std::path::Path;

/// Get cover art for an album or track
#[tauri::command]
pub async fn cover_art_get(query: CoverArtQuery, size: Option<String>) -> CoverArtResponse {
    if query.album.is_none() && query.title.is_none() {
        return CoverArtResponse {
            name: String::from(""),
            status: CoverArtRequestStatus::Failed,
            image: None,
        };
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
        if let Some(image) = get_from_path(file_path.clone(), size.clone()).await {
            queue::set_status(name.clone(), CoverArtRequestStatus::Loaded);

            return CoverArtResponse {
                name,
                status: CoverArtRequestStatus::Loaded,
                image: Some(image),
            };
        }

        let cover_art = request::request_cover_art(query, size).await;

        if cover_art.is_none() {
            queue::set_status(name.clone(), CoverArtRequestStatus::Failed);
            return CoverArtResponse {
                name,
                status: CoverArtRequestStatus::Failed,
                image: None,
            };
        }

        return CoverArtResponse {
            name,
            status: CoverArtRequestStatus::Loaded,
            image: cover_art,
        };
    }

    if queue_item.unwrap().status == CoverArtRequestStatus::Loaded {
        if let Some(image) = get_from_path(file_path.clone(), size).await {
            return CoverArtResponse {
                name,
                status: CoverArtRequestStatus::Loaded,
                image: Some(image),
            };
        }
    }

    CoverArtResponse {
        name,
        status: CoverArtRequestStatus::Failed,
        image: None,
    }
}

async fn get_from_path(file_path: String, size: Option<String>) -> Option<String> {
    if let Ok(data) = std::fs::read(&file_path) {
        let ext = Path::new(&file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        // If it's already PNG or JPEG, don't re-encode
        if ext == "png" || ext == "jpg" || ext == "jpeg" {
            return ImageHandler::resize_image_to_base64(data.as_slice(), size);
        }

        let reader = ImageReader::new(Cursor::new(data)).with_guessed_format();
        if let Ok(reader) = reader {
            let mut buf = Cursor::new(Vec::new());
            if reader
                .decode()
                .unwrap()
                .write_to(&mut buf, image::ImageFormat::Png)
                .is_ok()
            {
                return ImageHandler::resize_image_to_base64(buf.get_ref(), size);
            }
        }
    }

    None
}
