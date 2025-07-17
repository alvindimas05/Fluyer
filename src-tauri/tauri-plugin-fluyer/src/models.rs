use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::plugin::PermissionState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToastRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationBarHeight {
    pub value: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationBarSize {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusBarHeight {
    pub value: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub audio: PermissionState,
    pub storage: PermissionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    Audio, Storage
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WatcherPlaylistChange {
    pub is_next: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WatchPlaylistChangeResponse {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PlayerCommand {
    Play,
    Pause,
    Resume,
    Stop,
    Next,
    Seek,
    Volume,
    Clear,
    RemovePlaylist,
    GotoPlaylist,
    Repeat,
    RepeatOne,
    RepeatNone,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCommandArguments {
    pub command: PlayerCommand,
    pub seek_position: Option<u64>,
    pub volume: Option<f32>,
    pub playlist_file_path: Option<String>,
    pub playlist_remove_index: Option<usize>,
    pub playlist_goto_index: Option<usize>,
}

impl PlayerCommandArguments {
    pub fn new(command: PlayerCommand) -> Self {
        Self {
            command,
            seek_position: None,
            volume: None,
            playlist_file_path: None,
            playlist_remove_index: None,
            playlist_goto_index: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGetInfo {
    pub current_position: u128,
    pub is_empty: bool,
    pub is_playing: bool,
    pub index: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPlaylistAdd {
    pub playlist: Vec<PlaylistAddMusic>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistAddMusic {
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistMoveTo {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkVersion {
    pub value: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationBarVisibility {
    pub value: bool,
}