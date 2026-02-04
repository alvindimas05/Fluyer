use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct CoverArtRequest {
    pub name: String,
    #[allow(dead_code)]
    pub status: CoverArtRequestStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CoverArtRequestStatus {
    Pending,
    Loaded,
    Failed,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverArtResponse {
    pub name: String,
    pub status: CoverArtRequestStatus,
    pub image: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CoverArtQuery {
    pub artist: String,
    pub album: Option<String>,
    pub title: Option<String>,
}
