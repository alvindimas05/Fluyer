use crate::database::database::GLOBAL_DATABASE;
use crate::folder::types::MUSIC_PATH_SEPARATOR;
use crate::folder::{database, scanner, types::FolderItem};
use crate::music::commands::directory::MUSIC_STORE_PATH_NAME;
use crate::music::metadata::MusicMetadata;
use crate::platform::is_ios;
use crate::state::app_store;

#[cfg(target_os = "android")]
use crate::commands::mobile::check_read_audio_permission;

#[tauri::command]
pub fn folder_get_items(path: String) -> Vec<FolderItem> {
    scanner::get_folder_items(path.as_str())
}

#[tauri::command]
pub fn folder_get_first_music_path(path: String) -> Option<String> {
    let mut conn_guard = GLOBAL_DATABASE.lock().ok()?;
    let conn = conn_guard.as_mut()?;
    database::get_folder_first_music_path(conn, path.as_str())
}

#[tauri::command]
pub async fn music_get_all() -> Option<Vec<MusicMetadata>> {
    #[cfg(target_os = "android")]
    if !check_read_audio_permission() {
        return None;
    }

    let mut search_dirs: Vec<String> = vec![];

    // Get store music paths
    let dir = app_store().get(MUSIC_STORE_PATH_NAME)?.to_string();
    let dir_paths = dir.split(MUSIC_PATH_SEPARATOR);

    for d in dir_paths {
        let trimmed = d.trim().trim_matches('"');
        if !trimmed.is_empty() {
            search_dirs.push(trimmed.to_string());
        }
    }

    if is_ios() {
        search_dirs.push(scanner::get_home_dir())
    }

    let now = std::time::Instant::now();
    // Scan directories for music files
    let paths = scanner::scan_directories(search_dirs);
    crate::info!("Scan directories took {}s", now.elapsed().as_secs_f64());

    // database::windows_fix_music_paths_older_version(conn);

    let now = std::time::Instant::now();
    // Process files and update database
    scanner::process_supported_files(&paths).await;
    crate::info!("Process files took {}s", now.elapsed().as_secs_f64());

    let now = std::time::Instant::now();
    database::delete_non_existing_paths(paths);
    crate::info!("Delete non existing paths took {}s", now.elapsed().as_secs_f64());

    let now = std::time::Instant::now();
    let musics = database::get_all_music_from_db();
    crate::info!("Get all music took {}s", now.elapsed().as_secs_f64());

    Some(musics)
}
