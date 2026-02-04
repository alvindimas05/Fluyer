use crate::api::musicbrainz::MusicBrainz;
use crate::coverart::{cache, types::CoverArtQuery};
use std::io::copy;
use std::io::Cursor;

/// Request cover art from external API and cache it
pub async fn request_cover_art(query: CoverArtQuery) -> Result<Option<Vec<u8>>, String> {
    let url = MusicBrainz::get_cover_art(query.clone()).await?;

    if url.is_none() {
        return Ok(None);
    }

    let res = reqwest::get(url.unwrap())
        .await
        .map_err(|e| e.to_string())?;

    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    let cache_dir = cache::get_cache_directory();

    let mut folder_name = String::from("");
    let mut file_path = String::from("");
    if query.album.is_some() {
        folder_name = "album".to_string();
        file_path = format!(
            "{}/{}/{} {}",
            cache_dir,
            folder_name,
            query.artist,
            query.album.unwrap()
        );
    }
    if query.title.is_some() {
        folder_name = "music".to_string();
        file_path = format!(
            "{}/{}/{} {}",
            cache_dir,
            folder_name,
            query.artist,
            query.title.unwrap()
        );
    }

    std::fs::create_dir_all(format!("{}/{}", cache_dir.clone(), folder_name))
        .map_err(|e| e.to_string())?;

    let mut file = std::fs::File::create(file_path).map_err(|e| e.to_string())?;
    let mut content = Cursor::new(bytes.clone());
    copy(&mut content, &mut file).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;

    Ok(Some(bytes.to_vec()))
}
