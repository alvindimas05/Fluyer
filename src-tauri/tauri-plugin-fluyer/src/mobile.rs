use serde::de::DeserializeOwned;
use tauri::{
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

    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
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
}
