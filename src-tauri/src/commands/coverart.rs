use std::{
    io::{copy, Cursor},
    sync::Mutex,
};

use image::ImageReader;
use lazy_static::lazy_static;
// use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::Manager;

use crate::{api::musicbrainz::MusicBrainz};
use crate::music::image::ImageHandler;
use crate::state::GLOBAL_APP_HANDLE;

#[derive(Clone, Debug)]
struct CoverArtRequest {
    name: String,
    #[allow(dead_code)]
    status: CoverArtRequestStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
enum CoverArtRequestStatus {
    Loaded,
    // Loading,
    Failed,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverArtResponse {
    name: String,
    status: CoverArtRequestStatus,
    image: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct CoverArtQuery {
    pub artist: String,
    pub album: Option<String>,
    pub title: Option<String>,
}

lazy_static! {
    static ref COVER_ART_QUEUE: Mutex<Vec<CoverArtRequest>> = Mutex::new(vec![]);
}

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

    let file_path = format!("{}/{}/{}", cover_art_cache_directory(), folder_name, name);
    let queue = cover_art_get_queue(name.clone());
    if queue.is_none() {
        if let Some(image) = cover_art_get_from_path(file_path.clone(), size.clone()).await {
            cover_art_set_status(name.clone(), CoverArtRequestStatus::Loaded);

            return CoverArtResponse {
                name,
                status: CoverArtRequestStatus::Loaded,
                image: Some(image),
            };
        }

        let cover_art = cover_art_request(query, size).await;

        if cover_art.is_none() {
            cover_art_set_status(name.clone(), CoverArtRequestStatus::Failed);
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

    if queue.unwrap().status == CoverArtRequestStatus::Loaded {
        if let Some(image) = cover_art_get_from_path(file_path.clone(), size).await {
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

async fn cover_art_get_from_path(file_path: String, size: Option<String>) -> Option<String> {
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

async fn cover_art_request(query: CoverArtQuery, size: Option<String>) -> Option<String> {
    let url = MusicBrainz::get_cover_art(query.clone()).await;

    if url.is_none() {
        return None;
    }

    let res = reqwest::get(url.unwrap()).await;
    if res.is_err() {
        return None;
    }

    let bytes = res.unwrap().bytes().await;
    if bytes.is_err() {
        return None;
    }
    let cache = cover_art_cache_directory();

    let mut folder_name = String::from("");
    let mut file_path = String::from("");
    if query.album.is_some() {
        folder_name = "album".to_string();
        file_path = format!(
            "{}/{}/{} {}",
            cache,
            folder_name,
            query.artist,
            query.album.unwrap()
        );
    }
    if query.title.is_some() {
        folder_name = "music".to_string();
        file_path = format!(
            "{}/{}/{} {}",
            cache,
            folder_name,
            query.artist,
            query.title.unwrap()
        );
    }

    if std::fs::create_dir_all(format!("{}/{}", cache.clone(), folder_name)).is_err() {
        return None;
    }

    let mut file = std::fs::File::create(file_path).unwrap();
    let mut content = Cursor::new(bytes.unwrap());
    if copy(&mut content, &mut file).is_err() {
        return None;
    }
    if file.sync_all().is_err() {
        return None;
    }

    ImageHandler::resize_image_to_base64(content.get_ref(), size)
}

fn cover_art_get_queue(album: String) -> Option<CoverArtRequest> {
    let index = COVER_ART_QUEUE
        .lock()
        .unwrap()
        .iter()
        .position(|e| e.name == album);
    if index.is_none() {
        return None;
    }
    Some(
        COVER_ART_QUEUE
            .lock()
            .unwrap()
            .get(index.unwrap())
            .unwrap()
            .clone(),
    )
}

fn cover_art_set_status(name: String, status: CoverArtRequestStatus) {
    cover_art_remove(name.clone());
    COVER_ART_QUEUE
        .lock()
        .unwrap()
        .push(CoverArtRequest { name, status });
}

fn cover_art_remove(name: String) {
    COVER_ART_QUEUE.lock().unwrap().retain(|c| c.name == name);
}

fn cover_art_cache_directory() -> String {
    let dir = format!(
        "{}/coverarts",
        GLOBAL_APP_HANDLE
            .get()
            .unwrap()
            .path()
            .app_cache_dir()
            .unwrap()
            .display()
    );
    std::fs::create_dir_all(dir.clone()).expect("Failed to create cover arts cache directory.");
    dir
}

// fn wait_until_file_created(album: String, file_path: String) -> CoverArtRequestStatus {
//     let path = Path::new(file_path.as_str());
//     if path.exists() {
//         return CoverArtRequestStatus::Loaded
//     }

//     let parent_dir = path.parent().unwrap();
//     let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();

//     let mut watcher: RecommendedWatcher = Watcher::new(
//         tx,
//         notify::Config::default().with_poll_interval(Duration::from_secs(1)),
//     )
//     .expect("Failed to create watcher");

//     watcher
//         .watch(parent_dir, RecursiveMode::NonRecursive)
//         .expect("Failed to watch directory");

//     #[allow(for_loops_over_fallibles)]
//     for res in rx {

//         let queue = cover_art_get_queue(album.clone());
//         if queue.is_some() && queue.unwrap().status == CoverArtRequestStatus::Failed {
//             return CoverArtRequestStatus::Failed
//         }

//         if let Ok(event) = res {
//             if matches!(event.kind, EventKind::Create(_)) {
//                 if event.paths.iter().any(|p| p == path) {
//                     return CoverArtRequestStatus::Loaded
//                 }
//             }
//         }
//     }
//     CoverArtRequestStatus::Failed
// }
