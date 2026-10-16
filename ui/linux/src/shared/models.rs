use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// ---------------------------------------------------------------------------
// TrackItemObject
// ---------------------------------------------------------------------------

mod track_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::TrackItemObject)]
    pub struct TrackItemObject {
        #[property(get, set)]
        pub index: std::cell::Cell<u64>,
        #[property(get, set)]
        pub path: std::cell::RefCell<String>,
        #[property(get, set)]
        pub title: std::cell::RefCell<String>,
        #[property(get, set)]
        pub artist: std::cell::RefCell<String>,
        #[property(get, set)]
        pub album: std::cell::RefCell<String>,
        #[property(get, set)]
        pub duration_ms: std::cell::Cell<u64>,
        #[property(get, set)]
        pub duration_formatted: std::cell::RefCell<String>,
        #[property(get, set)]
        pub is_current: std::cell::Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TrackItemObject {
        const NAME: &'static str = "FluyerTrackItemObject";
        type Type = super::TrackItemObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TrackItemObject {}
}

glib::wrapper! {
    pub struct TrackItemObject(ObjectSubclass<track_imp::TrackItemObject>);
}

impl TrackItemObject {
    pub fn new(
        index: u64,
        path: &str,
        title: &str,
        artist: &str,
        album: &str,
        duration_ms: u64,
        duration_formatted: &str,
        is_current: bool,
    ) -> Self {
        let obj: Self = glib::Object::new();
        obj.set_index(index);
        obj.set_path(path);
        obj.set_title(title);
        obj.set_artist(artist);
        obj.set_album(album);
        obj.set_duration_ms(duration_ms);
        obj.set_duration_formatted(duration_formatted);
        obj.set_is_current(is_current);
        obj
    }
}

// ---------------------------------------------------------------------------
// AlbumCardObject
// ---------------------------------------------------------------------------

mod album_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::AlbumCardObject)]
    pub struct AlbumCardObject {
        #[property(get, set)]
        pub index: std::cell::Cell<u64>,
        #[property(get, set)]
        pub name: std::cell::RefCell<String>,
        #[property(get, set)]
        pub artist: std::cell::RefCell<String>,
        #[property(get, set)]
        pub year: std::cell::RefCell<String>,
        #[property(get, set)]
        pub track_count: std::cell::Cell<u64>,
        #[property(get, set)]
        pub track_count_label: std::cell::RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AlbumCardObject {
        const NAME: &'static str = "FluyerAlbumCardObject";
        type Type = super::AlbumCardObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for AlbumCardObject {}
}

glib::wrapper! {
    pub struct AlbumCardObject(ObjectSubclass<album_imp::AlbumCardObject>);
}

impl AlbumCardObject {
    pub fn new(
        index: u64,
        name: &str,
        artist: &str,
        year: &str,
        track_count: u64,
        track_count_label: &str,
    ) -> Self {
        let obj: Self = glib::Object::new();
        obj.set_index(index);
        obj.set_name(name);
        obj.set_artist(artist);
        obj.set_year(year);
        obj.set_track_count(track_count);
        obj.set_track_count_label(track_count_label);
        obj
    }
}

// ---------------------------------------------------------------------------
// LyricLineObject
// ---------------------------------------------------------------------------

mod lyric_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::LyricLineObject)]
    pub struct LyricLineObject {
        #[property(get, set)]
        pub timestamp_ms: std::cell::Cell<u64>,
        #[property(get, set)]
        pub text: std::cell::RefCell<String>,
        #[property(get, set)]
        pub is_active: std::cell::Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LyricLineObject {
        const NAME: &'static str = "FluyerLyricLineObject";
        type Type = super::LyricLineObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for LyricLineObject {}
}

glib::wrapper! {
    pub struct LyricLineObject(ObjectSubclass<lyric_imp::LyricLineObject>);
}

impl LyricLineObject {
    pub fn new(timestamp_ms: u64, text: &str, is_active: bool) -> Self {
        let obj: Self = glib::Object::new();
        obj.set_timestamp_ms(timestamp_ms);
        obj.set_text(text);
        obj.set_is_active(is_active);
        obj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_and_album_objects() {
        let track = TrackItemObject::new(
            42,
            "/music/song.flac",
            "Midnight City",
            "M83",
            "Hurry Up, We're Dreaming",
            243000,
            "4:03",
            true,
        );
        assert_eq!(track.index(), 42);
        assert_eq!(track.title(), "Midnight City");
        assert_eq!(track.artist(), "M83");
        assert_eq!(track.album(), "Hurry Up, We're Dreaming");
        assert_eq!(track.duration_ms(), 243000);
        assert_eq!(track.duration_formatted(), "4:03");
        assert!(track.is_current());

        track.set_is_current(false);
        assert!(!track.is_current());

        let album = AlbumCardObject::new(
            7,
            "Random Access Memories",
            "Daft Punk",
            "2013",
            13,
            "13 tracks",
        );
        assert_eq!(album.index(), 7);
        assert_eq!(album.name(), "Random Access Memories");
        assert_eq!(album.artist(), "Daft Punk");
        assert_eq!(album.year(), "2013");
        assert_eq!(album.track_count(), 13);
        assert_eq!(album.track_count_label(), "13 tracks");

        let lyric = LyricLineObject::new(12500, "Waiting in a car", true);
        assert_eq!(lyric.timestamp_ms(), 12500);
        assert_eq!(lyric.text(), "Waiting in a car");
        assert!(lyric.is_active());
        lyric.set_is_active(false);
        assert!(!lyric.is_active());
    }
}
