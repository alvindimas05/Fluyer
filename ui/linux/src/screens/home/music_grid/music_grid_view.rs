use crate::shared::models::TrackItemObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

use super::track_row::TrackRow;

const MIN_COLUMN_WIDTH: i32 = 280;

pub struct MusicGridView {
    pub container: gtk::Stack,
}

impl MusicGridView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let container = gtk::Stack::new();
        container.set_hexpand(true);
        container.set_vexpand(true);

        // 1. Empty state
        let empty_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        empty_box.set_valign(gtk::Align::Center);
        empty_box.set_halign(gtk::Align::Center);

        let empty_icon = gtk::Image::from_icon_name("audio-x-generic-symbolic");
        empty_icon.set_pixel_size(48);
        empty_icon.add_css_class("dim-label");

        let empty_lbl = gtk::Label::new(Some("No songs in library"));
        empty_lbl.add_css_class("title-3");
        empty_lbl.add_css_class("dim-label");

        let add_folder_btn = gtk::Button::with_label("Open Music Folder...");
        add_folder_btn.add_css_class("suggested-action");
        {
            let app = app_state.clone();
            add_folder_btn.connect_clicked(move |btn| {
                let dialog = gtk::FileDialog::new();
                dialog.set_title("Select Music Folder");
                let root_win = btn.root().and_downcast::<gtk::Window>();
                let app = app.clone();
                dialog.select_folder(root_win.as_ref(), gtk::gio::Cancellable::NONE, move |res| {
                    if let Ok(file) = res {
                        if let Some(path) = file.path() {
                            let path_str = path.to_string_lossy().to_string();
                            app.library.scan(&app.engine, &[path_str]);
                        }
                    }
                });
            });
        }

        empty_box.append(&empty_icon);
        empty_box.append(&empty_lbl);
        empty_box.append(&add_folder_btn);

        // 2. Track list
        let store = app_state.library.filtered_tracks_store();
        let selection_model = gtk::NoSelection::new(Some(store.clone()));

        let factory = gtk::SignalListItemFactory::new();
        {
            let app = app_state.clone();
            factory.connect_bind(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let item = list_item
                    .item()
                    .and_downcast::<TrackItemObject>()
                    .expect("item is TrackItemObject");

                let row = TrackRow::new();
                row.bind(&item, app.clone());
                list_item.set_child(Some(&row.container));
            });
        }

        // Adaptive grid like macOS `GridItem(.adaptive(minimum: 280))`.
        let grid = gtk::GridView::new(Some(selection_model), Some(factory));
        grid.add_css_class("music-grid");
        grid.set_single_click_activate(true);
        grid.set_max_columns(12);

        // Click track: play the shown list from that track.
        {
            let app = app_state.clone();
            let store_clone = store.clone();
            grid.connect_activate(move |_, pos| {
                if let Some(t) = store_clone.item(pos).and_downcast::<TrackItemObject>() {
                    app.library.play_track(&app.engine, &t.path());
                }
            });
        }

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::External);
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);
        scrolled.set_child(Some(&grid));

        // ponytail: per-frame width check (GTK4 has no resize signal).
        {
            let grid = grid.clone();
            let last_w = std::cell::Cell::new(0);
            scrolled.add_tick_callback(move |w, _| {
                let width = w.width();
                if width > 0 && width != last_w.get() {
                    last_w.set(width);
                    let cols = ((width - 32) / (MIN_COLUMN_WIDTH + 12)).max(1) as u32;
                    grid.set_min_columns(cols);
                    grid.set_max_columns(cols);
                }
                gtk::glib::ControlFlow::Continue
            });
        }

        container.add_named(&empty_box, Some("empty"));
        container.add_named(&scrolled, Some("list"));

        // Switch between empty state and list based on track count
        {
            let stack = container.clone();
            let app = app_state.clone();
            app_state.library.connect_track_count_notify(move |ls| {
                if ls.track_count() == 0 {
                    stack.set_visible_child_name("empty");
                } else {
                    stack.set_visible_child_name("list");
                }
            });
            // Initial check
            if app.library.track_count() == 0 {
                container.set_visible_child_name("empty");
            } else {
                container.set_visible_child_name("list");
            }
        }

        Self { container }
    }
}
