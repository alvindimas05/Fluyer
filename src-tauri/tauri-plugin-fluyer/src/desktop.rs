use serde::de::DeserializeOwned;
use tauri::{ipc::Channel, plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Fluyer<R>> {
    Ok(Fluyer(app.clone()))
}

/// Access to the fluyer APIs.
pub struct Fluyer<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Fluyer<R> {
    pub fn toast(&self, _value: String) -> crate::Result<()> {
        Ok(())
    }

    pub fn get_navigation_bar_height(&self) -> crate::Result<NavigationBarHeight> {
        Ok(NavigationBarHeight::default())
    }

    pub fn get_navigation_bar_size(&self) -> crate::Result<NavigationBarSize> {
        Ok(NavigationBarSize::default())
    }

    pub fn get_status_bar_height(&self) -> crate::Result<StatusBarHeight> {
        Ok(StatusBarHeight::default())
    }

    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
        Ok(PermissionStatus::default())
    }

    pub fn request_permissions(
        &self,
        _permissions: Option<Vec<PermissionType>>,
    ) -> crate::Result<PermissionStatus> {
        Ok(PermissionStatus::default())
    }

    pub fn watch_playlist_change<F: Fn(WatcherPlaylistChange) + Send + Sync + 'static>(
        &self,
        _callback: F,
    ) -> crate::Result<WatchPlaylistChangeResponse> {
        Ok(WatchPlaylistChangeResponse { value: false })
    }

    pub(crate) fn watch_playlist_change_inner(&self, _channel: Channel) -> crate::Result<WatchPlaylistChangeResponse> {
        Ok(WatchPlaylistChangeResponse { value: false })
    }

    pub fn restart_app(&self) -> crate::Result<()> {
        Ok(())
    }
    
    pub fn player_run_command(&self, _args: PlayerCommandArguments) -> crate::Result<()> {
        Ok(())
    }
    
    pub fn player_get_info(&self) -> crate::Result<PlayerGetInfo> {
        Ok(PlayerGetInfo::default())
    }
}

impl<R: Runtime> Fluyer<R> {}
