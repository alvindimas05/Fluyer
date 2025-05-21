use std::path::Path;

#[cfg(target_os = "android")]
use crate::commands::mobile::check_read_audio_permission;
use crate::{
    commands::music::STORE_PATH_NAME,
    logger,
    music::metadata::MusicMetadata,
    platform::{is_android, is_desktop, is_ios},
    store::GLOBAL_APP_STORE,
    GLOBAL_APP_HANDLE,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tauri::Manager;
use walkdir::{DirEntry, WalkDir};

fn is_audio_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "mp3" | "flac" | "aac" | "m4a" | "wav" | "ogg"
        )
    } else {
        false
    }
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with('.'))
        .unwrap_or(false)
}

pub fn get_all_music() -> Option<Vec<MusicMetadata>> {
    #[cfg(target_os = "android")]
    if !check_read_audio_permission() {
        return None;
    }

    let mut search_dirs: Vec<String> = vec![];
    let mut dirs: Vec<Result<DirEntry, walkdir::Error>> = vec![];

    if is_desktop() {
        let mut dir = GLOBAL_APP_STORE.get()?.get(STORE_PATH_NAME)?.to_string();
        dir.remove(0);
        dir.pop();
        search_dirs.push(dir);
    }

    if is_android() {
        search_dirs.extend(get_android_audio_dirs())
    }

    if is_ios() {
        search_dirs.push(get_home_dir())
    }

    for dir in search_dirs {
        dirs.extend(
            WalkDir::new(dir)
                .into_iter()
                .filter_entry(|e| is_not_hidden(e))
                .collect::<Vec<_>>(),
        );
    }

    let musics: Vec<MusicMetadata> = dirs
        .into_par_iter()
        .filter_map(|e| {
            if let Err(err) = &e {
                logger::error!("Error reading entry: {}", err);
                return None;
            }
            e.ok()
        })
        .filter(|e| {
            e.path().is_file()
                && !e
                    .path()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    .contains("au_uu_SzH34yR2")
                && is_audio_file(e)
        })
        .filter_map(|entry| {
            let path_str = entry.path().to_str()?;
            Some(MusicMetadata::new(path_str.to_string()).get())
        })
        .collect();
    Some(musics)
}

fn get_android_audio_dirs() -> Vec<String> {
    let home_dir = get_home_dir();
    let dirs = vec![
        format!("{}/Music", home_dir).to_string(),
        format!("{}/Musics", home_dir).to_string(),
        format!("{}/Download", home_dir).to_string(),
        format!("{}/Downloads", home_dir).to_string(),
    ];

    dirs.into_iter()
        .filter(|dir| Path::new(dir).exists())
        .collect()
}

fn get_home_dir() -> String {
    GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE")
        .path()
        .home_dir()
        .expect("Failed to get home dir on mobile.")
        .to_string_lossy()
        .to_string()
}
