use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
struct SettingsFile {
    backdrop_source: Option<String>,
    animated_background: Option<bool>,
}

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::SettingsState)]
    pub struct SettingsState {
        #[property(get, set)]
        pub backdrop_source: RefCell<String>,
        #[property(get, set)]
        pub animated_background: std::cell::Cell<bool>,
        pub config_path: RefCell<PathBuf>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SettingsState {
        const NAME: &'static str = "FluyerSettingsState";
        type Type = super::SettingsState;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SettingsState {
        fn constructed(&self) {
            self.parent_constructed();
            let config_dir = glib::user_config_dir().join("fluyer");
            let _ = std::fs::create_dir_all(&config_dir);
            let path = config_dir.join("settings.json");
            *self.config_path.borrow_mut() = path.clone();

            let file = std::fs::read_to_string(&path)
                .ok()
                .and_then(|data| serde_json::from_str::<SettingsFile>(&data).ok())
                .unwrap_or_default();
            *self.backdrop_source.borrow_mut() = file.backdrop_source.unwrap_or_else(|| "artwork".into());
            self.animated_background.set(file.animated_background.unwrap_or(true));
        }
    }
}

glib::wrapper! {
    pub struct SettingsState(ObjectSubclass<imp::SettingsState>);
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsState {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn save(&self) {
        let imp = self.imp();
        let path = imp.config_path.borrow().clone();
        let file = SettingsFile {
            backdrop_source: Some(self.backdrop_source()),
            animated_background: Some(self.animated_background()),
        };
        if let Ok(json) = serde_json::to_string_pretty(&file) {
            let _ = std::fs::write(path, json);
        }
    }

    pub fn set_source_and_save(&self, source: &str) {
        self.set_backdrop_source(source);
        self.save();
    }
}
