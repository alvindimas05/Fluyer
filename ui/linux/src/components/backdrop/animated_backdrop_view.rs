//! Animated backdrop: `gtk::GLArea` driving `BackdropRenderer` every frame (port of macOS
//! `MetalBackdropView`). Falls back to the static view when GL is unavailable.
use super::backdrop_renderer::BackdropRenderer;
use crate::shared::AppState;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::ffi::{c_char, c_void, CStr};
use std::rc::Rc;
use std::time::Instant;

extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// GL entry points via libepoxy, which GTK already loaded and uses for its own contexts.
/// Epoxy exports each function as a pointer variable named `epoxy_<name>`.
fn epoxy_proc(name: &CStr) -> *const c_void {
    let symbol = format!("epoxy_{}\0", name.to_str().unwrap_or_default());
    // SAFETY: RTLD_DEFAULT (null) searches loaded libraries; the symbol is a function-pointer variable.
    unsafe {
        let var = dlsym(std::ptr::null_mut(), symbol.as_ptr().cast());
        if var.is_null() {
            std::ptr::null()
        } else {
            *(var as *const *const c_void)
        }
    }
}

pub fn new(app: Rc<AppState>, on_gl_failed: impl Fn() + 'static) -> gtk::GLArea {
    let area = gtk::GLArea::new();
    area.set_hexpand(true);
    area.set_vexpand(true);
    area.set_can_target(false);

    let renderer: Rc<RefCell<Option<BackdropRenderer>>> = Rc::default();
    let shown: Rc<RefCell<Option<gdk::Texture>>> = Rc::default();
    let epoch = Instant::now();
    let animation_time = Rc::new(Cell::new(0.0f32));
    let last_frame = Rc::new(Cell::new(None::<f64>));
    let on_gl_failed = Rc::new(on_gl_failed);

    {
        let renderer = renderer.clone();
        let on_gl_failed = on_gl_failed.clone();
        area.connect_realize(move |area| {
            area.make_current();
            if let Some(err) = area.error() {
                eprintln!("[Backdrop] GL unavailable: {err}");
                return on_gl_failed();
            }
            let es = area.context().is_some_and(|c| c.uses_es());
            // SAFETY: context made current above; loader returns epoxy dispatch pointers.
            let gl = unsafe { glow::Context::from_loader_function_cstr(epoxy_proc) };
            match unsafe { BackdropRenderer::new(gl, es) } {
                Ok(r) => *renderer.borrow_mut() = Some(r),
                Err(e) => {
                    eprintln!("[Backdrop] shader setup failed: {e}");
                    on_gl_failed();
                }
            }
        });
    }
    {
        let renderer = renderer.clone();
        let shown = shown.clone();
        area.connect_unrealize(move |area| {
            area.make_current();
            if let Some(r) = renderer.borrow_mut().take() {
                // SAFETY: context current.
                unsafe { r.destroy() };
            }
            *shown.borrow_mut() = None; // re-upload after a re-realize
        });
    }

    // ponytail: advance only while mapped (macOS freezes while hidden); Reduce Motion maps to
    // GTK's "enable animations" setting at 1/10 speed, like macOS.
    {
        let animation_time = animation_time.clone();
        let last_frame = last_frame.clone();
        area.add_tick_callback(move |area, _| {
            if area.is_mapped() {
                let now = epoch.elapsed().as_secs_f64();
                let delta = (now - last_frame.get().unwrap_or(now)).clamp(0.0, 1.0 / 15.0);
                let reduce = !area.settings().is_gtk_enable_animations();
                animation_time.set(animation_time.get() + delta as f32 / if reduce { 10.0 } else { 1.0 });
                last_frame.set(Some(now));
                area.queue_render();
            } else {
                last_frame.set(None);
            }
            glib::ControlFlow::Continue
        });
    }

    area.connect_render(move |area, _| {
        let mut guard = renderer.borrow_mut();
        let Some(r) = guard.as_mut() else { return glib::Propagation::Stop };
        let clock = epoch.elapsed().as_secs_f64();

        let source = app.settings.backdrop_source();
        if let Some(tex) = app.covers.backdrop(&app.engine, &source, area.upcast_ref()) {
            if shown.borrow().as_ref() != Some(&tex) {
                let mut dl = gdk::TextureDownloader::new(&tex);
                dl.set_format(gdk::MemoryFormat::R8g8b8a8);
                let (bytes, stride) = dl.download_bytes();
                // SAFETY: GLArea makes its context current before `render`.
                unsafe { r.set_artwork(tex.width(), tex.height(), stride as i32, &bytes, clock) };
                *shown.borrow_mut() = Some(tex);
            }
        }

        let scale = area.scale_factor();
        let size = (area.width() * scale, area.height() * scale);
        // SAFETY: as above; GLArea bound its own framebuffer, render into it.
        unsafe {
            use glow::HasContext;
            let gl_fbo = r.gl().get_parameter_i32(glow::DRAW_FRAMEBUFFER_BINDING) as u32;
            let fbo = std::num::NonZeroU32::new(gl_fbo).map(glow::NativeFramebuffer);
            r.render(fbo, size, animation_time.get(), clock);
        }
        glib::Propagation::Stop
    });

    area
}
