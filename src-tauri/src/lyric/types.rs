use serde::{Deserialize, Serialize};

/// Query parameters for lyric search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricQuery {
    pub path: String, // music file path for embedded lyrics
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: Option<u64>, // in milliseconds
}

/// Status of a lyric request in the queue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LyricRequestStatus {
    Pending,
    Loaded,
    Failed,
}

/// A lyric request entry in the queue
#[derive(Debug, Clone)]
pub struct LyricRequest {
    pub name: String,
    pub status: LyricRequestStatus,
}

/// LrcLib API search result
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LrcLibResult {
    pub name: String,
    pub artist_name: String,
    pub album_name: Option<String>,
    pub duration: Option<f64>,
    pub synced_lyrics: Option<String>,
    pub plain_lyrics: Option<String>,
}
