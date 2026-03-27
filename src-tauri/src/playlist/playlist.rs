use crate::database::database::GLOBAL_DATABASE;
use crate::state::app_handle;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub image: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub paths: Vec<String>,
}

impl Playlist {
    pub fn get_all() -> Vec<Playlist> {
        let mut conn_guard = GLOBAL_DATABASE.lock().ok().unwrap();
        let conn = conn_guard.as_mut().unwrap();
        let tx = conn.transaction().unwrap();

        let mut stmt = tx
            .prepare("SELECT id, name, image, title, artist FROM playlists")
            .unwrap();

        let playlist_rows = stmt
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        let mut playlists = Vec::new();

        for (id, name, image, title, artist) in playlist_rows {
            let mut music_stmt = tx
                .prepare(
                    "SELECT path FROM playlist_musics
                     WHERE playlist_id = ?1
                     ORDER BY position ASC",
                )
                .unwrap();

            let paths = music_stmt
                .query_map(params![id], |row| row.get::<_, String>(0))
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            playlists.push(Playlist {
                id: Some(id),
                name,
                image,
                title,
                artist,
                paths,
            });
        }

        playlists
    }

    pub fn create(playlist: Playlist) -> Result<i64, rusqlite::Error> {
        let mut conn_guard = GLOBAL_DATABASE.lock().unwrap();
        let conn = conn_guard.as_mut().unwrap();
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO playlists (name, image, title, artist) VALUES (?1, ?2, ?3, ?4)",
            params![
                playlist.name,
                playlist.image,
                playlist.title,
                playlist.artist
            ],
        )?;

        let playlist_id = tx.last_insert_rowid();

        for (position, path) in playlist.paths.iter().enumerate() {
            tx.execute(
                "INSERT INTO playlist_musics (playlist_id, path, position)
                 VALUES (?1, ?2, ?3)",
                params![playlist_id, path, position as i64],
            )?;
        }

        tx.commit()?;
        Ok(playlist_id)
    }

    pub fn delete(id: i64) -> Result<(), rusqlite::Error> {
        let mut conn_guard = GLOBAL_DATABASE.lock().unwrap();
        let conn = conn_guard.as_mut().unwrap();

        conn.execute("DELETE FROM playlists WHERE id = ?1", params![id])?;

        // playlist_musics are deleted automatically due to ON DELETE CASCADE
        Ok(())
    }

    pub async fn read_image(id: u8) -> Result<Vec<u8>, String> {
        let path = {
            let conn_guard = GLOBAL_DATABASE.lock().unwrap();
            let conn = conn_guard.as_ref().unwrap();

            let mut stmt = conn
                .prepare("SELECT image FROM playlists WHERE id = ?1")
                .map_err(|e| e.to_string())?;

            let path: Option<String> = stmt
                .query_row(params![id], |row| row.get(0))
                .map_err(|e| e.to_string())?;

            path.ok_or_else(|| "Playlist has no image".to_string())?
        };

        let file = tokio::fs::read(path).await.map_err(|e| e.to_string())?;
        Ok(file)
    }

    pub async fn upload_image() -> Result<String, String> {
        let filename = format!("playlist_{}.png", chrono::Utc::now().timestamp());
        let file = tokio::task::spawn_blocking(|| {
            app_handle()
                .dialog()
                .file()
                .add_filter("Image", &["png", "jpg", "jpeg", "webp"])
                .blocking_pick_file()
        })
        .await
        .unwrap();
        if file.is_none() {
            return Err("No file selected".to_string());
        }

        let app_data_dir = app_handle()
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?;

        let images_dir = app_data_dir.join("playlist_images");
        std::fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;

        std::fs::copy(file.unwrap().as_path().unwrap(), images_dir.join(&filename))
            .map_err(|e| e.to_string())?;

        Ok(images_dir
            .join(&filename)
            .to_str()
            .unwrap_or_default()
            .to_string())
    }
}
