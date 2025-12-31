use crate::api::musicbrainz::MusicBrainz;
use crate::coverart::{cache, types::CoverArtQuery};
use crate::music::image::ImageHandler;
use std::io::copy;
use std::io::Cursor;

/// Request cover art from external API and cache it
pub async fn request_cover_art(query: CoverArtQuery, _size: Option<String>) -> Option<String> {
    let url = MusicBrainz::get_cover_art(query.clone()).await;

    if url.is_none() {
        return None;
    }

    let res = reqwest::get(url.unwrap()).await;
    if res.is_err() {
        return None;
    }

    let bytes = res.unwrap().bytes().await;
    if bytes.is_err() {
        return None;
    }
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

    if std::fs::create_dir_all(format!("{}/{}", cache_dir.clone(), folder_name)).is_err() {
        return None;
    }

    let mut file = std::fs::File::create(file_path).unwrap();
    let mut content = Cursor::new(bytes.unwrap());
    if copy(&mut content, &mut file).is_err() {
        return None;
    }
    if file.sync_all().is_err() {
        return None;
    }

    ImageHandler::encode_to_base64(content.get_ref())
}
