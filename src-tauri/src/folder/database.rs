use crate::music::metadata::MusicMetadata;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct GetMusicFromDbOptions {
    pub path: Option<String>,
}

/// Get first music file path from a folder
pub fn get_folder_first_music_path(
    conn: &mut Connection,
    path: &str,
    _size: Option<String>,
) -> Option<String> {
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
pub fn get_musics_from_db(
    conn: &mut Connection,
    options: GetMusicFromDbOptions,
) -> Vec<MusicMetadata> {
    let mut query = "
        SELECT path, duration, title, artist, album, album_artist, track_number,
        genre, bits_per_sample, sample_rate, date, id FROM musics
    "
    .to_string();

    if options.path.is_some() {
        query.push_str(" WHERE path LIKE ?1");
    }

    let mut stmt = conn.prepare(&query).ok().unwrap();
    let params = if let Some(path) = options.path {
        params![format!("{}%", path)]
    } else {
        params![]
    };

    stmt.query_map(params, |row| {
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

/// Fix Windows paths for older versions (removes double backslashes)
pub fn windows_fix_music_paths_older_version(conn: &mut Connection) {
    let tx = conn.transaction().unwrap();
    tx.execute(
        "UPDATE musics SET path = REPLACE(path, ':\\\\', ':\\')",
        params![],
    )
    .unwrap();
    tx.commit().unwrap();
}

/// Delete paths from database that no longer exist on filesystem
pub fn delete_non_existing_paths(conn: &mut Connection, musics: Vec<String>) {
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
            stmt.execute(params![path]).unwrap();
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
