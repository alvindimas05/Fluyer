use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tauri::http::HeaderMap;

use crate::commands::coverart::CoverArtQuery;

const BASE_URL: &str = "https://musicbrainz.org/ws/2";
const BASE_COVER_ART_URL: &str = "https://coverartarchive.org";

#[derive(Serialize, Deserialize)]
struct ReleaseGroupResponse {
    #[serde(alias = "release-groups")]
    release_groups: Vec<ReleaseGroup>
}

#[derive(Serialize, Deserialize)]
struct ReleaseGroup {
    id: String
}

#[derive(Serialize, Deserialize)]
struct CoverArtResponse {
    images: Vec<CoverArtImage>
}

#[derive(Serialize, Deserialize)]
struct CoverArtImage {
    thumbnails: CoverArtThumbnails
}

#[derive(Serialize, Deserialize)]
struct CoverArtThumbnails {
    #[serde(alias = "500")]
    i500: String
}

// enum BrowseType {
//     Release,
//     ReleaseGroup
// }

fn user_agent() -> String {
    format!("{}/{} ( {} )",
        dotenv!("VITE_APP_NAME").to_string(),
        dotenv!("VITE_APP_VERSION").to_string(),
        dotenv!("VITE_APP_CONTACT_INFO").to_string())
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", user_agent().parse().unwrap());
    headers
}

pub struct MusicBrainz;

impl MusicBrainz {
    async fn browse(query: CoverArtQuery) -> Option<ReleaseGroupResponse> {
        let mut btype = String::from("");
        let mut bquery = String::from("");
        if query.album.is_some(){
            btype = "release-group".to_string();
            bquery = format!("{} {}", query.artist, query.album.unwrap());
        }
        if query.title.is_some(){
            btype = "release".to_string();
            bquery = format!("{} {}", query.artist, query.title.unwrap());
        }
        
        let url = format!("{}/{}?query={}&fmt=json&limit=1", BASE_URL, btype, bquery);
        
        let client = reqwest::Client::builder()
            .default_headers(headers())
            .build();
        if client.is_err(){
            return None
        }
        
        let response = client.unwrap()
            .get(url)
            .headers(headers())
            .send().await;
        if response.is_err(){
            return None
        }
        
        let json = response.unwrap().json::<ReleaseGroupResponse>().await;
        if json.is_err() {
            return None
        }
        
        Some(json.unwrap())
    }
    
    pub async fn get_cover_art(query: CoverArtQuery) -> Option<String> {
        let bresponse = MusicBrainz::browse(query).await;
        if bresponse.is_none() {
            return None
        }
        
        let release_groups = bresponse.unwrap().release_groups;
        if release_groups.len() < 1 {
            return None
        }
        
        let client = reqwest::Client::builder()
            .default_headers(headers())
            .build();
        if client.is_err(){
            return None
        }
        
        let response = client.unwrap()
            .get(format!("{}/release-group/{}", BASE_COVER_ART_URL, release_groups[0].id))
            .headers(headers())
            .send().await;
        if response.is_err(){
            return None
        }
        
        let json = response.unwrap().json::<CoverArtResponse>().await;
        if json.is_err() {
            return None
        }
        
        let images = json.unwrap().images;
        if images.len() < 1 {
            return None
        }
        
        Some(images[0].thumbnails.i500.clone())
    }
}