use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tauri::http::HeaderMap;

use crate::coverart::types::CoverArtQuery;

const BASE_URL: &str = "https://musicbrainz.org/ws/2";
const BASE_COVER_ART_URL: &str = "https://coverartarchive.org";

#[derive(Serialize, Deserialize)]
struct ReleaseGroupResponse {
    #[serde(alias = "release-groups")]
    release_groups: Vec<ReleaseGroup>,
}

#[derive(Serialize, Deserialize)]
struct ReleaseResponse {
    releases: Vec<Release>,
}

#[derive(Serialize, Deserialize)]
struct ReleaseGroup {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Release {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct CoverArtResponse {
    images: Vec<CoverArtImage>,
}

#[derive(Serialize, Deserialize)]
struct CoverArtImage {
    thumbnails: CoverArtThumbnails,
}

#[derive(Serialize, Deserialize)]
struct CoverArtThumbnails {
    #[serde(alias = "500")]
    i500: String,
}

// enum BrowseType {
//     Release,
//     ReleaseGroup
// }

fn user_agent() -> String {
    format!(
        "{}/{} ( {} )",
        dotenv!("VITE_APP_NAME").to_string(),
        dotenv!("VITE_APP_VERSION").to_string(),
        dotenv!("VITE_APP_CONTACT_INFO").to_string()
    )
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", user_agent().parse().unwrap());
    headers
}

pub struct MusicBrainz;

impl MusicBrainz {
    async fn browse(query: CoverArtQuery) -> Result<reqwest::Response, String> {
        let mut btype = String::from("");
        let mut bquery = String::from("");
        if query.album.is_some() {
            btype = "release-group".to_string();
            bquery = format!("{} {}", query.artist, query.album.unwrap());
        }
        if query.title.is_some() {
            btype = "release".to_string();
            bquery = format!("{} {}", query.artist, query.title.unwrap());
        }

        let url = format!("{}/{}?query={}&fmt=json&limit=1", BASE_URL, btype, bquery);

        let client = reqwest::Client::builder()
            .default_headers(headers())
            .build()
            .map_err(|e| e.to_string())?;

        client
            .get(url)
            .headers(headers())
            .send()
            .await
            .map_err(|e| e.to_string())
    }

    async fn browse_release_group(query: CoverArtQuery) -> Result<ReleaseGroupResponse, String> {
        let response = MusicBrainz::browse(query).await?;
        response
            .json::<ReleaseGroupResponse>()
            .await
            .map_err(|e| e.to_string())
    }

    async fn browse_release(query: CoverArtQuery) -> Result<ReleaseResponse, String> {
        let response = MusicBrainz::browse(query).await?;
        response
            .json::<ReleaseResponse>()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_cover_art(query: CoverArtQuery) -> Result<Option<String>, String> {
        let mut id = String::from("");
        let mut ctype = String::from("");

        if query.album.is_some() {
            let rg_browse = MusicBrainz::browse_release_group(query.clone()).await?;

            let release_groups = rg_browse.release_groups;
            if release_groups.is_empty() {
                return Ok(None);
            }
            ctype = "release-group".to_string();
            id = release_groups.first().unwrap().id.clone();
        }

        if query.title.is_some() {
            let r_browse = MusicBrainz::browse_release(query.clone()).await?;

            let releases = r_browse.releases;
            if releases.is_empty() {
                return Ok(None);
            }

            ctype = "release".to_string();
            id = releases.first().unwrap().id.clone();
        }

        let client = reqwest::Client::builder()
            .default_headers(headers())
            .build()
            .map_err(|e| e.to_string())?;

        let response = client
            .get(format!("{}/{}/{}", BASE_COVER_ART_URL, ctype, id))
            .headers(headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // If the cover art doesn't exist, the API might return 404.
        // We should check the status before trying to parse JSON.
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        // Perform error check for other non-success codes if desirable,
        // but sticking to original logic of just trying to parse or catching error.
        // Actually, if it's not 200 OK, parsing JSON might fail or return error JSON.
        // Let's rely on map_err for json parsing unless we want strict status checking.
        // Original code didn't check status explicitly, just allowed fail on .json().

        let json = response
            .json::<CoverArtResponse>()
            .await
            .map_err(|e| e.to_string())?;

        let images = json.images;
        if images.is_empty() {
            return Ok(None);
        }

        Ok(Some(images[0].thumbnails.i500.clone()))
    }
}
