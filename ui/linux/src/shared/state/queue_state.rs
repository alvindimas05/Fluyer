use crate::shared::models::TrackItemObject;
use fluyer_core::FluyerEngine;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::QueueState)]
    pub struct QueueState {
        #[property(get, set)]
        pub is_open: Cell<bool>,
        #[property(get, set)]
        pub count: Cell<u64>,

        pub tracks: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for QueueState {
        const NAME: &'static str = "FluyerQueueState";
        type Type = super::QueueState;
    }

    #[glib::derived_properties]
    impl ObjectImpl for QueueState {
        fn constructed(&self) {
            self.parent_constructed();
            *self.tracks.borrow_mut() = Some(gio::ListStore::new::<TrackItemObject>());
        }
    }
}

glib::wrapper! {
    pub struct QueueState(ObjectSubclass<imp::QueueState>);
}

impl Default for QueueState {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueState {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn tracks_store(&self) -> gio::ListStore {
        self.imp().tracks.borrow().as_ref().unwrap().clone()
    }

    pub fn reload(&self, engine: &FluyerEngine) {
        let store = self.tracks_store();
        store.remove_all();

        let queue = engine.queue_get();
        self.set_count(queue.len() as u64);

        for t in queue {
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
            store.append(&obj);
        }
    }

    pub fn goto(&self, engine: &FluyerEngine, index: usize) {
        engine.queue_goto(index);
    }

    pub fn remove(&self, engine: &FluyerEngine, index: usize) {
        engine.queue_remove(index);
        self.reload(engine);
    }

    pub fn clear(&self, engine: &FluyerEngine) {
        engine.queue_clear();
        self.reload(engine);
    }

    pub fn toggle_open(&self) {
        self.set_is_open(!self.is_open());
    }
}
