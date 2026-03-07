use gtk::prelude::*;
use tauri::{Emitter, Manager};

use crate::state::app_handle;

#[derive(Clone, serde::Serialize)]
struct MouseLeavePayload {
    x: f64,
    y: f64,
}

pub fn linux_listen_mouse_leave() -> Result<(), tauri::Error> {
    let window = app_handle().get_webview_window("main").unwrap();
    let gtk_window = window.gtk_window()?;

    gtk_window.connect_leave_notify_event(move |_, crossing| {
        let (x, y) = crossing.position();
        crate::debug!("GTK Mouse Leave: {:?}", crossing.position());
        let _ = app_handle().emit(crate::commands::route::SIDEBAR_MOUSE_LEAVE, MouseLeavePayload { x, y });
        gtk::glib::Propagation::Proceed
    });

    Ok(())
}
