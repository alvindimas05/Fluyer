use std::sync::{Arc, OnceLock};

use tauri::{App, Wry};
use tauri_plugin_store::{Store, StoreExt};

static GLOBAL_STORE_NAME: &str = "store.json";
pub static GLOBAL_APP_STORE: OnceLock<Arc<Store<Wry>>> = OnceLock::new();

pub fn init_store(app: &mut App) {
    let store = app
        .store(GLOBAL_STORE_NAME)
        .expect("Failed to initialize store.");
    if GLOBAL_APP_STORE.set(store).is_err() {
        log::error!("Failed to set GLOBAL_APP_STORE");
    }
}
