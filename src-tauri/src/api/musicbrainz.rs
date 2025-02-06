use serde::{Deserialize, Serialize};
use tauri::http::HeaderMap;

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

enum BrowseType {
    // Release,
    ReleaseGroup
}

fn user_agent() -> String {
    format!("{}/{} ( {} )",
        std::env::var("VITE_APP_NAME").unwrap(),
        std::env::var("VITE_APP_VERSION").unwrap(),
        std::env::var("VITE_APP_CONTACT_INFO").unwrap())
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", user_agent().parse().unwrap());
    headers
}

fn browse(browse_type: BrowseType, query: String) -> Option<ReleaseGroupResponse> {
    let btype = match browse_type {
        // BrowseType::Release => "release",
        BrowseType::ReleaseGroup => "release-group",
    };
    let url = format!("{}/{}?query={}&fmt=json&limit=1", BASE_URL, btype, query);
    
    let response = reqwest::blocking::Client::new()
        .get(url)
        .headers(headers())
        .send();
    if response.is_err(){
        return None
    }
    
    let json = response.unwrap().json::<ReleaseGroupResponse>();
    if json.is_err() {
        return None
    }
    
    Some(json.unwrap())
}

pub struct MusicBrainz;

impl MusicBrainz {
    pub fn get_cover_art_from_album(album: String) -> Option<String> {
        let bresponse = browse(BrowseType::ReleaseGroup, album);
        if bresponse.is_none() {
            return None
        }
        
        let release_groups = bresponse.unwrap().release_groups;
        if release_groups.len() < 1 {
            return None
        }
        
        let response = reqwest::blocking::Client::new()
            .get(format!("{}/release-group/{}", BASE_COVER_ART_URL, release_groups[0].id))
            .headers(headers())
            .send();
        if response.is_err(){
            return None
        }
        let json = response.unwrap().json::<CoverArtResponse>();
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