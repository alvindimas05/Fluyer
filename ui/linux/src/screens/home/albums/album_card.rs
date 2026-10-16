use crate::shared::models::AlbumCardObject;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

/// One album cover with name + artist (macOS `AlbumCard`). Click opens the album,
/// right-click offers Play / Add to Queue / Shuffle. While another album is open
/// the rest dim (`dimmed` CSS class).
pub struct AlbumCard {
    pub container: gtk::Box,
}

impl AlbumCard {
    pub fn new(album: &AlbumCardObject, cover_size: i32, app: Rc<AppState>) -> Self {
        let index = album.index() as usize;

        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        container.add_css_class("album-card");
        container.set_size_request(cover_size, -1);
        container.set_halign(gtk::Align::Start);
        container.set_valign(gtk::Align::Start);

        let cover = gtk::Picture::new();
        cover.set_size_request(cover_size, cover_size);
        cover.set_can_shrink(true);
        cover.set_content_fit(gtk::ContentFit::Cover);
        cover.set_overflow(gtk::Overflow::Hidden);
        cover.add_css_class("album-card-cover");

        let name = label(&album.name(), "album-card-name");
        let artist = label(&album.artist(), "album-card-artist");

        container.append(&cover);
        container.append(&name);
        container.append(&artist);

        // 2x pixels for HiDPI, like macOS.
        let px = (cover_size.max(16) * 2) as u32;
        app.covers.album_into(&app.engine, index, px, &cover);

        let selected = app.library.selected_album_index();
        if selected == index as i64 {
            container.add_css_class("selected");
        } else if selected >= 0 {
            container.add_css_class("dimmed");
        }

        let click = gtk::GestureClick::new();
        {
            let app = app.clone();
            click.connect_released(move |_, _, _, _| app.library.select_album(&app.engine, index));
        }
        container.add_controller(click);

        let menu = gtk::GestureClick::builder().button(3).build();
        {
            let app = app.clone();
            let anchor = container.clone();
            menu.connect_pressed(move |_, _, x, y| show_menu(&anchor, &app, index, x, y));
        }
        container.add_controller(menu);

        Self { container }
    }
}

fn label(text: &str, class: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    l.add_css_class(class);
    // Fill the card width; max_width_chars(1) keeps long names from widening it.
    l.set_halign(gtk::Align::Fill);
    l.set_xalign(0.0);
    l.set_ellipsize(gtk::pango::EllipsizeMode::End);
    l.set_max_width_chars(1);
    l
}

fn show_menu(anchor: &gtk::Box, app: &Rc<AppState>, index: usize, x: f64, y: f64) {
    let pop = gtk::Popover::new();
    pop.set_parent(anchor);
    pop.set_has_arrow(false);
    pop.set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
    let col = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let items: [(&str, fn(&fluyer_core::FluyerEngine, usize)); 3] = [
        ("Play", |e, i| e.album_play(i)),
        ("Add to Queue", |e, i| e.album_queue(i)),
        ("Shuffle", |e, i| e.album_shuffle(i)),
    ];
    for (title, action) in items {
        let b = gtk::Button::with_label(title);
        b.add_css_class("flat");
        let app = app.clone();
        let p = pop.clone();
        b.connect_clicked(move |_| {
            action(&app.engine, index);
            p.popdown();
        });
        col.append(&b);
    }
    pop.set_child(Some(&col));
    pop.connect_closed(|p| p.unparent());
    pop.popup();
}
