mod components;
mod events;
mod screens;
mod shared;

use adw::prelude::*;
use components::AnimatedBackgroundView;
use events::{AppEvent, EventBridge};
use fluyer_core::FluyerEngine;
use gtk::gdk;
use gtk::glib;
use screens::home::HomeView;
use screens::play::PlayView;
use shared::{format_time, AppState};
use std::sync::Arc;
use std::time::Duration;

const APP_ID: &str = "org.alvindimas05.fluyer";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let data_dir = glib::user_data_dir().join("fluyer");
    let cache_dir = glib::user_cache_dir().join("fluyer");

    let (event_tx, event_rx) = async_channel::unbounded::<AppEvent>();
    let bridge = Arc::new(EventBridge(event_tx));

    let engine = match FluyerEngine::new(&data_dir, &cache_dir, Some(bridge)) {
        Ok(eng) => Arc::new(eng),
        Err(err) => {
            eprintln!("Failed to start FluyerEngine: {}", err);
            return;
        }
    };

    let app_state = AppState::new(engine);

    // Drain engine events on the GTK main thread
    glib::spawn_future_local({
        let state = app_state.clone();
        async move {
            while let Ok(event) = event_rx.recv().await {
                state.handle_event(event);
            }
        }
    });

    // Clock ticker: 250ms position and active lyric sync
    glib::timeout_add_local(Duration::from_millis(250), {
        let state = app_state.clone();
        move || {
            if state.playback.is_playing() {
                let pos = state.engine.player_get_position();
                let dur = state.playback.duration_ms();
                if dur > 0 {
                    let pct = (pos as f32 / dur as f32).clamp(0.0, 1.0);
                    state.playback.set_position_ms(pos);
                    state.playback.set_progress_pct(pct);
                    let label = format!("{} / {}", format_time(pos), format_time(dur));
                    state.playback.set_time_label(label.as_str());
                    let active = state.engine.lyrics_get_active_index(pos);
                    if state.playback.current_lyric_index() != active {
                        state.playback.set_current_lyric_index(active);
                    }
                }
            }
            glib::ControlFlow::Continue
        }
    });

    load_css();

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Fluyer")
        .default_width(1100)
        .default_height(720)
        .build();

    let toast_overlay = adw::ToastOverlay::new();
    *app_state.toast_overlay.borrow_mut() = Some(toast_overlay.clone());

    let root_overlay = gtk::Overlay::new();

    // Background: GSK blurred block artwork / cover
    let backdrop = AnimatedBackgroundView::new(app_state.clone());
    root_overlay.set_child(Some(&backdrop));

    // Foreground: Stack between Home and PlayView
    let main_stack = gtk::Stack::new();
    main_stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    main_stack.set_vexpand(true);
    main_stack.set_hexpand(true);

    let home_view = HomeView::new(app_state.clone());
    let play_view = PlayView::new(app_state.clone());

    main_stack.add_named(&home_view.split_view, Some("home"));
    main_stack.add_named(&play_view.container, Some("play"));

    {
        let stack = main_stack.clone();
        app_state
            .playback
            .connect_show_play_view_notify(move |ps| {
                if ps.show_play_view() {
                    stack.set_visible_child_name("play");
                } else {
                    stack.set_visible_child_name("home");
                }
            });
    }

    root_overlay.add_overlay(&main_stack);
    toast_overlay.set_child(Some(&root_overlay));
    window.set_content(Some(&toast_overlay));

    // Keyboard Shortcuts
    let key_controller = gtk::EventControllerKey::new();
    {
        let state = app_state.clone();
        key_controller.connect_key_pressed(move |_, key, _, modifier| {
            let ctrl = modifier.contains(gdk::ModifierType::CONTROL_MASK);

            match key {
                gdk::Key::space if !ctrl => {
                    state.playback.toggle_play(&state.engine);
                    glib::Propagation::Stop
                }
                gdk::Key::Escape => {
                    if state.playback.show_play_view() {
                        state.playback.set_show_play_view(false);
                        glib::Propagation::Stop
                    } else if state.queue.is_open() {
                        state.queue.set_is_open(false);
                        glib::Propagation::Stop
                    } else {
                        glib::Propagation::Proceed
                    }
                }
                gdk::Key::l | gdk::Key::L if ctrl => {
                    state.queue.toggle_open();
                    glib::Propagation::Stop
                }
                gdk::Key::_1 if ctrl => {
                    state.library.set_mode(0);
                    glib::Propagation::Stop
                }
                gdk::Key::_2 if ctrl => {
                    state.library.set_mode(1);
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
    }
    window.add_controller(key_controller);

    window.present();
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(CSS);
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

const CSS: &str = r#"
window {
    background-color: #121214;
}

.player-bar-container {
    background: rgba(18, 18, 22, 0.85);
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    padding: 6px 16px 12px 16px;
}

.player-bar-controls {
    padding: 2px 0px;
}

.player-seek-bar highlight {
    background-color: #3584e4;
}

.queue-panel {
    background: rgba(18, 18, 22, 0.88);
    border-left: 1px solid rgba(255, 255, 255, 0.1);
    padding: 12px 16px;
}

.queue-header {
    margin-bottom: 8px;
}

.queue-row, .track-row {
    padding: 6px 10px;
    border-radius: 6px;
}

.queue-row:hover, .track-row:hover {
    background: rgba(255, 255, 255, 0.08);
}

.current-track {
    background: rgba(53, 132, 228, 0.18);
    color: #3584e4;
}

.cover-thumbnail {
    border-radius: 6px;
}

.album-grid {
    padding: 16px;
}

.album-card {
    padding: 8px;
    border-radius: 8px;
}

.album-card:hover {
    background: rgba(255, 255, 255, 0.08);
}

.album-card-cover {
    border-radius: 8px;
}

.album-card-name {
    font-weight: bold;
}

.album-card-artist, .album-card-meta {
    opacity: 0.7;
    font-size: 0.9em;
}

.active-control {
    color: #3584e4;
}

.play-view-container {
    padding: 24px;
    background: rgba(0, 0, 0, 0.25);
}

.play-view-header {
    margin-bottom: 16px;
}

.play-view-cover {
    border-radius: 12px;
}

.lyric-line {
    padding: 8px 12px;
    border-radius: 6px;
}

.lyric-active {
    font-size: 1.4em;
    font-weight: bold;
    color: #ffffff;
}

.lyric-inactive {
    font-size: 1.1em;
    color: rgba(255, 255, 255, 0.45);
}

.dim-label {
    opacity: 0.65;
}
"#;
