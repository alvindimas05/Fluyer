use crate::shared::models::{AlbumCardObject, TrackItemObject};
use fluyer_core::FluyerEngine;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::LibraryState)]
    pub struct LibraryState {
        #[property(get, set)]
        pub is_scanning: Cell<bool>,
        #[property(get, set)]
        pub scan_progress_label: RefCell<String>,
        #[property(get, set)]
        pub track_count: Cell<u64>,
        #[property(get, set)]
        pub album_count: Cell<u64>,
        #[property(get, set)]
        pub mode: Cell<u32>, // 0 = Songs, 1 = Albums
        #[property(get, set)]
        pub search_query: RefCell<String>,
        #[property(get, set)]
        pub selected_album_index: Cell<i64>, // -1 = None
        #[property(get, set)]
        pub selected_album_subtitle: RefCell<String>,

        /// Track paths of the open album (album rows use album-local indices).
        pub selected_album_paths: RefCell<std::collections::HashSet<String>>,

        pub tracks: RefCell<Option<gio::ListStore>>,
        pub filtered_tracks: RefCell<Option<gio::ListStore>>,
        pub albums: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LibraryState {
        const NAME: &'static str = "FluyerLibraryState";
        type Type = super::LibraryState;
    }

    #[glib::derived_properties]
    impl ObjectImpl for LibraryState {
        fn constructed(&self) {
            self.parent_constructed();
            self.selected_album_index.set(-1);
            *self.tracks.borrow_mut() = Some(gio::ListStore::new::<TrackItemObject>());
            *self.filtered_tracks.borrow_mut() = Some(gio::ListStore::new::<TrackItemObject>());
            *self.albums.borrow_mut() = Some(gio::ListStore::new::<AlbumCardObject>());
        }
    }
}

glib::wrapper! {
    pub struct LibraryState(ObjectSubclass<imp::LibraryState>);
}

impl Default for LibraryState {
    fn default() -> Self {
        Self::new()
    }
}

impl LibraryState {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn tracks_store(&self) -> gio::ListStore {
        self.imp().tracks.borrow().as_ref().unwrap().clone()
    }

    pub fn filtered_tracks_store(&self) -> gio::ListStore {
        self.imp().filtered_tracks.borrow().as_ref().unwrap().clone()
    }

    pub fn albums_store(&self) -> gio::ListStore {
        self.imp().albums.borrow().as_ref().unwrap().clone()
    }

    pub fn reload(&self, engine: &FluyerEngine) {
        let tracks_store = self.tracks_store();
        tracks_store.remove_all();

        let track_count = engine.library_get_track_count();
        self.set_track_count(track_count as u64);

        for i in 0..track_count {
            if let Some(t) = engine.library_get_track(i) {
                let obj = TrackItemObject::new(
                    t.index,
                    &t.path,
                    &t.title,
                    &t.artist,
                    &t.album,
                    t.duration_ms,
                    &t.duration_formatted,
                    t.is_current,
                );
                tracks_store.append(&obj);
            }
        }

        let albums_store = self.albums_store();
        albums_store.remove_all();

        let album_count = engine.album_get_count();
        self.set_album_count(album_count as u64);

        for i in 0..album_count {
            if let Some(a) = engine.album_get_card(i) {
                let obj = AlbumCardObject::new(
                    a.index,
                    &a.name,
                    &a.artist,
                    &a.year,
                    a.track_count,
                    &a.track_count_label,
                );
                albums_store.append(&obj);
            }
        }

        let open = self.selected_album_index();
        if open >= 0 && (open as usize) < album_count {
            self.load_album(engine, open as usize); // re-read paths after rescan
        } else {
            self.clear_album(engine);
        }
    }

    pub fn apply_filter(&self, _engine: &FluyerEngine) {
        let tracks_store = self.tracks_store();
        let filtered = self.filtered_tracks_store();
        filtered.remove_all();

        let query = self.search_query().to_lowercase();
        let album_filter = self.selected_album_index();

        for i in 0..tracks_store.n_items() {
            if let Some(track) = tracks_store.item(i).and_downcast::<TrackItemObject>() {
                let matches_query = if query.is_empty() {
                    true
                } else {
                    track.title().to_lowercase().contains(&query)
                        || track.artist().to_lowercase().contains(&query)
                        || track.album().to_lowercase().contains(&query)
                };

                let matches_album = album_filter < 0
                    || self.imp().selected_album_paths.borrow().contains(&track.path());

                if matches_query && matches_album {
                    filtered.append(&track);
                }
            }
        }
    }

    /// Open an album: grid shows its tracks; from the album grid this returns to Songs.
    pub fn select_album(&self, engine: &FluyerEngine, index: usize) {
        self.load_album(engine, index);
        self.set_mode(0);
    }

    fn load_album(&self, engine: &FluyerEngine, index: usize) {
        let Some(detail) = engine.album_get_detail(index) else {
            return self.clear_album(engine);
        };
        *self.imp().selected_album_paths.borrow_mut() =
            detail.tracks.iter().map(|t| t.path.clone()).collect();
        self.set_selected_album_subtitle(detail.subtitle);
        self.set_selected_album_index(index as i64);
        self.apply_filter(engine);
    }

    pub fn clear_album(&self, engine: &FluyerEngine) {
        self.imp().selected_album_paths.borrow_mut().clear();
        self.set_selected_album_index(-1);
        self.apply_filter(engine);
    }

    pub fn scan(&self, engine: &FluyerEngine, folders: &[String]) {
        self.set_is_scanning(true);
        self.set_scan_progress_label("Scanning music library...".to_string());
        engine.library_scan(folders);
    }

}
