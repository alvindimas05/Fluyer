use crate::shared::models::AlbumCardObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

use super::album_card::AlbumCard;

pub struct AlbumGridView {
    pub container: gtk::ScrolledWindow,
}

impl AlbumGridView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let store = app_state.library.albums_store();
        let selection_model = gtk::NoSelection::new(Some(store.clone()));

        let factory = gtk::SignalListItemFactory::new();
        {
            let app = app_state.clone();
            factory.connect_bind(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let item = list_item
                    .item()
                    .and_downcast::<AlbumCardObject>()
                    .expect("item is AlbumCardObject");

                let card = AlbumCard::new();
                card.bind(&item, app.clone());
                list_item.set_child(Some(&card.container));
            });
        }

        let grid_view = gtk::GridView::new(Some(selection_model), Some(factory));
        grid_view.set_max_columns(8);
        grid_view.set_min_columns(2);
        grid_view.add_css_class("album-grid");

        // Click album -> play album
        {
            let app = app_state.clone();
            let store_clone = store.clone();
            grid_view.connect_activate(move |_, pos| {
                if let Some(item) = store_clone.item(pos).and_downcast::<AlbumCardObject>() {
                    app.library.play_album(&app.engine, item.index() as usize);
                }
            });
        }

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);
        scrolled.set_child(Some(&grid_view));

        Self { container: scrolled }
    }
}
