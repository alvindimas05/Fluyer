use crate::database::migrations::DATABASE_MIGRATIONS;
use crate::{logger, GLOBAL_APP_HANDLE};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

pub const DATABASE_NAME: &str = "fluyer.db";
pub static GLOBAL_DATABASE: Mutex<Option<Connection>> = Mutex::new(None);

pub fn initialize_database() {
    let app_data_dir = GLOBAL_APP_HANDLE
        .get()
        .unwrap()
        .path()
        .app_data_dir()
        .unwrap();

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_data_dir).unwrap();

    let db_path = app_data_dir
        .join(DATABASE_NAME)
        .to_str()
        .unwrap()
        .to_string();

    let mut conn = Connection::open(db_path).unwrap();
    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();
    DATABASE_MIGRATIONS.to_latest(&mut conn).unwrap();
    GLOBAL_DATABASE.lock().unwrap().replace(conn);
}
