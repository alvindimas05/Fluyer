use std::{
    io::{copy, Cursor}, sync::Mutex
};

use base64::Engine;
use image::ImageReader;
use tauri::Manager;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, EventKind};
use std::path::Path;
use std::time::Duration;

use crate::GLOBAL_APP_HANDLE;

#[derive(Clone)]
struct CoverArtRequest {
    name: String,
    status: CoverArtRequestStatus
}
#[derive(Clone, Copy, PartialEq)]
enum CoverArtRequestStatus {
    Loading,
    Failed
}

const COVER_ART_QUEUE: Mutex<Vec<CoverArtRequest>> = Mutex::new(vec![]);

#[tauri::command]
pub async fn cover_art_from_album(album: String) -> Option<String> {
    let file_path = format!("{}/album/{}", cover_art_cache_directory(), album);
    let data = std::fs::read(file_path.clone());
    if data.is_err() && cover_art_get_queue(album.clone()).is_none() {
        return None;
    }
    
    // Note: Assume it's not found for now
    if cover_art_get_queue(album.clone()).unwrap().status == CoverArtRequestStatus::Failed {
        return None;
    }
    wait_until_file_created(file_path);

    let reader = ImageReader::new(std::io::Cursor::new(data.unwrap())).with_guessed_format();
    if reader.is_err() {
        return None;
    }

    let mut buf = Cursor::new(vec![]);
    if reader
        .unwrap()
        .decode()
        .unwrap()
        .write_to(&mut buf, image::ImageFormat::Png)
        .is_err()
    {
        return None;
    }

    Some(base64::engine::general_purpose::STANDARD.encode(buf.get_ref()))
}

#[tauri::command]
pub async fn cover_art_request_album(album: String, url: String) -> Option<String> {
    COVER_ART_QUEUE.lock().unwrap().push(CoverArtRequest {
        name: album.clone(),
        status: CoverArtRequestStatus::Loading
    });
    let res = reqwest::get(url).await;
    if res.is_err() {
        cover_art_set_failed(album);
        return None;
    }

    let bytes = res.unwrap().bytes().await;
    if bytes.is_err() {
        cover_art_set_failed(album);
        return None;
    }
    let cache = cover_art_cache_directory();
    if std::fs::create_dir_all(format!("{}/album", cache.clone())).is_err(){
        cover_art_set_failed(album.clone());
        return None;
    }    
    let mut file =
        std::fs::File::create(format!("{}/album/{}", cache, album)).unwrap();
    let mut content = Cursor::new(bytes.unwrap());
    if copy(&mut content, &mut file).is_err() {
        cover_art_set_failed(album);
        return None;
    }
    if file.sync_all().is_err() {
        cover_art_set_failed(album);
        return None;
    }

    Some(base64::engine::general_purpose::STANDARD.encode(content.into_inner()))
}

fn cover_art_get_queue(album: String) -> Option<CoverArtRequest> {
    let index = COVER_ART_QUEUE.lock().unwrap().iter().position(|e| e.name == album);
    if index.is_none() {
        return None
    }
    Some(COVER_ART_QUEUE.lock().unwrap().get(index.unwrap()).unwrap().clone())
}

fn cover_art_set_failed(album: String){
    let index = COVER_ART_QUEUE.lock().unwrap().iter().position(|e| e.name == album);
    if index.is_none(){
       return
    }
    
    COVER_ART_QUEUE.lock().unwrap().remove(index.unwrap());
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

fn wait_until_file_created(file_path: String) {
    let path = Path::new(file_path.as_str());
    if path.exists() {
        return;
    }

    let parent_dir = path.parent().unwrap();
    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default()
        .with_poll_interval(Duration::from_secs(1)))
        .expect("Failed to create watcher");

    watcher.watch(parent_dir, RecursiveMode::NonRecursive)
        .expect("Failed to watch directory");

    while let Ok(result) = rx.recv() {
        if let Ok(event) = result {
            if matches!(event.kind, EventKind::Create(_)) {
                if event.paths.iter().any(|p| p == path) {
                    break;
                }
            }
        }
    }
}