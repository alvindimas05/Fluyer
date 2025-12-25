#[cfg(target_os = "macos")]
pub fn set_traffic_lights_position(window: &winit::window::Window, x: f64, y: f64) {
    use objc2_app_kit::{NSWindow, NSWindowButton};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};

    if let Ok(handle) = window.window_handle() {
        if let RawWindowHandle::AppKit(handle) = handle.as_raw() {
            unsafe {
                // Get NSView from the handle
                let ns_view = handle.ns_view.as_ptr() as *mut objc2::runtime::AnyObject;

                // Get the NSWindow from the view
                let ns_window: *mut NSWindow = objc2::msg_send![ns_view, window];
                let ns_window = &*ns_window;

                // Get the three traffic light buttons
                let close_button = ns_window.standardWindowButton(NSWindowButton::CloseButton);
                let miniaturize_button =
                    ns_window.standardWindowButton(NSWindowButton::MiniaturizeButton);
                let zoom_button = ns_window.standardWindowButton(NSWindowButton::ZoomButton);

                // Set positions for each button
                if let Some(button) = close_button {
                    let mut frame = button.frame();
                    frame.origin.x = x;
                    frame.origin.y = y;
                    button.setFrame(frame);
                }

                if let Some(button) = miniaturize_button {
                    let mut frame = button.frame();
                    frame.origin.x = x + 20.0; // 20px spacing between buttons
                    frame.origin.y = y;
                    button.setFrame(frame);
                }

                if let Some(button) = zoom_button {
                    let mut frame = button.frame();
                    frame.origin.x = x + 40.0; // 40px from first button
                    frame.origin.y = y;
                    button.setFrame(frame);
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
/// Setup traffic light position adjustment on window redraw
pub fn setup_traffic_lights<T: slint::ComponentHandle + 'static>(ui: &T) {
    use i_slint_backend_winit::{EventResult, WinitWindowAccessor};
    use winit::event::WindowEvent;

    let ui_weak = ui.as_weak();
    ui.window().on_winit_window_event(move |_, event| {
        if event.eq(&WindowEvent::RedrawRequested) {
            if let Some(ui) = ui_weak.upgrade() {
                ui.window().with_winit_window(|window| {
                    set_traffic_lights_position(window, 12.0, 0.0);
                });
            }
        }
        EventResult::Propagate
    });
}