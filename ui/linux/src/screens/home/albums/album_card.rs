use crate::shared::models::AlbumCardObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct AlbumCard {
    pub container: gtk::Box,
    pub cover_pic: gtk::Picture,
    pub name_lbl: gtk::Label,
    pub artist_lbl: gtk::Label,
    pub meta_lbl: gtk::Label,
}

impl AlbumCard {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        container.add_css_class("album-card");
        container.set_size_request(160, 230);

        let cover_pic = gtk::Picture::new();
        cover_pic.set_size_request(160, 160);
        cover_pic.set_content_fit(gtk::ContentFit::Cover);
        cover_pic.add_css_class("album-card-cover");

        let name_lbl = gtk::Label::new(None);
        name_lbl.add_css_class("album-card-name");
        name_lbl.set_halign(gtk::Align::Start);
        name_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let artist_lbl = gtk::Label::new(None);
        artist_lbl.add_css_class("album-card-artist");
        artist_lbl.set_halign(gtk::Align::Start);
        artist_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let meta_lbl = gtk::Label::new(None);
        meta_lbl.add_css_class("album-card-meta");
        meta_lbl.set_halign(gtk::Align::Start);
        meta_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        container.append(&cover_pic);
        container.append(&name_lbl);
        container.append(&artist_lbl);
        container.append(&meta_lbl);

        Self {
            container,
            cover_pic,
            name_lbl,
            artist_lbl,
            meta_lbl,
        }
    }

    pub fn bind(&self, album: &AlbumCardObject, app_state: Rc<AppState>) {
        self.name_lbl.set_text(&album.name());
        self.artist_lbl.set_text(&album.artist());

        let year = album.year();
        let label = if !year.is_empty() {
            format!("{} • {}", year, album.track_count_label())
        } else {
            album.track_count_label()
        };
        self.meta_lbl.set_text(&label);

        app_state
            .covers
            .album_into(&app_state.engine, album.index() as usize, 320, &self.cover_pic);
    }
}
