use crate::shared::models::TrackItemObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

use super::queue_row::QueueRow;

pub struct QueueView {
    pub container: gtk::Box,
}

impl QueueView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        container.add_css_class("queue-panel");
        container.set_size_request(320, -1);

        // Header: "Now Playing" + Clear button
        let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        header_box.add_css_class("queue-header");
        header_box.set_hexpand(true);

        let title_lbl = gtk::Label::new(Some("Now Playing"));
        title_lbl.add_css_class("title-2");
        title_lbl.set_halign(gtk::Align::Start);
        title_lbl.set_hexpand(true);

        let clear_btn = gtk::Button::from_icon_name("user-trash-symbolic");
        clear_btn.add_css_class("flat");
        clear_btn.set_tooltip_text(Some("Clear Queue"));
        {
            let app = app_state.clone();
            clear_btn.connect_clicked(move |_| {
                app.queue.clear(&app.engine);
            });
        }

        header_box.append(&title_lbl);
        header_box.append(&clear_btn);
        container.append(&header_box);

        // List
        let store = app_state.queue.tracks_store();
        let selection_model = gtk::NoSelection::new(Some(store));

        let factory = gtk::SignalListItemFactory::new();
        {
            let app = app_state.clone();
            factory.connect_bind(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let item = list_item
                    .item()
                    .and_downcast::<TrackItemObject>()
                    .expect("item is TrackItemObject");
                let pos = list_item.position() as usize;

                let row = QueueRow::new();
                row.bind(&item, pos, app.clone());
                list_item.set_child(Some(&row.container));
            });
        }

        let list_view = gtk::ListView::new(Some(selection_model), Some(factory));
        list_view.add_css_class("queue-list");

        // Row activation: jump to track
        {
            let app = app_state.clone();
            list_view.connect_activate(move |_, pos| {
                app.queue.goto(&app.engine, pos as usize);
            });
        }

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);
        scrolled.set_child(Some(&list_view));

        container.append(&scrolled);

        Self { container }
    }
}
