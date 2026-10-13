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

        /// 0 Album (library order), 1 Title, 2 Artist, 3 Duration.
        #[property(get, set)]
        pub track_sort: Cell<u32>,
        /// 0 Name, 1 Artist, 2 Year, 3 Track Count.
        #[property(get, set)]
        pub album_sort: Cell<u32>,
        /// Shared by both modes, like macOS `sortAscending`.
        #[property(get, set)]
        pub sort_ascending: Cell<bool>,

        /// Track paths of the open album (album rows use album-local indices).
        pub selected_album_paths: RefCell<std::collections::HashSet<String>>,

        pub tracks: RefCell<Option<gio::ListStore>>,
        pub filtered_tracks: RefCell<Option<gio::ListStore>>,
        pub albums: RefCell<Option<gio::ListStore>>,
        pub displayed_albums: RefCell<Option<gio::ListStore>>,
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
            *self.displayed_albums.borrow_mut() = Some(gio::ListStore::new::<AlbumCardObject>());
            self.sort_ascending.set(true);
            // Any filter/sort input change re-derives the displayed lists.
            let obj = self.obj();
            for prop in ["search-query", "track-sort", "album-sort", "sort-ascending"] {
                obj.connect_notify_local(Some(prop), |ls, _| ls.refresh_displayed());
            }
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

    /// Albums as shown: `query` on name/artist, sorted by `album_sort`.
    pub fn displayed_albums_store(&self) -> gio::ListStore {
        self.imp().displayed_albums.borrow().as_ref().unwrap().clone()
    }

    fn albums_store(&self) -> gio::ListStore {
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
            self.clear_album();
        }
        self.refresh_displayed();
    }

    /// Re-derive the grid's tracks and the albums list (macOS `displayedTracks`
    /// / `displayedAlbums`). ponytail: rebuilt on every change; fine for a few
    /// thousand tracks, cache if big libraries stutter.
    pub fn refresh_displayed(&self) {
        let query = self.search_query();
        let query = query.trim();
        let asc = self.sort_ascending();

        let tracks: Vec<_> = self
            .sorted_source()
            .into_iter()
            .filter(|t| matches(&[t.title(), t.artist(), t.album()], query))
            .collect();
        let shown = self.filtered_tracks_store();
        shown.splice(0, shown.n_items(), &tracks);

        let mut albums: Vec<AlbumCardObject> = self
            .albums_store()
            .iter::<AlbumCardObject>()
            .flatten()
            .filter(|a| matches(&[a.name(), a.artist()], query))
            .collect();
        let sort = self.album_sort();
        albums.sort_by(|a, b| {
            let key = match sort {
                1 => natural(&a.artist(), &b.artist()),
                2 => natural(&a.year(), &b.year()),
                3 => a.track_count().cmp(&b.track_count()),
                _ => natural(&a.name(), &b.name()),
            };
            key.then(a.index().cmp(&b.index()))
        });
        if !asc {
            albums.reverse();
        }
        let shown = self.displayed_albums_store();
        shown.splice(0, shown.n_items(), &albums);
    }

    /// Unfiltered rows in grid order: the open album's tracks, else the library.
    fn sorted_source(&self) -> Vec<TrackItemObject> {
        let open = self.imp().selected_album_paths.borrow();
        let mut tracks: Vec<TrackItemObject> = self
            .tracks_store()
            .iter::<TrackItemObject>()
            .flatten()
            .filter(|t| open.is_empty() || open.contains(&t.path()))
            .collect();
        let sort = self.track_sort();
        if sort != 0 {
            tracks.sort_by(|a, b| {
                let key = match sort {
                    1 => natural(&a.title(), &b.title()),
                    2 => natural(&a.artist(), &b.artist()),
                    _ => a.duration_ms().cmp(&b.duration_ms()),
                };
                key.then(a.index().cmp(&b.index()))
            });
        }
        if !self.sort_ascending() {
            tracks.reverse();
        }
        tracks
    }

    /// Play a shown track. Queue = sorted, unfiltered list, so it follows the grid
    /// and stays whole under a search (macOS `playTrack`).
    pub fn play_track(&self, engine: &FluyerEngine, path: &str) {
        let queue = self.sorted_source();
        let start = queue.iter().position(|t| t.path() == path).unwrap_or(0);
        let indices: Vec<usize> = queue.iter().map(|t| t.index() as usize).collect();
        engine.library_play_tracks(&indices, start);
    }

    /// Header "Play": the open album in the grid's order.
    pub fn play_selected(&self, engine: &FluyerEngine) {
        if self.selected_album_index() < 0 {
            return;
        }
        let indices: Vec<usize> = self.sorted_source().iter().map(|t| t.index() as usize).collect();
        engine.library_play_tracks(&indices, 0);
    }

    /// Open an album: grid shows its tracks; from the album grid this returns to Songs.
    pub fn select_album(&self, engine: &FluyerEngine, index: usize) {
        self.load_album(engine, index);
        self.set_mode(0);
    }

    fn load_album(&self, engine: &FluyerEngine, index: usize) {
        let Some(detail) = engine.album_get_detail(index) else {
            return self.clear_album();
        };
        *self.imp().selected_album_paths.borrow_mut() =
            detail.tracks.iter().map(|t| t.path.clone()).collect();
        self.set_selected_album_subtitle(detail.subtitle);
        self.set_selected_album_index(index as i64);
        self.refresh_displayed();
    }

    pub fn clear_album(&self) {
        self.imp().selected_album_paths.borrow_mut().clear();
        self.set_selected_album_index(-1);
        self.refresh_displayed();
    }

    pub fn scan(&self, engine: &FluyerEngine, folders: &[String]) {
        self.set_is_scanning(true);
        self.set_scan_progress_label("Scanning music library...".to_string());
        engine.library_scan(folders);
    }

}

/// Finder-like order: locale-aware, case-insensitive, numbers by value ("2" < "10").
fn natural(a: &str, b: &str) -> std::cmp::Ordering {
    // casefold first: the key alone is case-sensitive under the C locale.
    let key = |s: &str| glib::FilenameCollationKey::from(glib::casefold(s));
    key(a).cmp(&key(b))
}

/// Case- and diacritic-insensitive substring match on any field.
fn matches(fields: &[String], query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = fold(query);
    fields.iter().any(|f| fold(f).contains(&q))
}

fn fold(s: &str) -> String {
    // NFD splits "é" into "e" + U+0301; dropping combining marks removes accents.
    glib::normalize(glib::casefold(s), glib::NormalizeMode::All)
        .chars()
        .filter(|c| !('\u{300}'..='\u{36f}').contains(c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_order_and_folded_search() {
        assert_eq!(natural("Track 2", "track 10"), std::cmp::Ordering::Less);
        assert!(matches(&["Beyoncé".to_string()], "beyonce"));
        assert!(!matches(&["Daft Punk".to_string()], "dabin"));
    }

    #[test]
    fn sort_and_filter_tracks() {
        let ls = LibraryState::new();
        for (i, title, dur) in [(0, "b 10", 300), (1, "B 2", 100), (2, "a", 200)] {
            ls.tracks_store().append(&TrackItemObject::new(
                i, &format!("/{i}"), title, "x", "y", dur, "", false,
            ));
        }
        let titles = || -> Vec<String> {
            ls.filtered_tracks_store().iter::<TrackItemObject>().flatten().map(|t| t.title().into()).collect()
        };
        ls.refresh_displayed();
        assert_eq!(titles(), ["b 10", "B 2", "a"]); // Album = library order
        ls.set_track_sort(1);
        assert_eq!(titles(), ["a", "B 2", "b 10"]);
        ls.set_sort_ascending(false);
        assert_eq!(titles(), ["b 10", "B 2", "a"]);
        ls.set_search_query("b");
        assert_eq!(titles(), ["b 10", "B 2"]);
    }
}
