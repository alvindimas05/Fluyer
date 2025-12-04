use serde::{Deserialize, Serialize};
use tauri::Emitter;
use crate::state::app_handle;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ToastType {
    Info,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToastData {
    message: String,
    toast_type: ToastType,
}

pub struct Toast {}

impl Toast {
    pub fn show(message: String, toast_type: ToastType) {
        app_handle().emit("toast",
    ToastData { message, toast_type }).ok();
    }
}