use std::path::PathBuf;

use crate::database::database::GLOBAL_DATABASE;
use crate::folder::types::FolderItem;
use crate::folder::utils::is_not_hidden;
use crate::music::metadata::MusicMetadata;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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
            path: entry.path().to_str().unwrap_or_default().to_string(),
        })
        .collect()
}

/// Scan directories for audio files
pub fn scan_directories(search_dirs: Vec<String>) -> Vec<PathBuf> {
    let mut dirs: Vec<Result<DirEntry, walkdir::Error>> = vec![];

    for dir in search_dirs {
        dirs.extend(
            WalkDir::new(dir)
                .into_iter()
                .filter_entry(|e| is_not_hidden(e))
                .collect::<Vec<_>>(),
        );
    }

    dirs.into_par_iter()
        .filter_map(|e| {
            if let Err(err) = &e {
                crate::error!("Error reading entry: {}", err);
                return None;
            }
            e.ok()
        })
        .filter(|e| {
            e.path().is_file() && e.path().file_name().unwrap_or_default() != "au_uu_SzH34yR2.mp3"
        })
        .map(|entry| entry.path().to_path_buf())
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
pub async fn process_supported_files(paths: &[PathBuf]) {
    // Optimization: Pre-fetch existing records to skip unmodified files
    let existing_records = tokio::task::spawn_blocking(|| {
        let conn_guard = GLOBAL_DATABASE.lock().ok()?;
        let conn = conn_guard.as_ref()?;
        let mut stmt = conn.prepare("SELECT path, modified_at FROM musics").ok()?;

        let mut map = std::collections::HashMap::new();
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))
            })
            .ok()?;

        for row in rows {
            if let Ok((path, modified)) = row {
                map.insert(path, modified.unwrap_or_default());
            }
        }
        Some(map)
    })
    .await
    .ok()
    .flatten()
    .unwrap_or_default();

    let existing_records = std::sync::Arc::new(existing_records);

    let metadata_results: Vec<_> = futures::stream::iter(paths.to_vec())
        .map(|path| {
            let existing_records = existing_records.clone();
            async move {
                let path_str = path.display().to_string();
                let modified = get_modified_time(&path);

                // Check if file is unmodified in DB
                if let Some(db_modified) = existing_records.get(&path_str) {
                    if let Some(curr_modified) = &modified {
                        if db_modified == curr_modified {
                            return None;
                        }
                    }
                }

                #[cfg(target_os = "android")]
                let metadata = MusicMetadata::get_android(path_str.clone()).await;

                #[cfg(not(target_os = "android"))]
                let metadata = MusicMetadata::get(path_str.clone()).await;

                Some((path_str, modified, metadata))
            }
        })
        .buffer_unordered(10)
        .filter_map(|res| async { res })
        .collect()
        .await;

    if metadata_results.is_empty() {
        crate::info!("No new or modified files to process.");
        return;
    }
    crate::info!("Processed metadata for {} files.", metadata_results.len());

    // Then do one blocking DB transaction
    tokio::task::spawn_blocking(move || {
        let mut conn_guard = GLOBAL_DATABASE.lock().ok()?;
        let conn = conn_guard.as_mut()?;
        let tx = conn.transaction().ok()?;

        for (path, modified_at, metadata) in metadata_results {
            if metadata.is_err() {
                crate::warn!(
                    "Failed to read metadata for file {}: {:?}",
                    path,
                    metadata.unwrap_err()
                );
                continue;
            }
            let metadata = metadata.unwrap();
            let path_string = path.to_string();
            let modified_at = modified_at.unwrap_or_default();

            // Check if exists
            let mut stmt = tx
                .prepare("SELECT modified_at FROM musics WHERE path = ?1")
                .ok()
                .unwrap();
            let result: Result<String, _> =
                stmt.query_row(rusqlite::params![path_string], |row| row.get(0));

            match result {
                Err(rusqlite::Error::QueryReturnedNoRows) => {
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
                        crate::error!("Insert music to table error: {}", res.unwrap_err());
                    }
                }
                Ok(existing_modified_at) => {
                    if existing_modified_at != modified_at {
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
                            crate::error!("Update music to table error: {}", res.unwrap_err());
                        }
                    }
                }
                Err(e) => {
                    crate::error!("Database error: {:?}", e);
                }
            }
        }

        tx.commit().ok()
    })
    .await
    .ok();
}

fn get_modified_time(path: &PathBuf) -> Option<String> {
    std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(|mtime| DateTime::<Utc>::from(mtime).to_rfc3339())
}
