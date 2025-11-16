use rusqlite::params;
use serde::{Deserialize, Serialize};
use crate::database::database::GLOBAL_DATABASE;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub name: String,
    pub music_ids: Vec<(i64, i64)>,
}

impl Playlist {
    pub fn get_all() -> Vec<Playlist> {
        let mut conn_guard = GLOBAL_DATABASE.lock().ok().unwrap();
        let conn = conn_guard.as_mut().unwrap();
        let tx = conn.transaction().unwrap();

        let mut stmt = tx
            .prepare("SELECT id, name FROM playlists")
            .unwrap();

        let playlist_rows = stmt
            .query_map(params![], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        let mut playlists = Vec::new();

        for (playlist_id, name) in playlist_rows {
            let mut music_stmt = tx
                .prepare(
                    "SELECT music_id, position 
                     FROM playlist_musics 
                     WHERE playlist_id = ?1 
                     ORDER BY position ASC"
                )
                .unwrap();

            let music_ids = music_stmt
                .query_map(params![playlist_id], |row| {
                    Ok((
                        row.get::<_, i64>(0)?, // music_id
                        row.get::<_, i64>(1)?  // position
                    ))
                })
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            playlists.push(Playlist { name, music_ids });
        }

        playlists
    }

    pub fn create(playlist: Playlist) -> Result<i64, rusqlite::Error> {
        let mut conn_guard = GLOBAL_DATABASE.lock().unwrap();
        let conn = conn_guard.as_mut().unwrap();
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO playlists (name) VALUES (?1)",
            params![playlist.name],
        )?;

        let playlist_id = tx.last_insert_rowid();

        for (position, (music_id, _)) in playlist.music_ids.iter().enumerate() {
            tx.execute(
                "INSERT INTO playlist_musics (playlist_id, music_id, position) 
                 VALUES (?1, ?2, ?3)",
                params![playlist_id, music_id, position as i64],
            )?;
        }

        tx.commit()?;
        Ok(playlist_id)
    }

    pub fn update(id: i64, playlist: Playlist) -> Result<(), rusqlite::Error> {
        let mut conn_guard = GLOBAL_DATABASE.lock().unwrap();
        let conn = conn_guard.as_mut().unwrap();
        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE playlists SET name = ?1 WHERE id = ?2",
            params![playlist.name, id],
        )?;

        tx.execute(
            "DELETE FROM playlist_musics WHERE playlist_id = ?1",
            params![id],
        )?;

        for (position, (music_id, _)) in playlist.music_ids.iter().enumerate() {
            tx.execute(
                "INSERT INTO playlist_musics (playlist_id, music_id, position) 
                 VALUES (?1, ?2, ?3)",
                params![id, music_id, position as i64],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn delete(id: i64) -> Result<(), rusqlite::Error> {
        let mut conn_guard = GLOBAL_DATABASE.lock().unwrap();
        let conn = conn_guard.as_mut().unwrap();

        conn.execute("DELETE FROM playlists WHERE id = ?1", params![id])?;

        // playlist_musics are deleted automatically due to ON DELETE CASCADE
        Ok(())
    }
}
