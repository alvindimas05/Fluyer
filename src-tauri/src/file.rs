use std::path::PathBuf;

use crate::{
    commands::music::STORE_PATH_NAME, music::metadata::MusicMetadata, platform::is_mobile,
    store::GLOBAL_APP_STORE, GLOBAL_APP_HANDLE,
};
use tauri::{path::PathResolver, Manager};
use tauri_plugin_os::hostname;
use walkdir::{DirEntry, WalkDir};

fn is_audio_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        match ext.to_str().unwrap_or("").to_lowercase().as_str() {
            "mp3" | "flac" | "aac" | "m4a" | "wav" | "ogg" => true,
            _ => false,
        }
    } else {
        false
    }
}

pub fn get_all_music() -> Option<Vec<MusicMetadata>> {
    let mut dir: String = if is_mobile() {
        format!(
            "{}/Music",
            GLOBAL_APP_HANDLE
                .get()?
                .path()
                .home_dir()
                .expect("Failed to get home dir on mobile.")
                .into_os_string()
                .into_string()
                .unwrap()
        )
    } else {
        GLOBAL_APP_STORE.get()?.get(STORE_PATH_NAME)?.to_string()
    };

    if !is_mobile() {
        dir.remove(0);
        dir.pop();
    }

    let mut musics: Vec<MusicMetadata> = vec![];
    for entry in WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| {
            if let Err(err) = &e {
                log::warn!("Error reading entry: {}", err);
            }
            e.ok()
        })
        .filter(|e| e.path().is_file() && is_audio_file(e))
    {
        let path_str = entry.path().to_str();
        match path_str {
            Some(path) => {
                let metadata = MusicMetadata::new(path.to_string()).get();
                musics.push(metadata);
            }
            None => {
                log::warn!("Skipping invalid UTF-8 path: {:?}", entry.path());
            }
        }
    }

    Some(musics)
}
