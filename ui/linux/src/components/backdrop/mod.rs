mod animated_backdrop_view;
mod backdrop_mesh;
mod backdrop_renderer;
mod static_backdrop_view;

use crate::shared::AppState;
use gtk::prelude::*;
use static_backdrop_view::StaticBackdropView;
use std::rc::Rc;

/// Window backdrop. Animated GL (macOS port) when `settings.animated_background`, else the
/// static GSK blur. GL failure falls back to static for the session.
pub fn backdrop_view(app: Rc<AppState>) -> gtk::Widget {
    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    stack.set_can_target(false);
    stack.add_named(&StaticBackdropView::new(app.clone()), Some("static"));
    let gl_failed = Rc::new(std::cell::Cell::new(false));
    let animated = {
        let (stack, gl_failed) = (stack.clone(), gl_failed.clone());
        animated_backdrop_view::new(app.clone(), move || {
            gl_failed.set(true);
            // Deferred: switching the visible child inside `realize` would unrealize mid-call.
            let stack = stack.clone();
            gtk::glib::idle_add_local_once(move || stack.set_visible_child_name("static"));
        })
    };
    stack.add_named(&animated, Some("animated"));
    let sync = {
        let stack = stack.clone();
        move |s: &crate::shared::state::SettingsState| {
            let on = s.animated_background() && !gl_failed.get();
            stack.set_visible_child_name(if on { "animated" } else { "static" });
        }
    };
    sync(&app.settings);
    app.settings.connect_animated_background_notify(sync);
    stack.upcast()
}
