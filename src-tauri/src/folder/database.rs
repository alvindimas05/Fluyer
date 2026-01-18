use crate::{database::database::GLOBAL_DATABASE, music::metadata::MusicMetadata};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

/// Get first music file path from a folder
pub fn get_folder_first_music_path(conn: &mut Connection, path: &str) -> Option<String> {
    let mut stmt = conn
        .prepare("SELECT path FROM musics WHERE path LIKE ? ORDER BY path LIMIT 1")
        .expect("Failed to prepare statement");
    if let Ok(res) = stmt.query_one(params![format!("{}%", path)], |row| Ok(row.get(0))) {
        if let Ok(path) = res {
            return path;
        }
    }
    None
}

/// Retrieve music metadata from database
pub fn get_all_music_from_db() -> Vec<MusicMetadata> {
    let query = "
        SELECT path, duration, title, artist, album, album_artist, track_number,
        genre, bits_per_sample, sample_rate, date, id FROM musics
    "
    .to_string();

    let conn_guard = GLOBAL_DATABASE.lock().ok().unwrap();
    let conn = conn_guard.as_ref().unwrap();
    let mut stmt = conn.prepare(&query).ok().unwrap();

    stmt.query_map(params![], |row| {
        let path: String = row.get(0)?;
        let filename = Path::new(&path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());

        Ok(MusicMetadata {
            id: row.get(11)?,
            path: path.clone(),
            duration: row.get::<_, Option<i64>>(1)?.map(|v| v as u128),
            title: row.get(2)?,
            artist: row.get::<_, Option<String>>(3)?.map(|v| {
                v.replace(
                    &MusicMetadata::artist_separator(),
                    MusicMetadata::separator(),
                )
            }),
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
    .ok()
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

/// Delete paths from database that no longer exist on filesystem
pub fn delete_non_existing_paths(musics: Vec<PathBuf>) {
    let mut conn_guard = GLOBAL_DATABASE.lock().ok().unwrap();
    let conn = conn_guard.as_mut().unwrap();
    let tx = conn.transaction().unwrap();

    // Create temporary table to hold all current paths
    tx.execute("DROP TABLE IF EXISTS temp_existing_paths", [])
        .unwrap();
    tx.execute(
        "CREATE TEMP TABLE temp_existing_paths (path TEXT PRIMARY KEY)",
        [],
    )
    .unwrap();

    // Insert all known existing paths into the temp table
    {
        let mut stmt = tx
            .prepare("INSERT INTO temp_existing_paths (path) VALUES (?)")
            .unwrap();
        for path in &musics {
            stmt.execute(params![path.display().to_string()]).unwrap();
        }
    }

    // Delete all paths in musics table that are not in the temp table
    tx.execute(
        "DELETE FROM musics WHERE path NOT IN (SELECT path FROM temp_existing_paths)",
        [],
    )
    .unwrap();

    tx.commit().unwrap();
}
