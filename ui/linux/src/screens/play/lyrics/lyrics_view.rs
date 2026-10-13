use crate::shared::models::LyricLineObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct LyricsView {
    pub scrolled: gtk::ScrolledWindow,
}

impl LyricsView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let store = app_state.playback.lyrics_store();
        let selection_model = gtk::NoSelection::new(Some(store.clone()));

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_bind(move |_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let item = list_item
                .item()
                .and_downcast::<LyricLineObject>()
                .expect("item is LyricLineObject");

            let lbl = gtk::Label::new(None);
            lbl.add_css_class("lyric-line");
            lbl.set_halign(gtk::Align::Start);
            lbl.set_wrap(true);
            lbl.set_xalign(0.0);

            let text = item.text();
            if text.is_empty() {
                lbl.set_text("♪");
            } else {
                lbl.set_text(&text);
            }

            if item.is_active() {
                lbl.add_css_class("lyric-active");
            } else {
                lbl.add_css_class("lyric-inactive");
            }

            list_item.set_child(Some(&lbl));
        });

        let list_view = gtk::ListView::new(Some(selection_model), Some(factory));
        list_view.add_css_class("lyrics-list");

        // Click line -> seek
        {
            let app = app_state.clone();
            let store_clone = store.clone();
            list_view.connect_activate(move |_, pos| {
                if let Some(line) = store_clone.item(pos).and_downcast::<LyricLineObject>() {
                    app.playback.seek_ms(&app.engine, line.timestamp_ms());
                }
            });
        }

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);
        scrolled.set_child(Some(&list_view));

        // Auto-scroll on lyric index change
        {
            let lv = list_view.clone();
            let sc = scrolled.clone();
            app_state
                .playback
                .connect_current_lyric_index_notify(move |ps| {
                    let idx = ps.current_lyric_index();
                    // set_active_lyric keeps idx inside the store
                    if idx >= 0 {
                        lv.scroll_to(idx as u32, gtk::ListScrollFlags::NONE, None);
                    }
                    let _ = &sc;
                });
        }

        Self { scrolled }
    }
}
