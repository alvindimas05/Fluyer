use chrono::{DateTime, Utc};
#[cfg(target_os = "android")]
use crate::commands::mobile::check_read_audio_permission;
use crate::{
    commands::music::MUSIC_STORE_PATH_NAME,
    logger,
    music::metadata::MusicMetadata,
    platform::{is_desktop, is_ios},
    store::GLOBAL_APP_STORE,
    GLOBAL_APP_HANDLE,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rusqlite::{params, params_from_iter, Connection};
use tauri::Manager;
use walkdir::{DirEntry, WalkDir};
use crate::database::database::GLOBAL_DATABASE;

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

    // Get store music paths
    let dir = GLOBAL_APP_STORE
        .get()?
        .get(MUSIC_STORE_PATH_NAME)?
        .to_string();
    let dir_paths = dir.split("||");

    for d in dir_paths {
        let trimmed = d.trim().trim_matches('"'); // optionally remove whitespace and quotes
        if !trimmed.is_empty() {
            search_dirs.push(trimmed.to_string());
        }
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

    // Get all audio paths
    let musics: Vec<String> = dirs
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
            Some(path_str.to_string())
        })
        .collect();

    let mut conn_guard = GLOBAL_DATABASE.lock().ok()?;
    let conn = conn_guard.as_mut()?;

    let tx = conn.transaction().unwrap();

    for path in &musics {
        let modified_at = std::fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok())
            .map(|mtime| DateTime::<Utc>::from(mtime).to_rfc3339())
            .unwrap_or_else(|| "".to_string());

        // Check if exists
        let mut stmt = tx.prepare("SELECT modified_at FROM musics WHERE path = ?1").ok()?;
        let result: Result<String, _> = stmt.query_row(params![path], |row| row.get(0));

        match result {
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Doesn't exist, insert
                let metadata = MusicMetadata::new(path.to_string()).get();
                let res = tx.execute(
                    "INSERT INTO musics (
                        path, duration, title, artist, album, album_artist,
                        track_number, genre, date, bits_per_sample, sample_rate, modified_at
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    params![
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
                        params![
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

    delete_non_existing_paths(conn, musics);

    // Retrieve and return the final state from DB
    let mut stmt = conn.prepare("SELECT path, duration, title, artist, album, album_artist, track_number, genre, bits_per_sample, sample_rate, date FROM musics").ok()?;
    let rows: Vec<MusicMetadata> = stmt
        .query_map([], |row| {
            let path: String = row.get(0)?;
            let filename = path
                .split('/')
                .last()
                .map(|s| s.to_string());

            Ok(MusicMetadata {
                path: path.clone(),
                duration: row.get::<_, Option<i64>>(1)?.map(|v| v as u128),
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                album_artist: row.get(5)?,
                track_number: row.get(6)?,
                genre: row.get(7)?,
                bits_per_sample: row.get::<_, Option<i64>>(8)?.map(|v| v as u32),
                sample_rate: row.get::<_, Option<i64>>(9)?.map(|v| v as u32),
                date: row.get(10)?,

                filename,
                image: None,
                extra_tags: None,
            })
        })
        .ok()?
        .filter_map(|r| r.ok())
        .collect();


    Some(rows)
}

fn delete_non_existing_paths(conn: &mut Connection, musics: Vec<String>) {
    let tx = conn.transaction().unwrap();

    // Create temporary table to hold all current paths
    tx.execute("DROP TABLE IF EXISTS temp_existing_paths", []).unwrap();
    tx.execute(
        "CREATE TEMP TABLE temp_existing_paths (path TEXT PRIMARY KEY)",
        [],
    ).unwrap();

    // Insert all known existing paths into the temp table
    {
        let mut stmt = tx.prepare("INSERT INTO temp_existing_paths (path) VALUES (?)").unwrap();
        for path in &musics {
            stmt.execute(params![path]).unwrap();
        }
    }

    // Delete all paths in musics table that are not in the temp table
    tx.execute(
        "DELETE FROM musics WHERE path NOT IN (SELECT path FROM temp_existing_paths)",
        [],
    ).unwrap();

    tx.commit().unwrap();
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
