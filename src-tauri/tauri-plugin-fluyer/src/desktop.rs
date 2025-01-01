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

    pub fn watch_state<F: Fn(WatcherState) + Send + Sync + 'static>(
        &self,
        _callback: F,
    ) -> crate::Result<WatchResponse> {
        Ok(WatchResponse { value: false })
    }

    pub(crate) fn watch_state_inner(&self, _channel: Channel) -> crate::Result<WatchResponse> {
        Ok(WatchResponse { value: false })
    }
    
    pub fn listen_to_headset_change(&self) -> crate::Result<()> {
        Ok(())
    }
}

impl<R: Runtime> Fluyer<R> {}
