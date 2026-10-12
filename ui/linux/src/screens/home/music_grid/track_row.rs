use crate::shared::models::TrackItemObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct TrackRow {
    pub container: gtk::Box,
    pub cover_pic: gtk::Picture,
    pub speaker_icon: gtk::Image,
    pub title_lbl: gtk::Label,
    pub subtitle_lbl: gtk::Label,
    pub duration_lbl: gtk::Label,
}

impl TrackRow {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        container.add_css_class("track-row");
        container.set_hexpand(true);

        let cover_pic = gtk::Picture::new();
        cover_pic.set_size_request(40, 40);
        cover_pic.set_content_fit(gtk::ContentFit::Cover);
        cover_pic.add_css_class("cover-thumbnail");

        let text_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(gtk::Align::Center);

        let title_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let speaker_icon = gtk::Image::from_icon_name("audio-volume-high-symbolic");
        speaker_icon.set_pixel_size(12);
        speaker_icon.set_visible(false);

        let title_lbl = gtk::Label::new(None);
        title_lbl.add_css_class("track-row-title");
        title_lbl.set_halign(gtk::Align::Start);
        title_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        title_box.append(&speaker_icon);
        title_box.append(&title_lbl);

        let subtitle_lbl = gtk::Label::new(None);
        subtitle_lbl.add_css_class("track-row-subtitle");
        subtitle_lbl.set_halign(gtk::Align::Start);
        subtitle_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        text_box.append(&title_box);
        text_box.append(&subtitle_lbl);

        let duration_lbl = gtk::Label::new(None);
        duration_lbl.add_css_class("track-row-duration");
        duration_lbl.set_valign(gtk::Align::Center);

        container.append(&cover_pic);
        container.append(&text_box);
        container.append(&duration_lbl);

        Self {
            container,
            cover_pic,
            speaker_icon,
            title_lbl,
            subtitle_lbl,
            duration_lbl,
        }
    }

    pub fn bind(&self, track: &TrackItemObject, app_state: Rc<AppState>) {
        self.title_lbl.set_text(&track.title());
        self.subtitle_lbl
            .set_text(&format!("{} • {}", track.artist(), track.album()));
        self.duration_lbl.set_text(&track.duration_formatted());

        let is_current = track.is_current();
        self.speaker_icon.set_visible(is_current);
        if is_current {
            self.container.add_css_class("current-track");
        } else {
            self.container.remove_css_class("current-track");
        }

        app_state
            .covers
            .track_into(&app_state.engine, track.index() as usize, 80, &self.cover_pic);
    }
}
