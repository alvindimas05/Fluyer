use super::album_card::AlbumCard;
use super::album_metrics::{AlbumMetrics, CARD_GAP, EDGE};
use crate::shared::models::AlbumCardObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Full-height album grid (Albums mode). Same card size as the carousel.
/// Click opens the album and returns to Songs.
pub struct AlbumGridView {
    pub container: gtk::ScrolledWindow,
}

impl AlbumGridView {
    pub fn new(app: Rc<AppState>) -> Self {
        let store = app.library.displayed_albums_store();
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

        let grid = gtk::GridView::new(Some(gtk::NoSelection::new(Some(store.clone()))), Some(factory));
        grid.add_css_class("album-grid");
        // Each card has 6px side margins (CSS), so EDGE - 6 here keeps the AlbumMetrics math exact.
        grid.set_margin_start((EDGE - CARD_GAP / 2.0) as i32);
        grid.set_margin_end((EDGE - CARD_GAP / 2.0) as i32);

        let scrolled = gtk::ScrolledWindow::new();
        // External, not Never: Never makes the window grow to fit the grid, and the grid is sized from the window (feedback loop).
        scrolled.set_policy(gtk::PolicyType::External, gtk::PolicyType::External);
        scrolled.set_vexpand(true);
        scrolled.set_child(Some(&grid));

        // ponytail: per-frame width check, same as the carousel.
        {
            let grid = grid.clone();
            let last_w = Cell::new(0);
            scrolled.add_tick_callback(move |w, _| {
                let width = w.width();
                if width > 0 && width != last_w.get() {
                    last_w.set(width);
                    let m = AlbumMetrics::new(width as f64);
                    grid.set_min_columns(m.columns);
                    grid.set_max_columns(m.columns);
                    cover_size.set(m.cover_size as i32);
                    grid.set_model(None::<&gtk::NoSelection>);
                    grid.set_model(Some(&gtk::NoSelection::new(Some(store.clone()))));
                }
                gtk::glib::ControlFlow::Continue
            });
        }

        Self { container: scrolled }
    }
}
