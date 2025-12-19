use crate::music_scanner::{AlbumInfo, MusicMetadata};
use crate::services::{ImageService, MetadataService, MusicService};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

slint::include_modules!();

/// Constants for image loading
pub const DEBOUNCE_DELAY_MS: u64 = 50;
const THUMBNAIL_MUSIC_MAX_SIZE: u32 = 100;
const THUMBNAIL_ALBUM_MAX_SIZE: u32 = 500;

/// ViewModel handles view logic and state management
/// This is a generic viewmodel that works with AppWindow from the Slint UI
pub struct AppViewModel {
    // music_service: MusicService,
    // image_service: ImageService,
    // metadata_service: MetadataService,
}

impl AppViewModel {
    /// Create a new viewmodel instance
    pub fn new() -> Self {
        Self {
            // music_service: MusicService::new(),
            // image_service: ImageService::new(),
            // metadata_service: MetadataService::new(),
        }
    }

    /// Load music library asynchronously
    pub fn load_music_library(&self, ui: &AppWindow, music_dir: &str) {
        let ui_weak = ui.as_weak();
        let music_dir = music_dir.to_string();

        std::thread::spawn(move || {
            let service = MusicService::new();
            let music_list = service.scan_directory(&music_dir);
            let grouped = service.group_by_album(&music_list);

            let music_list = Arc::new(grouped.songs.clone());
            let music_list_clone = Arc::clone(&music_list);
            let albums = Arc::new(grouped.albums);
            let albums_clone = Arc::clone(&albums);

            // Update UI in event loop
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_weak.upgrade() {
                    Self::process_albums(&ui, (*albums_clone).clone());
                    Self::process_music_items(&ui, &music_list_clone);

                    Self::setup_progressive_loading(&ui, music_list_clone, albums_clone);
                }
            })
            .ok();
        });
    }

    /// Process albums and update UI
    fn process_albums(ui: &AppWindow, albums: Vec<crate::music_scanner::AlbumInfo>) {
        let mut album_items = Vec::new();

        for album in albums {
            album_items.push(AlbumItemData {
                cover_image: slint::Image::default(),
                title: album.album.clone().into(),
                artist: album.artist.clone().into(),
            });
        }

        let album_model = Rc::new(VecModel::from(album_items));
        ui.set_album_items(ModelRc::from(album_model));
    }

    /// Create music items from metadata and update UI
    fn process_music_items(ui: &AppWindow, music_list: &Arc<Vec<MusicMetadata>>) {
        let items: Vec<MusicItemData> = music_list
            .iter()
            .map(|metadata| {
                let (info, metadata_str) = MetadataService::format_for_display(metadata);
                MusicItemData {
                    cover_image: slint::Image::default(),
                    title: metadata.title.clone().into(),
                    info: info.into(),
                    metadata: metadata_str.into(),
                }
            })
            .collect();

        let model = Rc::new(VecModel::from(items));
        ui.set_music_items(ModelRc::from(model));
    }

    /// Setup progressive image loading with debouncing
    fn setup_progressive_loading(
        ui: &AppWindow,
        music_list: Arc<Vec<MusicMetadata>>,
        albums: Arc<Vec<AlbumInfo>>,
    ) {
        let ui_weak = ui.as_weak();

        let music_last_request_type = Arc::new(Mutex::new(Instant::now()));
        let music_pending_range = Arc::new(Mutex::new(Option::<(i32, i32)>::None));

        let album_last_request_time = Arc::new(Mutex::new(Instant::now()));
        let album_pending_range = Arc::new(Mutex::new(Option::<(i32, i32)>::None));

        ui.on_request_images(move |request_type, start_idx, end_idx| {
            if request_type == "music" {
                *music_pending_range.lock().unwrap() = Some((start_idx, end_idx));
                *music_last_request_type.lock().unwrap() = Instant::now();
            } else if request_type == "album" {
                *album_pending_range.lock().unwrap() = Some((start_idx, end_idx));
                *album_last_request_time.lock().unwrap() = Instant::now();
            }

            let pending_range = if request_type == "music" {
                Arc::clone(&music_pending_range)
            } else {
                Arc::clone(&album_pending_range)
            };
            let last_request_time = if request_type == "music" {
                Arc::clone(&music_last_request_type)
            } else {
                Arc::clone(&album_last_request_time)
            };
            let ui_weak = ui_weak.clone();
            let music_list = Arc::clone(&music_list);
            let albums = Arc::clone(&albums);
            let request_type_clone = request_type.clone();

            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(DEBOUNCE_DELAY_MS));

                let elapsed = last_request_time.lock().unwrap().elapsed();
                if elapsed < Duration::from_millis(DEBOUNCE_DELAY_MS) {
                    return;
                }

                if let Some((start, end)) = pending_range.lock().unwrap().take() {
                    if request_type_clone == "music" {
                        Self::load_music_images_for_range(
                            ui_weak.clone(),
                            Arc::clone(&music_list),
                            start,
                            end,
                        );
                    } else if request_type_clone == "album" {
                        Self::load_album_images_for_range(
                            ui_weak.clone(),
                            Arc::clone(&albums),
                            start,
                            end,
                        );
                    }
                }
            });
        });
    }

    /// Load music images for a specific range of items
    fn load_music_images_for_range(
        ui_weak: slint::Weak<AppWindow>,
        music_list: Arc<Vec<MusicMetadata>>,
        start_idx: i32,
        end_idx: i32,
    ) {
        let ui_weak_clone = ui_weak.clone();
        let music_list_clone = Arc::clone(&music_list);

        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak_clone.upgrade() {
                let model = ui.get_music_items();
                let start = start_idx.max(0) as usize;
                let end = (end_idx as usize).min(music_list_clone.len().saturating_sub(1));

                // Unload images outside visible range to free memory
                for idx in 0..model.row_count() {
                    if idx < start || idx > end {
                        if let Some(item) = model.row_data(idx) {
                            if item.cover_image.size().width > 0 {
                                let mut updated_item = item;
                                updated_item.cover_image = slint::Image::default();
                                model.set_row_data(idx, updated_item);
                            }
                        }
                    }
                }

                // Collect indices that need loading
                let mut indices_to_load = Vec::new();
                for idx in start..=end {
                    if idx >= music_list_clone.len() {
                        break;
                    }

                    if idx < model.row_count() {
                        if let Some(item) = model.row_data(idx) {
                            if item.cover_image.size().width == 0 {
                                indices_to_load.push(idx);
                            }
                        }
                    }
                }

                // Extract cover images in background thread
                if !indices_to_load.is_empty() {
                    let ui_weak_for_thread = ui_weak.clone();
                    let music_list_for_thread = Arc::clone(&music_list);

                    std::thread::spawn(move || {
                        let image_service = ImageService::new();
                        let mut loaded_images = Vec::new();

                        for idx in indices_to_load {
                            let metadata = &music_list_for_thread[idx];

                            if let Some((image_data, width, height)) = image_service
                                .load_cover_resized(&metadata.file_path, THUMBNAIL_MUSIC_MAX_SIZE)
                            {
                                loaded_images.push((idx, image_data, width, height));
                            }
                        }

                        // Update UI with loaded images
                        slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_weak_for_thread.upgrade() {
                                let model = ui.get_music_items();

                                for (idx, image_data, width, height) in loaded_images {
                                    if idx < model.row_count() {
                                        if let Some(mut item) = model.row_data(idx) {
                                            let pixel_buffer =
                                                slint::SharedPixelBuffer::clone_from_slice(
                                                    &image_data,
                                                    width,
                                                    height,
                                                );
                                            item.cover_image =
                                                slint::Image::from_rgba8(pixel_buffer);
                                            model.set_row_data(idx, item);
                                        }
                                    }
                                }
                            }
                        })
                        .ok();
                    });
                }
            }
        })
        .ok();
    }

    /// Load album images for a specific range of items
    fn load_album_images_for_range(
        ui_weak: slint::Weak<AppWindow>,
        albums: Arc<Vec<AlbumInfo>>,
        start_idx: i32,
        end_idx: i32,
    ) {
        let ui_weak_clone = ui_weak.clone();
        let albums_clone = Arc::clone(&albums);

        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak_clone.upgrade() {
                let model = ui.get_album_items();
                let start = start_idx.max(0) as usize;
                let end = (end_idx as usize).min(albums_clone.len().saturating_sub(1));

                // Unload images outside visible range to free memory
                for idx in 0..model.row_count() {
                    if idx < start || idx > end {
                        if let Some(item) = model.row_data(idx) {
                            if item.cover_image.size().width > 0 {
                                let mut updated_item = item;
                                updated_item.cover_image = slint::Image::default();
                                model.set_row_data(idx, updated_item);
                            }
                        }
                    }
                }

                // Collect indices that need loading
                let mut indices_to_load = Vec::new();
                for idx in start..=end {
                    if idx >= albums_clone.len() {
                        break;
                    }

                    if idx < model.row_count() {
                        if let Some(item) = model.row_data(idx) {
                            if item.cover_image.size().width == 0 {
                                indices_to_load.push(idx);
                            }
                        }
                    }
                }

                // Extract cover images in background thread
                if !indices_to_load.is_empty() {
                    let ui_weak_for_thread = ui_weak.clone();
                    let albums_for_thread = Arc::clone(&albums);

                    std::thread::spawn(move || {
                        let image_service = ImageService::new();
                        let mut loaded_images = Vec::new();

                        for idx in indices_to_load {
                            let album = &albums_for_thread[idx];

                            if let Some(cover_path) = &album.cover_image_path {
                                if let Some((image_data, width, height)) = image_service
                                    .load_cover_resized(cover_path, THUMBNAIL_ALBUM_MAX_SIZE)
                                {
                                    loaded_images.push((idx, image_data, width, height));
                                }
                            }
                        }

                        // Update UI with loaded images
                        slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_weak_for_thread.upgrade() {
                                let model = ui.get_album_items();

                                for (idx, image_data, width, height) in loaded_images {
                                    if idx < model.row_count() {
                                        if let Some(mut item) = model.row_data(idx) {
                                            let pixel_buffer =
                                                slint::SharedPixelBuffer::clone_from_slice(
                                                    &image_data,
                                                    width,
                                                    height,
                                                );
                                            item.cover_image =
                                                slint::Image::from_rgba8(pixel_buffer);
                                            model.set_row_data(idx, item);
                                        }
                                    }
                                }
                            }
                        })
                        .ok();
                    });
                }
            }
        })
        .ok();
    }
}

impl Default for AppViewModel {
    fn default() -> Self {
        Self::new()
    }
}
