use crate::shared::AppState;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

pub struct PlayerBarView {
    pub container: gtk::Box,
}

impl PlayerBarView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 4);
        container.add_css_class("player-bar-container");

        // 1. Seek bar (Scale)
        let seek_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.001);
        seek_scale.set_draw_value(false);
        seek_scale.set_hexpand(true);
        seek_scale.add_css_class("player-seek-bar");

        let is_user_seeking = Rc::new(Cell::new(false));

        // Connect seek gesture / value changed
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

        // Update seek bar on progress_pct notify
        {
            let scale = seek_scale.clone();
            let seeking = is_user_seeking.clone();
            app_state.playback.connect_progress_pct_notify(move |ps| {
                if !seeking.get() {
                    scale.set_value(ps.progress_pct() as f64);
                }
            });
        }

        // 2. Control row (HBox)
        let controls_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        controls_box.add_css_class("player-bar-controls");
        controls_box.set_hexpand(true);

        // --- Left: Transport buttons ---
        let transport_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        transport_box.set_halign(gtk::Align::Start);

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
        play_btn.add_css_class("flat");
        play_btn.add_css_class("play-pause-btn");
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

        transport_box.append(&prev_btn);
        transport_box.append(&play_btn);
        transport_box.append(&next_btn);

        // --- Center: Now Playing Info ---
        let now_playing_btn = gtk::Button::new();
        now_playing_btn.add_css_class("flat");
        now_playing_btn.set_hexpand(true);
        now_playing_btn.set_halign(gtk::Align::Center);
        now_playing_btn.set_tooltip_text(Some("Open Now Playing (Lyrics)"));

        let now_playing_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        let cover_pic = gtk::Picture::new();
        cover_pic.set_size_request(42, 42);
        cover_pic.set_content_fit(gtk::ContentFit::Cover);
        cover_pic.add_css_class("cover-thumbnail");

        let titles_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        titles_box.set_valign(gtk::Align::Center);

        let title_lbl = gtk::Label::new(Some("No track playing"));
        title_lbl.add_css_class("player-bar-title");
        title_lbl.set_halign(gtk::Align::Start);
        title_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let artist_lbl = gtk::Label::new(Some("Fluyer"));
        artist_lbl.add_css_class("player-bar-artist");
        artist_lbl.set_halign(gtk::Align::Start);
        artist_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);

        titles_box.append(&title_lbl);
        titles_box.append(&artist_lbl);

        now_playing_box.append(&cover_pic);
        now_playing_box.append(&titles_box);
        now_playing_btn.set_child(Some(&now_playing_box));

        {
            let app = app_state.clone();
            now_playing_btn.connect_clicked(move |_| {
                let cur = app.playback.show_play_view();
                app.playback.set_show_play_view(!cur);
            });
        }

        {
            let t_lbl = title_lbl.clone();
            let a_lbl = artist_lbl.clone();
            let pic = cover_pic.clone();
            let app = app_state.clone();
            app_state.playback.connect_title_notify(move |ps| {
                t_lbl.set_text(&ps.title());
                a_lbl.set_text(&ps.artist());
                app.covers.current_into(&app.engine, 84, &pic);
            });
        }

        // --- Right: Secondary Controls ---
        let secondary_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        secondary_box.set_halign(gtk::Align::End);

        let time_lbl = gtk::Label::new(Some("0:00 / 0:00"));
        time_lbl.add_css_class("player-bar-time");
        time_lbl.set_valign(gtk::Align::Center);
        {
            let lbl = time_lbl.clone();
            app_state.playback.connect_time_label_notify(move |ps| {
                lbl.set_text(&ps.time_label());
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

        {
            let btn = repeat_btn.clone();
            app_state.playback.connect_repeat_mode_notify(move |ps| {
                match ps.repeat_mode() {
                    0 => {
                        btn.remove_css_class("active-control");
                        btn.set_icon_name("media-playlist-repeat-symbolic");
                    }
                    1 => {
                        btn.add_css_class("active-control");
                        btn.set_icon_name("media-playlist-repeat-symbolic");
                    }
                    _ => {
                        btn.add_css_class("active-control");
                        btn.set_icon_name("media-playlist-repeat-song-symbolic");
                    }
                }
            });
        }

        let shuffle_btn = gtk::Button::from_icon_name("media-playlist-shuffle-symbolic");
        shuffle_btn.add_css_class("flat");
        shuffle_btn.set_tooltip_text(Some("Shuffle"));
        {
            let app = app_state.clone();
            shuffle_btn.connect_clicked(move |_| {
                app.playback.shuffle(&app.engine);
            });
        }

        {
            let btn = shuffle_btn.clone();
            app_state.playback.connect_is_shuffled_notify(move |ps| {
                if ps.is_shuffled() {
                    btn.add_css_class("active-control");
                } else {
                    btn.remove_css_class("active-control");
                }
            });
        }

        let vol_btn = gtk::Button::from_icon_name("audio-volume-high-symbolic");
        vol_btn.add_css_class("flat");
        vol_btn.set_tooltip_text(Some("Mute / Unmute"));
        {
            let app = app_state.clone();
            vol_btn.connect_clicked(move |_| {
                app.playback.toggle_mute(&app.engine);
            });
        }

        let vol_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.01);
        vol_scale.set_draw_value(false);
        vol_scale.set_size_request(80, -1);
        vol_scale.set_value(app_state.playback.volume() as f64);
        {
            let app = app_state.clone();
            vol_scale.connect_value_changed(move |scale| {
                app.playback.set_volume_val(&app.engine, scale.value() as f32);
            });
        }

        {
            let btn = vol_btn.clone();
            let scale = vol_scale.clone();
            app_state.playback.connect_volume_notify(move |ps| {
                let v = ps.volume();
                scale.set_value(v as f64);
                if v <= 0.001 {
                    btn.set_icon_name("audio-volume-muted-symbolic");
                } else if v < 0.5 {
                    btn.set_icon_name("audio-volume-low-symbolic");
                } else {
                    btn.set_icon_name("audio-volume-high-symbolic");
                }
            });
        }

        secondary_box.append(&time_lbl);
        secondary_box.append(&repeat_btn);
        secondary_box.append(&shuffle_btn);
        secondary_box.append(&vol_btn);
        secondary_box.append(&vol_scale);

        controls_box.append(&transport_box);
        controls_box.append(&now_playing_btn);
        controls_box.append(&secondary_box);

        container.append(&seek_scale);
        container.append(&controls_box);

        Self { container }
    }
}
