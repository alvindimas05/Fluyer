use crate::screens::settings::SettingsView;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct ToolbarView {
    pub header_bar: adw::HeaderBar,
}

impl ToolbarView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let header_bar = adw::HeaderBar::new();
        header_bar.add_css_class("flat");

        // --- Left: Scan indicator ---
        let scan_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        scan_box.set_valign(gtk::Align::Center);
        scan_box.set_visible(false);

        let spinner = gtk::Spinner::new();
        spinner.start();

        let scan_lbl = gtk::Label::new(None);
        scan_lbl.add_css_class("caption");
        scan_lbl.add_css_class("dim-label");

        scan_box.append(&spinner);
        scan_box.append(&scan_lbl);
        header_bar.pack_start(&scan_box);

        {
            let sbox = scan_box.clone();
            let slbl = scan_lbl.clone();
            app_state.library.connect_is_scanning_notify(move |ls| {
                let scanning = ls.is_scanning();
                sbox.set_visible(scanning);
                if scanning {
                    slbl.set_text(&ls.scan_progress_label());
                }
            });
        }

        // --- Center: Mode Toggle (Songs vs Albums) ---
        let mode_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        mode_box.add_css_class("linked");

        let songs_btn = gtk::ToggleButton::new();
        songs_btn.set_icon_name("audio-x-generic-symbolic");
        songs_btn.set_tooltip_text(Some("Songs (Ctrl+1)"));
        songs_btn.set_active(true);

        let albums_btn = gtk::ToggleButton::new();
        albums_btn.set_icon_name("view-grid-symbolic");
        albums_btn.set_tooltip_text(Some("Albums (Ctrl+2)"));
        albums_btn.set_group(Some(&songs_btn));

        mode_box.append(&songs_btn);
        mode_box.append(&albums_btn);
        header_bar.set_title_widget(Some(&mode_box));

        {
            let app = app_state.clone();
            songs_btn.connect_toggled(move |btn| {
                if btn.is_active() {
                    app.library.set_mode(0);
                } else {
                    app.library.set_mode(1);
                }
            });
        }

        // --- Right: Search + Folder + Queue + Settings ---
        let right_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let search_entry = gtk::SearchEntry::new();
        search_entry.set_placeholder_text(Some("Search title, artist, album"));
        search_entry.set_size_request(200, -1);
        {
            let app = app_state.clone();
            search_entry.connect_search_changed(move |entry| {
                app.library.set_search_query(entry.text().as_str());
            });
        }

        let folder_btn = gtk::Button::from_icon_name("folder-open-symbolic");
        folder_btn.set_tooltip_text(Some("Open Music Folder (Ctrl+O)"));
        {
            let app = app_state.clone();
            folder_btn.connect_clicked(move |btn| {
                let dialog = gtk::FileDialog::new();
                dialog.set_title("Add Music Folder");
                let root = btn.root().and_downcast::<gtk::Window>();
                let app = app.clone();
                dialog.select_folder(root.as_ref(), gtk::gio::Cancellable::NONE, move |res| {
                    if let Ok(file) = res {
                        if let Some(path) = file.path() {
                            let path_str = path.to_string_lossy().to_string();
                            app.library.scan(&app.engine, &[path_str]);
                        }
                    }
                });
            });
        }

        let queue_btn = gtk::ToggleButton::new();
        queue_btn.set_icon_name("view-list-bullet-symbolic");
        queue_btn.set_tooltip_text(Some("Queue (Ctrl+L)"));
        {
            let app = app_state.clone();
            queue_btn.connect_toggled(move |btn| {
                app.queue.set_is_open(btn.is_active());
            });
        }

        {
            let qbtn = queue_btn.clone();
            app_state.queue.connect_is_open_notify(move |qs| {
                if qbtn.is_active() != qs.is_open() {
                    qbtn.set_active(qs.is_open());
                }
            });
        }

        let settings_btn = gtk::Button::from_icon_name("emblem-system-symbolic");
        settings_btn.set_tooltip_text(Some("Settings"));
        {
            let app = app_state.clone();
            settings_btn.connect_clicked(move |btn| {
                let root = btn.root().and_downcast::<gtk::Window>();
                SettingsView::show(root.as_ref(), app.clone());
            });
        }

        right_box.append(&super::sort_menu::sort_menu(&app_state.library));
        right_box.append(&search_entry);
        right_box.append(&folder_btn);
        right_box.append(&queue_btn);
        right_box.append(&settings_btn);

        header_bar.pack_end(&right_box);

        Self { header_bar }
    }
}
