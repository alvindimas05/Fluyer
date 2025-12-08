use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderItem {
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FsItemType {
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
}

pub const MUSIC_PATH_SEPARATOR: &str = "||";
