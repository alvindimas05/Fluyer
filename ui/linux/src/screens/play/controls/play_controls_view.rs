use crate::shared::AppState;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

pub struct PlayControlsView {
    pub container: gtk::Box,
}

impl PlayControlsView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 16);
        container.add_css_class("play-controls-container");
        container.set_valign(gtk::Align::Center);

        // 1. Titles
        let titles_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        titles_box.set_halign(gtk::Align::Center);

        let title_lbl = gtk::Label::new(Some("No track playing"));
        title_lbl.add_css_class("title-1");
        title_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let artist_lbl = gtk::Label::new(Some("Fluyer"));
        artist_lbl.add_css_class("title-3");
        artist_lbl.add_css_class("dim-label");
        artist_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        titles_box.append(&title_lbl);
        titles_box.append(&artist_lbl);

        {
            let t_lbl = title_lbl.clone();
            let a_lbl = artist_lbl.clone();
            app_state.playback.connect_title_notify(move |ps| {
                t_lbl.set_text(&ps.title());
                a_lbl.set_text(&format!("{} • {}", ps.artist(), ps.album()));
            });
        }

        // 2. Seek Bar + Time Label
        let seek_box = gtk::Box::new(gtk::Orientation::Vertical, 4);

        let seek_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.001);
        seek_scale.set_draw_value(false);
        seek_scale.set_hexpand(true);
        seek_scale.add_css_class("play-seek-scale");

        let is_user_seeking = Rc::new(Cell::new(false));
        {
            let app = app_state.clone();
            let seeking = is_user_seeking.clone();
            seek_scale.connect_change_value(move |scale, _, new_val| {
                seeking.set(true);
                app.playback.seek_pct(&app.engine, new_val as f32);
                scale.set_value(new_val);
                seeking.set(false);
                glib::Propagation::Stop
            });
        }

        {
            let scale = seek_scale.clone();
            let seeking = is_user_seeking.clone();
            app_state.playback.connect_progress_pct_notify(move |ps| {
                if !seeking.get() {
                    scale.set_value(ps.progress_pct() as f64);
                }
            });
        }

        let time_lbl = gtk::Label::new(Some("0:00 / 0:00"));
        time_lbl.add_css_class("caption");
        time_lbl.add_css_class("dim-label");
        time_lbl.set_halign(gtk::Align::End);
        {
            let lbl = time_lbl.clone();
            app_state.playback.connect_time_label_notify(move |ps| {
                lbl.set_text(&ps.time_label());
            });
        }

        seek_box.append(&seek_scale);
        seek_box.append(&time_lbl);

        // 3. Transport Buttons
        let transport_box = gtk::Box::new(gtk::Orientation::Horizontal, 16);
        transport_box.set_halign(gtk::Align::Center);

        let shuffle_btn = gtk::Button::from_icon_name("media-playlist-shuffle-symbolic");
        shuffle_btn.add_css_class("flat");
        shuffle_btn.set_tooltip_text(Some("Shuffle"));
        {
            let app = app_state.clone();
            shuffle_btn.connect_clicked(move |_| {
                app.playback.shuffle(&app.engine);
            });
        }

        let prev_btn = gtk::Button::from_icon_name("media-skip-backward-symbolic");
        prev_btn.add_css_class("flat");
        prev_btn.set_tooltip_text(Some("Previous"));
        {
            let app = app_state.clone();
            prev_btn.connect_clicked(move |_| {
                app.playback.previous(&app.engine);
            });
        }

        let play_btn = gtk::Button::from_icon_name("media-playback-start-symbolic");
        play_btn.add_css_class("suggested-action");
        play_btn.add_css_class("circular");
        play_btn.set_size_request(54, 54);
        play_btn.set_tooltip_text(Some("Play / Pause"));
        {
            let app = app_state.clone();
            play_btn.connect_clicked(move |_| {
                app.playback.toggle_play(&app.engine);
            });
        }

        {
            let btn = play_btn.clone();
            app_state.playback.connect_is_playing_notify(move |ps| {
                if ps.is_playing() {
                    btn.set_icon_name("media-playback-pause-symbolic");
                } else {
                    btn.set_icon_name("media-playback-start-symbolic");
                }
            });
        }

        let next_btn = gtk::Button::from_icon_name("media-skip-forward-symbolic");
        next_btn.add_css_class("flat");
        next_btn.set_tooltip_text(Some("Next"));
        {
            let app = app_state.clone();
            next_btn.connect_clicked(move |_| {
                app.playback.next(&app.engine);
            });
        }

        let repeat_btn = gtk::Button::from_icon_name("media-playlist-repeat-symbolic");
        repeat_btn.add_css_class("flat");
        repeat_btn.set_tooltip_text(Some("Repeat"));
        {
            let app = app_state.clone();
            repeat_btn.connect_clicked(move |_| {
                app.playback.cycle_repeat(&app.engine);
            });
        }

        transport_box.append(&shuffle_btn);
        transport_box.append(&prev_btn);
        transport_box.append(&play_btn);
        transport_box.append(&next_btn);
        transport_box.append(&repeat_btn);

        // 4. Volume Slider
        let vol_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        vol_box.set_halign(gtk::Align::Center);

        let vol_icon = gtk::Image::from_icon_name("audio-volume-high-symbolic");
        let vol_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.01);
        vol_scale.set_draw_value(false);
        vol_scale.set_size_request(140, -1);
        vol_scale.set_value(app_state.playback.volume() as f64);
        {
            let app = app_state.clone();
            vol_scale.connect_value_changed(move |scale| {
                app.playback.set_volume_val(&app.engine, scale.value() as f32);
            });
        }

        vol_box.append(&vol_icon);
        vol_box.append(&vol_scale);

        container.append(&titles_box);
        container.append(&seek_box);
        container.append(&transport_box);
        container.append(&vol_box);

        Self { container }
    }
}
