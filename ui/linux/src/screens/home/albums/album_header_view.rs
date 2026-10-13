use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

/// Strip between carousel and song grid while an album is open (macOS
/// `AlbumHeaderView`): subtitle, Back, Play, Add to Queue, Shuffle.
pub struct AlbumHeaderView {
    pub container: gtk::Box,
}

impl AlbumHeaderView {
    pub fn new(app: Rc<AppState>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        container.add_css_class("album-header");
        container.set_visible(false);

        let subtitle = gtk::Label::new(None);
        subtitle.add_css_class("album-header-title");
        subtitle.set_hexpand(true);
        subtitle.set_halign(gtk::Align::Start);
        subtitle.set_ellipsize(gtk::pango::EllipsizeMode::End);
        container.append(&subtitle);

        let buttons: [(&str, &str, fn(&AppState, usize)); 4] = [
            ("go-previous-symbolic", "Back to Albums", |a, _| a.library.clear_album(&a.engine)),
            ("media-playback-start-symbolic", "Play Album", |a, i| a.engine.album_play(i)),
            ("list-add-symbolic", "Add to Queue", |a, i| a.engine.album_queue(i)),
            ("media-playlist-shuffle-symbolic", "Shuffle Album", |a, i| a.engine.album_shuffle(i)),
        ];
        for (icon, tip, action) in buttons {
            let b = gtk::Button::from_icon_name(icon);
            b.add_css_class("flat");
            b.set_tooltip_text(Some(tip));
            let app = app.clone();
            b.connect_clicked(move |_| {
                let i = app.library.selected_album_index();
                if i >= 0 {
                    action(&app, i as usize);
                }
            });
            container.append(&b);
        }

        let c = container.clone();
        app.library.connect_selected_album_index_notify(move |ls| {
            c.set_visible(ls.selected_album_index() >= 0);
            subtitle.set_text(&ls.selected_album_subtitle());
        });

        Self { container }
    }
}
