use crate::shared::models::TrackItemObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct QueueRow {
    pub container: gtk::Box,
    pub cover_pic: gtk::Picture,
    pub title_lbl: gtk::Label,
    pub subtitle_lbl: gtk::Label,
    pub duration_lbl: gtk::Label,
    pub remove_btn: gtk::Button,
}

impl QueueRow {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        container.add_css_class("queue-row");
        container.set_hexpand(true);

        let cover_pic = gtk::Picture::new();
        cover_pic.set_size_request(38, 38);
        cover_pic.set_content_fit(gtk::ContentFit::Cover);
        cover_pic.add_css_class("cover-thumbnail");

        let text_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(gtk::Align::Center);

        let title_lbl = gtk::Label::new(None);
        title_lbl.add_css_class("queue-row-title");
        title_lbl.set_halign(gtk::Align::Start);
        title_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let subtitle_lbl = gtk::Label::new(None);
        subtitle_lbl.add_css_class("queue-row-subtitle");
        subtitle_lbl.set_halign(gtk::Align::Start);
        subtitle_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        text_box.append(&title_lbl);
        text_box.append(&subtitle_lbl);

        let duration_lbl = gtk::Label::new(None);
        duration_lbl.add_css_class("queue-row-duration");
        duration_lbl.set_valign(gtk::Align::Center);

        let remove_btn = gtk::Button::from_icon_name("window-close-symbolic");
        remove_btn.add_css_class("flat");
        remove_btn.add_css_class("queue-remove-btn");
        remove_btn.set_valign(gtk::Align::Center);
        remove_btn.set_tooltip_text(Some("Remove from queue"));

        container.append(&cover_pic);
        container.append(&text_box);
        container.append(&duration_lbl);
        container.append(&remove_btn);

        Self {
            container,
            cover_pic,
            title_lbl,
            subtitle_lbl,
            duration_lbl,
            remove_btn,
        }
    }

    pub fn bind(&self, track: &TrackItemObject, row_idx: usize, app_state: Rc<AppState>) {
        self.title_lbl.set_text(&track.title());
        self.subtitle_lbl
            .set_text(&format!("{} • {}", track.artist(), track.album()));
        self.duration_lbl.set_text(&track.duration_formatted());

        if track.is_current() {
            self.container.add_css_class("current-track");
        } else {
            self.container.remove_css_class("current-track");
        }

        app_state
            .covers
            .track_into(&app_state.engine, track.index() as usize, 76, &self.cover_pic);

        let app = app_state.clone();
        self.remove_btn.connect_clicked(move |_| {
            app.queue.remove(&app.engine, row_idx);
        });
    }
}
