use serde::{Deserialize, Serialize};
use tauri::plugin::PermissionState;
use specta::Type;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ToastRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub audio: PermissionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    Audio,
}


// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ToastResponse {}

// #[derive(Serialize)]
// pub struct RequestReadAudioPayload {
//     pub channel: Channel,
// }

// #[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RequestReadAudioEvent {
//     pub result: bool
// }
