#[cfg(target_os = "android")]
use crate::commands::mobile::check_read_audio_permission;
use crate::{
    commands::music::STORE_PATH_NAME, logger, music::metadata::MusicMetadata, platform::{is_android, is_desktop, is_ios}, store::GLOBAL_APP_STORE, GLOBAL_APP_HANDLE
};
use tauri::{Emitter, Manager};
use walkdir::{DirEntry, WalkDir};

fn is_audio_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), "mp3" | "flac" | "aac" | "m4a" | "wav" | "ogg")
    } else {
        false
    }
}

pub fn get_all_music() -> Option<Vec<MusicMetadata>> {
    #[cfg(target_os = "android")]
    {
        if !check_read_audio_permission() {
            return None;
        }
    }
    
    let mut dirs: Vec<Result<DirEntry, walkdir::Error>> = vec![];

    if is_desktop() {
        let mut dir = GLOBAL_APP_STORE.get()?.get(STORE_PATH_NAME)?.to_string();
        dir.remove(0);
        dir.pop();
        dirs = WalkDir::new(dir).into_iter().collect();
    }
    
    if is_android() {
        let android_dirs = get_android_audio_dirs();
        for dir in android_dirs {
            dirs.extend(WalkDir::new(dir).into_iter().collect::<Vec<Result<DirEntry, walkdir::Error>>>());
        }
    }
    
    if is_ios() {
        dirs = WalkDir::new(get_home_dir()).into_iter().collect();
    }
    
    let mut musics: Vec<MusicMetadata> = vec![];
    for entry in dirs.into_iter()
        .filter_map(|e| {
            if let Err(err) = &e {
                logger::error!("Error reading entry: {}", err);
            }
            e.ok()
        })
        .filter(|e| {
            e.path().is_file()
            // Common unknown music file on Androids
                && !e.path()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    .contains("au_uu_SzH34yR2")
                && is_audio_file(e)
        })
    {
        let path_str = entry.path().to_str();
        match path_str {
            Some(path) => {
                let metadata = MusicMetadata::new(path.to_string()).get();
                musics.push(metadata);
            }
            None => {
                logger::error!("Skipping invalid UTF-8 path: {:?}", entry.path());
            }
        }
    }

    Some(musics)
}

fn get_android_audio_dirs() -> Vec<String> {
    let home_dir = get_home_dir();
    vec![
        format!("{}/Music", home_dir).to_string(),
        format!("{}/Downloads", home_dir).to_string(),
        format!("{}/Documents", home_dir).to_string(),
    ]
}

fn get_home_dir() -> String {
    GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE")
        .path()
        .home_dir()
        .expect("Failed to get home dir on mobile.")
        .into_os_string()
        .into_string().unwrap()
}