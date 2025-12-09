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
                let close_button =
                    ns_window.standardWindowButton(NSWindowButton::CloseButton);
                let miniaturize_button =
                    ns_window.standardWindowButton(NSWindowButton::MiniaturizeButton);
                let zoom_button =
                    ns_window.standardWindowButton(NSWindowButton::ZoomButton);

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

#[cfg(not(target_os = "macos"))]
/// No-op implementation for non-macOS platforms
pub fn set_traffic_lights_position(_window: &winit::window::Window, _x: f64, _y: f64) {
    // No-op on non-macOS platforms
}
