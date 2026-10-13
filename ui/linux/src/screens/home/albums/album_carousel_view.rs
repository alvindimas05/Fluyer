use super::album_card::AlbumCard;
use super::album_metrics::{AlbumMetrics, EDGE};
use crate::shared::models::AlbumCardObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Horizontal strip of album covers above the song grid (macOS `AlbumCarouselView`).
/// Click a cover to open that album in the grid below.
pub struct AlbumCarouselView {
    pub container: gtk::ScrolledWindow,
}

impl AlbumCarouselView {
    pub fn new(app: Rc<AppState>) -> Self {
        let store = app.library.albums_store();
        let cover_size = Rc::new(Cell::new(AlbumMetrics::new(1100.0).cover_size as i32));

        let factory = gtk::SignalListItemFactory::new();
        {
            let app = app.clone();
            let size = cover_size.clone();
            factory.connect_bind(move |_, obj| {
                let item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let album = item.item().and_downcast::<AlbumCardObject>().unwrap();
                item.set_child(Some(&AlbumCard::new(&album, size.get(), app.clone()).container));
            });
        }

        let list = gtk::ListView::new(Some(gtk::NoSelection::new(Some(store.clone()))), Some(factory));
        list.set_orientation(gtk::Orientation::Horizontal);
        list.add_css_class("album-carousel");

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_policy(gtk::PolicyType::External, gtk::PolicyType::Never);
        scrolled.set_child(Some(&list));
        scrolled.set_margin_start(EDGE as i32);
        scrolled.set_margin_end(EDGE as i32);
        scrolled.set_visible(store.n_items() > 0);

        // Rebind every card: new size or selection (dim state) changed.
        let rebind = {
            let list = list.clone();
            let store = store.clone();
            move || {
                list.set_model(None::<&gtk::NoSelection>);
                list.set_model(Some(&gtk::NoSelection::new(Some(store.clone()))));
            }
        };

        // ponytail: GTK4 has no resize signal; a tick callback compares widths each
        // frame (one integer check). Upgrade: custom widget overriding size_allocate.
        {
            let size = cover_size.clone();
            let last_w = Cell::new(0);
            let rebind = rebind.clone();
            scrolled.add_tick_callback(move |w, _| {
                let width = w.parent().map_or(0, |p| p.width());
                if width > 0 && width != last_w.get() {
                    last_w.set(width);
                    let m = AlbumMetrics::new(width as f64);
                    w.set_size_request(-1, m.height() as i32);
                    if m.cover_size as i32 != size.get() {
                        size.set(m.cover_size as i32);
                        rebind();
                    }
                }
                gtk::glib::ControlFlow::Continue
            });
        }

        {
            let rebind = rebind.clone();
            app.library.connect_selected_album_index_notify(move |_| rebind());
        }
        {
            let s = scrolled.clone();
            store.connect_items_changed(move |st, _, _, _| s.set_visible(st.n_items() > 0));
        }

        Self { container: scrolled }
    }
}
