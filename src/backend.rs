use i_slint_backend_winit::Backend;
use std::error::Error;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowAttributesExtMacOS;

/// Configure window backend with macOS-specific settings
pub fn configure_backend() -> Result<Backend, Box<dyn Error>> {
    let mut backend = Backend::new()?;
    backend.window_attributes_hook = Some(Box::new(|attributes| {
        #[cfg(target_os = "macos")]
        {
            attributes
                .with_fullsize_content_view(true)
                .with_title_hidden(true)
                .with_titlebar_transparent(true)
                .with_transparent(true)
                .with_maximized(true)
        }
        #[cfg(not(target_os = "macos"))]
        {
            attributes.with_transparent(true).with_maximized(true)
        }
    }));
    Ok(backend)
}
