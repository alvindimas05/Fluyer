use crate::folder::types::FolderItem;
use crate::folder::utils::{is_not_hidden, is_supported_audio_file, normalize_path};
use crate::{logger, music::metadata::MusicMetadata};
use chrono::{DateTime, Utc};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rusqlite::Connection;
use walkdir::{DirEntry, WalkDir};

/// Get all subdirectories in a given path
pub fn get_folder_items(path: &str) -> Vec<FolderItem> {
    WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_dir()
                && e.path().to_str().unwrap_or_default() != path
                && is_not_hidden(e)
        })
        .map(|entry| FolderItem {
            path: normalize_path(entry.path().to_str().unwrap_or_default().to_string()),
        })
        .collect()
}

/// Scan directories for audio files
pub fn scan_directories(search_dirs: Vec<String>) -> Vec<String> {
    let mut dirs: Vec<Result<DirEntry, walkdir::Error>> = vec![];

    for dir in search_dirs {
        dirs.extend(
            WalkDir::new(dir)
                .into_iter()
                .filter_entry(|e| is_not_hidden(e))
                .collect::<Vec<_>>(),
        );
    }

    // Get all audio paths
    dirs.into_par_iter()
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
                && is_supported_audio_file(e)
        })
        .filter_map(|entry| {
            let path_str = entry.path().to_str()?;
            Some(normalize_path(path_str.to_string()))
        })
        .collect()
}

/// Get home directory path
pub fn get_home_dir() -> String {
    use crate::state::app_handle;
    use tauri::Manager;

    app_handle()
        .path()
        .home_dir()
        .expect("Failed to get home dir on mobile.")
        .to_string_lossy()
        .to_string()
}

/// Process music files and update database
pub fn process_music_files(conn: &mut Connection, musics: &[String]) {
    let tx = conn.transaction().unwrap();

    for path in musics {
        let modified_at = std::fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok())
            .map(|mtime| DateTime::<Utc>::from(mtime).to_rfc3339())
            .unwrap_or_else(|| "".to_string());

        // Check if exists
        let mut stmt = tx
            .prepare("SELECT modified_at FROM musics WHERE path = ?1")
            .ok()
            .unwrap();
        let result: Result<String, _> = stmt.query_row(rusqlite::params![path], |row| row.get(0));

        match result {
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Doesn't exist, insert
                let metadata = MusicMetadata::new(path.to_string()).get();
                let res = tx.execute(
                    "INSERT INTO musics (
                        path, duration, title, artist, album, album_artist,
                        track_number, genre, date, bits_per_sample, sample_rate, modified_at
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    rusqlite::params![
                        metadata.path,
                        metadata.duration.map(|d| d as i64),
                        metadata.title,
                        metadata.artist,
                        metadata.album,
                        metadata.album_artist,
                        metadata.track_number,
                        metadata.genre,
                        metadata.date,
                        metadata.bits_per_sample.map(|b| b as i64),
                        metadata.sample_rate.map(|s| s as i64),
                        modified_at
                    ],
                );

                if res.is_err() {
                    logger::error!("Insert music to table error: {}", res.unwrap_err());
                }
            }
            Ok(existing_modified_at) => {
                if existing_modified_at != modified_at {
                    // Exists but modified, update
                    let metadata = MusicMetadata::new(path.to_string());
                    let res = tx.execute(
                        "UPDATE musics SET
                                duration = ?1, title = ?2, artist = ?3,
                                album = ?4, album_artist = ?5, track_number = ?6,
                                genre = ?7, bits_per_sample = ?8, sample_rate = ?9,
                                modified_at = ?10, date = ?11
                            WHERE path = ?12",
                        rusqlite::params![
                            metadata.duration.map(|d| d as i64),
                            metadata.title,
                            metadata.artist,
                            metadata.album,
                            metadata.album_artist,
                            metadata.track_number,
                            metadata.genre,
                            metadata.bits_per_sample.map(|b| b as i64),
                            metadata.sample_rate.map(|s| s as i64),
                            modified_at,
                            metadata.date,
                            metadata.path
                        ],
                    );

                    if res.is_err() {
                        logger::error!("Update music to table error: {}", res.unwrap_err());
                    }
                }
            }
            Err(e) => {
                logger::error!("Database error: {:?}", e);
            }
        }
    }

    tx.commit().unwrap();
}
