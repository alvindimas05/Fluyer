#![cfg(target_os = "linux")]

use gtk::prelude::*;
use image::RgbaImage;
use std::cell::RefCell;
use std::rc::Rc;
use tauri::{AppHandle, Manager};

use crate::state::app_handle;

struct AppState {
    gl: glow::Context,
    program: glow::Program,
    vertex_array: glow::VertexArray,
    texture_current: glow::Texture,
    texture_next: glow::Texture,
}

pub struct LinuxRendererState {
    app_state: Rc<RefCell<Option<AppState>>>,
    transition_start_time: Option<std::time::Instant>,
    mix_ratio: f32,
    needs_redraw: bool,
    cached_image: Option<RgbaImage>,
    upload_current: Option<RgbaImage>,
    upload_next: Option<RgbaImage>,
}

unsafe impl Send for LinuxRendererState {}
unsafe impl Sync for LinuxRendererState {}

pub struct SharedLinuxRenderer {
    pub state: std::sync::Mutex<LinuxRendererState>,
}

pub fn setup_linux_background(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    crate::debug!("setup_linux_background: Starting GTK OpenGL initialization");

    let window = app.get_webview_window("main").unwrap();

    // 1. Get the underlying GTK components
    let gtk_window = window.gtk_window()?;

    let overlay = gtk::Overlay::new();
    let gl_area = gtk::GLArea::new();
    gl_area.set_has_alpha(true);
    gl_area.set_auto_render(true);

    let state: Rc<RefCell<Option<AppState>>> = Rc::new(RefCell::new(None));
    let state_realize = state.clone();

    gl_area.connect_realize(move |gl_area| {
        gl_area.make_current();
        if gl_area.error().is_some() {
            crate::error!("Error creating GLArea context");
            return;
        }

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                let mut ptr = std::ptr::null();
                let name = std::ffi::CString::new(s).unwrap();

                if let Ok(lib) = libloading::Library::new("libGL.so.1") {
                    if let Ok(sym) = lib
                        .get::<unsafe extern "C" fn(*const i8) -> *const std::ffi::c_void>(
                            b"glXGetProcAddress\0",
                        )
                    {
                        ptr = sym(name.as_ptr());
                    }
                }
                if ptr.is_null() {
                    if let Ok(lib) = libloading::Library::new("libEGL.so.1") {
                        if let Ok(sym) =
                            lib.get::<unsafe extern "C" fn(*const i8) -> *const std::ffi::c_void>(
                                b"eglGetProcAddress\0",
                            )
                        {
                            ptr = sym(name.as_ptr());
                        }
                    }
                }
                ptr
            })
        };

        unsafe {
            use glow::HasContext as _;

            let vertex_array = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vertex_array));

            let program = gl.create_program().expect("Cannot create program");

            let vertex_shader_source = r#"
            #version 330 core
            out vec2 TexCoords;
            void main() {
                vec2 pos[4] = vec2[4](
                    vec2(-1.0, -1.0),
                    vec2( 1.0, -1.0),
                    vec2(-1.0,  1.0),
                    vec2( 1.0,  1.0)
                );
                vec2 tex[4] = vec2[4](
                    vec2(0.0, 1.0),
                    vec2(1.0, 1.0),
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0)
                );
                gl_Position = vec4(pos[gl_VertexID], 0.0, 1.0);
                TexCoords = tex[gl_VertexID];
            }
            "#;

            let fragment_shader_source = r#"
            #version 330 core
            in vec2 TexCoords;
            uniform float mix_ratio;
            uniform sampler2D tex_current;
            uniform sampler2D tex_next;
            out vec4 FragColor;
            
            void main() {
                vec4 current = texture(tex_current, TexCoords);
                vec4 next = texture(tex_next, TexCoords);
                FragColor = mix(current, next, mix_ratio);
            }
            "#;

            let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vs, vertex_shader_source);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                panic!("{}", gl.get_shader_info_log(vs));
            }

            let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fs, fragment_shader_source);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                panic!("{}", gl.get_shader_info_log(fs));
            }

            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.detach_shader(program, vs);
            gl.delete_shader(vs);
            gl.detach_shader(program, fs);
            gl.delete_shader(fs);

            let texture_current = gl.create_texture().unwrap();
            let texture_next = gl.create_texture().unwrap();

            *state_realize.borrow_mut() = Some(AppState {
                gl,
                program,
                vertex_array,
                texture_current,
                texture_next,
            });
        }
    });

    let shared_renderer = std::sync::Arc::new(SharedLinuxRenderer {
        state: std::sync::Mutex::new(LinuxRendererState {
            app_state: state.clone(),
            transition_start_time: None,
            mix_ratio: 0.0,
            needs_redraw: false,
            cached_image: None,
            upload_next: None,
            upload_current: None,
        }),
    });

    let state_render_arc = shared_renderer.clone();
    gl_area.connect_render(move |_gl_area, _gl_context| {
        let mut s = state_render_arc.state.lock().unwrap();

        let upload_next = s.upload_next.take();
        let upload_current = s.upload_current.take();
        let mix_ratio = s.mix_ratio;

        if let Some(app_state) = s.app_state.borrow_mut().as_mut() {
            unsafe {
                use glow::HasContext as _;

                if let Some(img) = upload_next {
                    std::mem::swap(&mut app_state.texture_current, &mut app_state.texture_next);

                    app_state
                        .gl
                        .bind_texture(glow::TEXTURE_2D, Some(app_state.texture_next));
                    app_state.gl.tex_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        glow::RGBA,
                        glow::UNSIGNED_BYTE,
                        glow::PixelUnpackData::Slice(Some(img.as_raw())),
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MIN_FILTER,
                        glow::LINEAR as i32,
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MAG_FILTER,
                        glow::LINEAR as i32,
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_S,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_T,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                }

                if let Some(img) = upload_current {
                    app_state
                        .gl
                        .bind_texture(glow::TEXTURE_2D, Some(app_state.texture_current));
                    app_state.gl.tex_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        glow::RGBA,
                        glow::UNSIGNED_BYTE,
                        glow::PixelUnpackData::Slice(Some(img.as_raw())),
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MIN_FILTER,
                        glow::LINEAR as i32,
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MAG_FILTER,
                        glow::LINEAR as i32,
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_S,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    app_state.gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_T,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                }

                app_state.gl.clear_color(0.1, 0.2, 0.3, 1.0);
                app_state.gl.clear(glow::COLOR_BUFFER_BIT);

                app_state.gl.use_program(Some(app_state.program));

                app_state.gl.active_texture(glow::TEXTURE0);
                app_state
                    .gl
                    .bind_texture(glow::TEXTURE_2D, Some(app_state.texture_current));
                let tc_loc = app_state
                    .gl
                    .get_uniform_location(app_state.program, "tex_current");
                app_state.gl.uniform_1_i32(tc_loc.as_ref(), 0);

                app_state.gl.active_texture(glow::TEXTURE1);
                app_state
                    .gl
                    .bind_texture(glow::TEXTURE_2D, Some(app_state.texture_next));
                let tn_loc = app_state
                    .gl
                    .get_uniform_location(app_state.program, "tex_next");
                app_state.gl.uniform_1_i32(tn_loc.as_ref(), 1);

                let mix_loc = app_state
                    .gl
                    .get_uniform_location(app_state.program, "mix_ratio");
                app_state.gl.uniform_1_f32(mix_loc.as_ref(), mix_ratio);

                app_state.gl.bind_vertex_array(Some(app_state.vertex_array));
                app_state.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            }
        }
        gtk::glib::Propagation::Proceed
    });

    let tick_renderer = shared_renderer.clone();
    gl_area.add_tick_callback(move |widget, _frame_clock| {
        let mut slock = tick_renderer.state.lock().unwrap();
        let mut redraw = false;

        if slock.needs_redraw {
            redraw = true;
            slock.needs_redraw = false;
        }

        if let Some(start) = slock.transition_start_time {
            let elapsed = start.elapsed().as_secs_f32();
            if elapsed < 1.0 {
                slock.mix_ratio = elapsed / 1.0;
                redraw = true;
            } else {
                slock.mix_ratio = 1.0;
                slock.transition_start_time = None;
                redraw = true;
            }
        }

        if redraw {
            widget.queue_render();
        }

        gtk::glib::ControlFlow::Continue
    });

    let state_unrealize = state.clone();
    gl_area.connect_unrealize(move |gl_area| {
        gl_area.make_current();
        if let Some(state) = state_unrealize.borrow_mut().take() {
            unsafe {
                use glow::HasContext as _;
                state.gl.delete_program(state.program);
                state.gl.delete_vertex_array(state.vertex_array);
                state.gl.delete_texture(state.texture_current);
                state.gl.delete_texture(state.texture_next);
            }
        }
    });

    if let Some(child) = gtk_window.child() {
        gtk_window.remove(&child);
        overlay.add(&gl_area);

        if let Ok(container) = child.clone().downcast::<gtk::Container>() {
            for c in container.children() {
                container.remove(&c);
                overlay.add_overlay(&c);
            }
        } else {
            overlay.add_overlay(&child);
        }

        gtk_window.add(&overlay);
    } else {
        overlay.add(&gl_area);
        gtk_window.add(&overlay);
    }

    overlay.show_all();

    app.manage(shared_renderer);

    Ok(())
}

pub fn update_background(img: RgbaImage) {
    if let Some(shared) = app_handle().try_state::<std::sync::Arc<SharedLinuxRenderer>>() {
        let mut state = shared.state.lock().unwrap();
        state.cached_image = Some(img.clone());
        state.upload_next = Some(img);
        state.transition_start_time = Some(std::time::Instant::now());
        state.mix_ratio = 0.0;
        state.needs_redraw = true;
    }
}

pub fn restore_background() {
    if let Some(shared) = app_handle().try_state::<std::sync::Arc<SharedLinuxRenderer>>() {
        let mut state = shared.state.lock().unwrap();

        if let Some(img) = state.cached_image.clone() {
            state.upload_current = Some(img);
            state.transition_start_time = None;
            state.mix_ratio = 0.0;
            state.needs_redraw = true;
        }
    }
}
