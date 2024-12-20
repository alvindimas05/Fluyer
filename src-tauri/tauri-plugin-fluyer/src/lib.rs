use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
pub mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Fluyer;
#[cfg(mobile)]
use mobile::Fluyer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the fluyer APIs.
pub trait FluyerExt<R: Runtime> {
    fn fluyer(&self) -> &Fluyer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::FluyerExt<R> for T {
    fn fluyer(&self) -> &Fluyer<R> {
        self.state::<Fluyer<R>>().inner()
    }
}

// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// struct ToastScope {
//     path: PathBuf,
// }

// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// struct FluyerScope {
//     path: PathBuf,
// }

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("fluyer")
        .invoke_handler(tauri::generate_handler![
            #[cfg(mobile)]
            commands::toast,
            #[cfg(mobile)]
            commands::check_permissions,
            #[cfg(mobile)]
            commands::request_permissions,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let fluyer = mobile::init(app, api)?;
            #[cfg(desktop)]
            let fluyer = desktop::init(app, api)?;
            app.manage(fluyer);
            Ok(())
        })
        .build()
}
