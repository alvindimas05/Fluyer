use crate::screens::play::controls::PlayControlsView;
use crate::screens::play::lyrics::LyricsView;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct PlayView {
    pub container: gtk::Box,
}

impl PlayView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.add_css_class("play-view-container");
        container.set_vexpand(true);
        container.set_hexpand(true);

        // Header with Back button
        let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        header_box.add_css_class("play-view-header");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.add_css_class("flat");
        back_btn.add_css_class("circular");
        back_btn.set_tooltip_text(Some("Back (Esc)"));
        {
            let app = app_state.clone();
            back_btn.connect_clicked(move |_| {
                app.playback.set_show_play_view(false);
            });
        }

        header_box.append(&back_btn);
        container.append(&header_box);

        // Two-column layout
        let body_box = gtk::Box::new(gtk::Orientation::Horizontal, 32);
        body_box.set_vexpand(true);
        body_box.set_hexpand(true);
        body_box.add_css_class("play-view-body");

        // Left column: Cover Art + PlayControlsView
        let left_col = gtk::Box::new(gtk::Orientation::Vertical, 20);
        left_col.set_valign(gtk::Align::Center);
        left_col.set_halign(gtk::Align::Center);
        left_col.set_hexpand(true);

        let big_cover = gtk::Picture::new();
        big_cover.set_size_request(340, 340);
        big_cover.set_content_fit(gtk::ContentFit::Cover);
        big_cover.add_css_class("play-view-cover");

        {
            let pic = big_cover.clone();
            let app = app_state.clone();
            app_state.playback.connect_title_notify(move |_| {
                app.covers.current_into(&app.engine, 680, &pic);
            });
            app_state.covers.current_into(&app_state.engine, 680, &big_cover);
        }

        let controls = PlayControlsView::new(app_state.clone());

        left_col.append(&big_cover);
        left_col.append(&controls.container);

        // Right column: LyricsView
        let lyrics = LyricsView::new(app_state.clone());
        lyrics.scrolled.set_hexpand(true);
        lyrics.scrolled.set_vexpand(true);
        lyrics.scrolled.set_size_request(380, -1);

        body_box.append(&left_col);
        body_box.append(&lyrics.scrolled);

        container.append(&body_box);

        Self { container }
    }
}
