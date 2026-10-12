use crate::shared::AppState;
use adw::prelude::*;
use std::rc::Rc;

pub struct SettingsView;

impl SettingsView {
    pub fn show(parent: Option<&gtk::Window>, app_state: Rc<AppState>) {
        let dialog = adw::PreferencesDialog::new();
        dialog.set_title("Settings");

        let page = adw::PreferencesPage::new();
        dialog.add(&page);

        // --- Library Group ---
        let lib_group = adw::PreferencesGroup::new();
        lib_group.set_title("Music Folders");
        lib_group.set_description(Some("Folders scanned into your library"));

        let add_row = adw::ActionRow::new();
        add_row.set_title("Add Folder...");
        let add_btn = gtk::Button::from_icon_name("list-add-symbolic");
        add_btn.add_css_class("flat");
        add_btn.set_valign(gtk::Align::Center);
        add_row.add_suffix(&add_btn);
        add_row.set_activatable_widget(Some(&add_btn));

        {
            let app = app_state.clone();
            add_btn.connect_clicked(move |btn| {
                let file_dialog = gtk::FileDialog::new();
                file_dialog.set_title("Add Music Folder");
                let root = btn.root().and_downcast::<gtk::Window>();
                let app = app.clone();
                file_dialog.select_folder(root.as_ref(), gtk::gio::Cancellable::NONE, move |res| {
                    if let Ok(file) = res {
                        if let Some(path) = file.path() {
                            let path_str = path.to_string_lossy().to_string();
                            app.library.scan(&app.engine, &[path_str]);
                        }
                    }
                });
            });
        }

        lib_group.add(&add_row);
        page.add(&lib_group);

        // --- Appearance Group ---
        let app_group = adw::PreferencesGroup::new();
        app_group.set_title("Appearance");

        let backdrop_row = adw::ComboRow::new();
        backdrop_row.set_title("Backdrop Source");
        backdrop_row.set_subtitle("Artwork or palette color blocks behind the interface");

        let options = gtk::StringList::new(&["Direct Artwork", "Color Blocks"]);
        backdrop_row.set_model(Some(&options));

        if app_state.settings.backdrop_source() == "blocks" {
            backdrop_row.set_selected(1);
        } else {
            backdrop_row.set_selected(0);
        }

        {
            let app = app_state.clone();
            backdrop_row.connect_selected_notify(move |row| {
                let src = if row.selected() == 1 {
                    "blocks"
                } else {
                    "artwork"
                };
                app.settings.set_source_and_save(src);
                app.covers.invalidate_current();
                app.trigger_backdrop_redraw();
            });
        }

        app_group.add(&backdrop_row);
        page.add(&app_group);

        dialog.present(parent);
    }
}
