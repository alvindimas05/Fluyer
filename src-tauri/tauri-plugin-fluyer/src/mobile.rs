use serde::{de::DeserializeOwned, Serialize};
use tauri::{
    ipc::{Channel, InvokeResponseBody},
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_fluyer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Fluyer<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("org.alvindimas05.fluyerplugin", "FluyerPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_fluyer)?;
    Ok(Fluyer(handle))
}

/// Access to the fluyer APIs.
pub struct Fluyer<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Fluyer<R> {
    pub fn toast(&self, value: String) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("toast", ToastRequest { value: Some(value) })
            .map_err(Into::into)
    }

    pub fn get_navigation_bar_height(&self) -> crate::Result<NavigationBarHeight> {
        self.0
            .run_mobile_plugin("getNavigationBarHeight", ())
            .map_err(Into::into)
    }

    pub fn get_navigation_bar_size(&self) -> crate::Result<NavigationBarSize> {
        self.0
            .run_mobile_plugin("getNavigationBarSize", ())
            .map_err(Into::into)
    }

    pub fn get_status_bar_height(&self) -> crate::Result<StatusBarHeight> {
        self.0
            .run_mobile_plugin("getStatusBarHeight", ())
            .map_err(Into::into)
    }

    pub fn check_permissions(&self) -> crate::Result<Option<PermissionStatus>> {
        self.0
            .run_mobile_plugin("checkPermissions", ())
            .map_err(Into::into)
    }

    pub fn request_permissions(
        &self,
        permissions: Option<Vec<PermissionType>>,
    ) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin(
                "requestPermissions",
                serde_json::json!({ "permissions": permissions }),
            )
            .map_err(Into::into)
    }

    pub fn watch_playlist_change<F: Fn(WatcherPlaylistChange) + Send + Sync + 'static>(
        &self,
        callback: F,
    ) -> crate::Result<WatchPlaylistChangeResponse> {
        let channel = Channel::new(move |event| {
            let payload = match event {
                InvokeResponseBody::Json(payload) => {
                    serde_json::from_str::<WatcherPlaylistChange>(&payload).unwrap()
                }
                _ => panic!("Failed to parse WatcherPlaylistChange payload"),
            };

            callback(payload);

            Ok(())
        });
        self.watch_playlist_change_inner(channel)
    }

    pub(crate) fn watch_playlist_change_inner(&self, channel: Channel) -> crate::Result<WatchPlaylistChangeResponse> {
        self.0
            .run_mobile_plugin("watchPlaylistChange", WatchPlaylistChangePayload { channel })
            .map_err(Into::into)
    }

    pub fn restart_app(&self) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("restartApp", ())
            .map_err(Into::into)
    }

    pub fn player_run_command(&self, args: PlayerCommandArguments) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("playerRunCommand", args)
            .map_err(Into::into)
    }
    
    pub fn player_get_info(&self) -> crate::Result<PlayerGetInfo> {
        self.0
            .run_mobile_plugin("playerGetInfo", ())
            .map_err(Into::into)
    }
    
    pub fn player_playlist_add(
        &self,
        playlist: Vec<PlaylistAddMusic>
    ) -> crate::Result<()> {
        self.0.run_mobile_plugin("playerPlaylistAdd", PlayerPlaylistAdd { playlist })
            .map_err(Into::into)
    }

    pub fn player_playlist_move_to(
        &self,
        from: usize,
        to: usize,
    ) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("playerPlaylistMoveTo", PlaylistMoveTo { from, to })
            .map_err(Into::into)
    }
    
    pub fn get_sdk_version(&self) -> crate::Result<SdkVersion> {
        self.0
            .run_mobile_plugin("getSdkVersion", ())
            .map_err(Into::into)
    }

    pub fn set_navigation_bar_visibility(
        &self,
        visible: bool,
    ) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setNavigationBarVisibility", NavigationBarVisibility { value: visible })
            .map_err(Into::into)
    }

    pub fn android_pick_folder<F: Fn(WatcherPickFolder) + Send + Sync + 'static>(
        &self,
        callback: F,
    ) -> crate::Result<WatchPickFolderResponse> {
        let channel = Channel::new(move |event| {
            let payload = match event {
                InvokeResponseBody::Json(payload) => {
                    serde_json::from_str::<WatcherPickFolder>(&payload).unwrap()
                }
                _ => panic!("Failed to parse WatcherPickFolder payload"),
            };

            callback(payload);

            Ok(())
        });
        self.android_pick_folder_inner(channel)
    }

    pub(crate) fn android_pick_folder_inner(&self, channel: Channel) -> crate::Result<WatchPickFolderResponse> {
        self.0
            .run_mobile_plugin("requestPickFolder", WatchPickFolderPayload { channel })
            .map_err(Into::into)
    }
}

#[derive(Serialize)]
struct WatchPlaylistChangePayload {
    pub channel: Channel,
}

#[derive(Serialize)]
struct WatchPickFolderPayload {
    pub channel: Channel,
}