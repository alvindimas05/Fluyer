use std::sync::Mutex;
use rusqlite::Connection;
use tauri::Manager;
use crate::database::migrations::DATABASE_MIGRATIONS;
use crate::GLOBAL_APP_HANDLE;

pub const DATABASE_NAME: &str = "fluyer.db";
pub static GLOBAL_DATABASE: Mutex<Option<Connection>> = Mutex::new(None);

pub fn initialize_database(){
    let db_path = format!("{}/{}", GLOBAL_APP_HANDLE.get().unwrap()
        .path().app_data_dir().unwrap().display(), DATABASE_NAME);
    let mut conn = Connection::open(db_path).unwrap();

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();

    DATABASE_MIGRATIONS.to_latest(&mut conn).unwrap();

    GLOBAL_DATABASE.lock().unwrap().replace(conn);
}