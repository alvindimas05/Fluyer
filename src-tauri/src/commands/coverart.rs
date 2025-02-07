use std::{
    io::{copy, Cursor},
    sync::Mutex,
};

use base64::Engine;
use image::ImageReader;
use lazy_static::lazy_static;
// use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::path::Path;
use tauri::Manager;

use crate::{api::musicbrainz::MusicBrainz, GLOBAL_APP_HANDLE};

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
    Loading,
    Failed,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverArtResponse {
    name: String,
    status: CoverArtRequestStatus,
    image: Option<String>,
}

lazy_static! {
    static ref COVER_ART_QUEUE: Mutex<Vec<CoverArtRequest>> = Mutex::new(vec![]);
}

#[tauri::command]
pub fn cover_art_from_album(album: String) -> CoverArtResponse {
    let file_path = format!("{}/album/{}", cover_art_cache_directory(), album);
    let queue = cover_art_get_queue(album.clone());
    if queue.is_none() {
        COVER_ART_QUEUE.lock().unwrap().push(CoverArtRequest {
            name: album.clone(),
            status: CoverArtRequestStatus::Loading,
        });
        if Path::new(file_path.as_str()).exists() {
            // Check if image is corrupted
            let data = std::fs::read(file_path.clone());
            if data.is_ok() {
                let reader =
                    ImageReader::new(std::io::Cursor::new(data.unwrap())).with_guessed_format();
                if reader.is_ok() {
                    let mut buf = Cursor::new(vec![]);
                    if reader
                        .unwrap()
                        .decode()
                        .unwrap()
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .is_ok()
                    {
                        return CoverArtResponse {
                            name: album,
                            status: CoverArtRequestStatus::Loaded,
                            image: Some(
                                base64::engine::general_purpose::STANDARD.encode(buf.get_ref()),
                            ),
                        };
                    }
                }
            }
        }
        
        let cover_art = cover_art_request_album(album.clone());
        if cover_art.is_none() {
            cover_art_set_status(album.clone(), CoverArtRequestStatus::Failed);
            return CoverArtResponse {
                name: album,
                status: CoverArtRequestStatus::Failed,
                image: None,
            };
        }

        return CoverArtResponse {
            name: album,
            status: CoverArtRequestStatus::Loaded,
            image: cover_art,
        };
    }

    // if queue.unwrap().status == CoverArtRequestStatus::Loading {
    //     let status = wait_until_file_created(album.clone(), file_path.clone());

    //     return CoverArtResponse {
    //         name: album,
    //         status,
    //         image: None,
    //     };
    // }

    CoverArtResponse {
        name: album.clone(),
        status: CoverArtRequestStatus::Failed,
        image: None,
    }
}

fn cover_art_request_album(album: String) -> Option<String> {    
    let url = MusicBrainz::get_cover_art_from_album(album.clone());

    if url.is_none() {
        return None;
    }

    let res = reqwest::blocking::get(url.unwrap());
    if res.is_err() {
        return None;
    }

    let bytes = res.unwrap().bytes();
    if bytes.is_err() {
        return None;
    }
    let cache = cover_art_cache_directory();
    if std::fs::create_dir_all(format!("{}/album", cache.clone())).is_err() {
        return None;
    }
    let mut file = std::fs::File::create(format!("{}/album/{}", cache, album)).unwrap();
    let mut content = Cursor::new(bytes.unwrap());
    if copy(&mut content, &mut file).is_err() {
        return None;
    }
    if file.sync_all().is_err() {
        return None;
    }

    Some(base64::engine::general_purpose::STANDARD.encode(content.into_inner()))
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

fn cover_art_set_status(album: String, status: CoverArtRequestStatus) {
    cover_art_remove(album.clone());
    COVER_ART_QUEUE.lock().unwrap().push(CoverArtRequest {
        name: album,
        status,
    });
}

fn cover_art_remove(album: String){
    COVER_ART_QUEUE.lock().unwrap().retain(|c| c.name == album);
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

//     logger::debug!("Waiting");
//     #[allow(for_loops_over_fallibles)]
//     for res in rx {
//         logger::debug!("Looping");
        
//         let queue = cover_art_get_queue(album.clone());
//         if queue.is_some() && queue.unwrap().status == CoverArtRequestStatus::Failed {
//             logger::debug!("Returning");
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
