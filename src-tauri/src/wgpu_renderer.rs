// WGPU Renderer module for rendering behind webview

use std::borrow::Cow;
use std::sync::{Arc, Condvar, Mutex};
use tauri::{async_runtime::block_on, Manager};
use tauri_plugin_device_info::DeviceInfoExt;
use wgpu::util::DeviceExt;
use wgpu::{BackendOptions, Backends, InstanceDescriptor, InstanceFlags};

use image::RgbaImage;

use crate::state::app_handle;

#[cfg(any(target_os = "windows", target_os = "linux"))]
const CORNER_RADIUS: f32 = 8.0;
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
const CORNER_RADIUS: f32 = 0.0;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mix_ratio: f32,
    width: f32,
    height: f32,
    radius: f32,
}

struct TextureState {
    #[allow(dead_code)]
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

pub struct RendererState {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: Option<wgpu::Surface<'static>>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,

    // Background resources
    uniform_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,

    current_texture: Option<TextureState>,
    next_texture: Option<TextureState>,

    transition_start_time: Option<std::time::Instant>,

    // Cached image for restoration on Android resume
    cached_image: Option<RgbaImage>,

    // Redraw flag
    needs_redraw: bool,
}

pub struct SharedRenderer {
    pub state: Mutex<RendererState>,
    pub cond: Condvar,
}

unsafe impl Send for RendererState {}
unsafe impl Sync for RendererState {}

// Helper to create surface
pub fn create_surface(
    instance: &wgpu::Instance,
    app_handle: &tauri::AppHandle,
) -> Result<wgpu::Surface<'static>, Box<dyn std::error::Error>> {
    #[cfg(target_os = "android")]
    let surface = {
        use jni::objects::{JClass, JObject, JValue};
        use jni::JNIEnv;
        use raw_window_handle::{
            AndroidDisplayHandle, AndroidNdkWindowHandle, RawDisplayHandle, RawWindowHandle,
        };
        use std::ffi::c_void;

        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;

        // Get the application context and class loader
        let context = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

        let class_context = env.find_class("android/content/Context")?;
        let get_class_loader_method =
            env.get_method_id(class_context, "getClassLoader", "()Ljava/lang/ClassLoader;")?;

        let class_loader = unsafe {
            env.call_method_unchecked(
                &context,
                get_class_loader_method,
                jni::signature::ReturnType::Object,
                &[],
            )
        }?
        .l()?;

        let class_class_loader = env.find_class("java/lang/ClassLoader")?;
        let load_class_method = env.get_method_id(
            class_class_loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
        )?;

        let class_name_str = env.new_string("org.alvindimas05.fluyerplugin.FluyerPlugin")?;

        let mut android_surface_obj: JObject = JObject::null();
        crate::debug!("create_surface: Waiting for surface class load...");

        // Wait loop for surface creation
        for _ in 0..50 {
            let fluyer_plugin_class_value = unsafe {
                env.call_method_unchecked(
                    &class_loader,
                    load_class_method,
                    jni::signature::ReturnType::Object,
                    &[JValue::Object(&class_name_str).as_jni()],
                )
            };

            if let Ok(val) = fluyer_plugin_class_value {
                let fluyer_plugin_class_obj = val.l()?;
                let fluyer_plugin_class: JClass = fluyer_plugin_class_obj.into();

                let field_id = env.get_static_field_id(
                    &fluyer_plugin_class,
                    "surface",
                    "Landroid/view/Surface;",
                )?;
                let surface_obj_res = env.get_static_field_unchecked(
                    &fluyer_plugin_class,
                    field_id,
                    jni::signature::JavaType::Object("Landroid/view/Surface;".to_string()),
                );

                if let Ok(obj_val) = surface_obj_res {
                    let obj = obj_val.l()?;
                    if !obj.is_null() {
                        crate::debug!("create_surface: Found valid surface object");
                        android_surface_obj = obj;
                        break;
                    }
                }
            }

            crate::debug!("create_surface: Waiting for surface check iteration...");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if android_surface_obj.is_null() {
            return Err("Timed out waiting for Android Surface".into());
        }

        let native_window = unsafe {
            ndk::native_window::NativeWindow::from_surface(
                env.get_native_interface(),
                android_surface_obj.as_raw(),
            )
        }
        .ok_or("Failed to create native window from surface")?;

        let _native_window_ref = native_window.ptr().as_ptr();

        // Leak native window
        std::mem::forget(native_window);

        let handle = AndroidNdkWindowHandle::new(
            std::ptr::NonNull::new(_native_window_ref as *mut c_void).unwrap(),
        );
        let raw_window_handle = RawWindowHandle::AndroidNdk(handle);

        let display_handle = AndroidDisplayHandle::new();
        let raw_display_handle = RawDisplayHandle::Android(display_handle);

        unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle,
                raw_window_handle,
            })?
        }
    };

    #[cfg(not(target_os = "android"))]
    let surface = {
        let window = app_handle.get_webview_window("main").unwrap();
        instance.create_surface(window.clone())?
    };

    let surface: wgpu::Surface<'static> = unsafe { std::mem::transmute(surface) };
    Ok(surface)
}

fn create_texture_from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img: &RgbaImage,
) -> TextureState {
    let texture_size = wgpu::Extent3d {
        width: img.width(),
        height: img.height(),
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("Background Texture"),
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        img,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * img.width()),
            rows_per_image: Some(img.height()),
        },
        texture_size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    TextureState { texture, view }
}

// Initialize WGPU rendering for a window
pub fn setup_wgpu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    crate::debug!("setup_wgpu: Starting WGPU initialization");

    let window = app.get_webview_window("main").unwrap();

    // On Android, avoid using window.inner_size() which calls tao's
    // getCurrentWindowMetrics() — unavailable on Android < 11 (API 30).
    // Use JNI DisplayMetrics instead, which works on all Android versions.
    #[cfg(target_os = "android")]
    let size = {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        let context =
            unsafe { jni::objects::JObject::from_raw(ctx.context() as jni::sys::jobject) };
        let resources = env
            .call_method(
                &context,
                "getResources",
                "()Landroid/content/res/Resources;",
                &[],
            )?
            .l()?;
        let display_metrics = env
            .call_method(
                &resources,
                "getDisplayMetrics",
                "()Landroid/util/DisplayMetrics;",
                &[],
            )?
            .l()?;
        let width = env.get_field(&display_metrics, "widthPixels", "I")?.i()? as u32;
        let height = env.get_field(&display_metrics, "heightPixels", "I")?.i()? as u32;
        tauri::PhysicalSize::new(width, height)
    };
    #[cfg(not(target_os = "android"))]
    let size = window.inner_size()?;

    crate::debug!("setup_wgpu: Window size {}x{}", size.width, size.height);

    #[cfg(not(target_os = "macos"))]
    let backends = Backends::GL;
    #[cfg(target_os = "macos")]
    let backends = Backends::METAL;

    crate::debug!("setup_wgpu: Using backends {:?}", backends);

    let instance = wgpu::Instance::new(&InstanceDescriptor {
        backends,
        flags: InstanceFlags::default(),
        backend_options: BackendOptions::default(),
    });

    #[cfg(target_os = "android")]
    let surface = create_surface(&instance, &app.handle().clone())?;

    #[cfg(not(target_os = "android"))]
    let surface = create_surface(&instance, &app.handle().clone())?;

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }))
    .expect("Failed to find an appropriate adapter");

    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .expect("Failed to create device");

    // Shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
            r#"
            struct VertexOutput {
                @builtin(position) position: vec4<f32>,
                @location(0) tex_coords: vec2<f32>,
            };

            @vertex
            fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
                var pos = array<vec2<f32>, 4>(
                    vec2<f32>(-1.0, -1.0),
                    vec2<f32>( 1.0, -1.0),
                    vec2<f32>(-1.0,  1.0),
                    vec2<f32>( 1.0,  1.0)
                );
                
                var tex = array<vec2<f32>, 4>(
                    vec2<f32>(0.0, 1.0),
                    vec2<f32>(1.0, 1.0),
                    vec2<f32>(0.0, 0.0),
                    vec2<f32>(1.0, 0.0)
                );

                var out: VertexOutput;
                out.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
                out.tex_coords = tex[in_vertex_index];
                return out;
            }

            @group(0) @binding(0) var t_current: texture_2d<f32>;
            @group(0) @binding(1) var s_current: sampler;
            @group(0) @binding(2) var t_next: texture_2d<f32>;
            @group(0) @binding(3) var s_next: sampler;
            @group(0) @binding(4) var<uniform> uniforms: Uniforms;

            struct Uniforms {
                mix_ratio: f32,
                width: f32,
                height: f32,
                radius: f32,
            };

            fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
                let q = abs(p) - b + vec2<f32>(r, r);
                return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - r;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                let color_current = textureSample(t_current, s_current, in.tex_coords);
                let color_next = textureSample(t_next, s_next, in.tex_coords);
                let mixed_color = mix(color_current, color_next, uniforms.mix_ratio);

                let resolution = vec2<f32>(uniforms.width, uniforms.height);
                let p = (in.tex_coords - 0.5) * resolution;
                let b = resolution * 0.5;
                let radius = uniforms.radius;

                let dist = sd_rounded_box(p, b, radius);
                let alpha = 1.0 - smoothstep(-0.5, 0.5, dist);
                let final_alpha = mixed_color.a * alpha;

                // Multiply RGB by alpha to ensure fully transparent pixels are (0,0,0,0)
                // This is safe for PreMultiplied alpha and prevents artifacts if the 
                // compositor expects it.
                return vec4<f32>(mixed_color.rgb * alpha, final_alpha);
            }
        "#,
        )),
    });

    // Uniforms
    let uniforms = Uniforms {
        mix_ratio: 0.0,
        width: size.width as f32,
        height: size.height as f32,
        radius: CORNER_RADIUS,
    };
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: Some("Bind Group Layout"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: swapchain_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    // Prefer PostMultiplied or PreMultiplied for transparency
    let alpha_mode = swapchain_capabilities
        .alpha_modes
        .iter()
        .find(|&&mode| mode == wgpu::CompositeAlphaMode::PostMultiplied)
        .or_else(|| {
            swapchain_capabilities
                .alpha_modes
                .iter()
                .find(|&&mode| mode == wgpu::CompositeAlphaMode::PreMultiplied)
        })
        .copied()
        .unwrap_or(swapchain_capabilities.alpha_modes[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // Create initial 1x1 black texture so we can fade in from it
    let texture_size = wgpu::Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("Default Black Texture"),
        view_formats: &[],
    });

    // Write black pixel
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &[0, 0, 0, 0], // Fully transparent
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4),
            rows_per_image: Some(1),
        },
        texture_size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let _initial_texture_state = TextureState { texture, view };

    app.manage(Arc::new(SharedRenderer {
        state: Mutex::new(RendererState {
            instance,
            surface: Some(surface),
            device,
            queue,
            config,
            render_pipeline,
            uniform_buffer,
            bind_group_layout,
            sampler,
            current_texture: Some(_initial_texture_state), // Start with black texture
            next_texture: None,
            transition_start_time: None,
            cached_image: None,
            needs_redraw: false,
        }),
        cond: Condvar::new(),
    }));

    Ok(())
}

pub fn handle_wgpu_resize(app_handle: &tauri::AppHandle, width: u32, height: u32) {
    if let Some(shared) = app_handle.try_state::<Arc<SharedRenderer>>() {
        let mut state = shared.state.lock().unwrap();
        state.config.width = if width > 0 { width } else { 1 };
        state.config.height = if height > 0 { height } else { 1 };
        if let Some(surface) = &state.surface {
            surface.configure(&state.device, &state.config);
        }
    }
}

pub fn suspend_wgpu(app_handle: &tauri::AppHandle) {
    #[cfg(target_os = "android")]
    {
        crate::debug!("Suspending WGPU");
        if let Some(shared) = app_handle.try_state::<Arc<SharedRenderer>>() {
            let mut state = shared.state.lock().unwrap();
            state.surface = None;
        }
    }
}

pub fn resume_wgpu(app_handle: &tauri::AppHandle) {
    #[cfg(target_os = "android")]
    {
        crate::debug!("Resuming WGPU logic");
        std::thread::spawn({
            let app_handle = app_handle.clone();
            move || {
                if let Some(shared) = app_handle.try_state::<Arc<SharedRenderer>>() {
                    // Lock just to check
                    let has_surface = {
                        let state = shared.state.lock().unwrap();
                        state.surface.is_some()
                    };

                    if !has_surface {
                        crate::debug!("Resuming WGPU: Recreating surface");
                        // We need to create surface without holding the lock if it takes time?
                        // But create_surface uses JNI which is fast enough?
                        // Wait, create_surface has a loop up to 5s. We SHOULD NOT hold the lock.

                        // We need the instance from state.
                        let instance = {
                            let state = shared.state.lock().unwrap();
                            state.instance.clone() // Instance is cloneable (Arc internally usually)
                        };
                        // Actually wgpu::Instance is not Clone?
                        // wgpu::Instance is not Clone in 0.19? It usually is.
                        // Let's check docs or source. wgpu::Instance is a wrapper around `Arc<Context>`.
                        // Yes it is Clone.

                        match create_surface(&instance, &app_handle) {
                            Ok(surface) => {
                                let mut state = shared.state.lock().unwrap();
                                surface.configure(&state.device, &state.config);
                                state.surface = Some(surface);

                                crate::debug!("Resuming WGPU: Surface recreated and configured");
                                shared.cond.notify_one();
                            }
                            Err(e) => {
                                crate::error!("Failed to recreate surface on resume: {}", e);
                            }
                        }
                    }
                }
            }
        });
    }
}

pub fn restore_background() {
    if let Some(shared) = app_handle().try_state::<Arc<SharedRenderer>>() {
        let mut state = shared.state.lock().unwrap();

        if state.cached_image.is_some() {
            // We need to temporarily release the borrow on state fields to call create_texture_from_image
            // But we need state to be mutable later.
            // Split borrow works for fields, but create_texture_from_image calls usually aren't that simple with mutex guards.
            // Actually, simply block-limiting the borrow of fields should work if the method signature allows it.

            let new_texture_state = {
                let img = state.cached_image.as_ref().unwrap();
                create_texture_from_image(&state.device, &state.queue, img)
            };

            state.current_texture = Some(new_texture_state);
            state.next_texture = None;
            state.transition_start_time = None;
            state.needs_redraw = true;
            shared.cond.notify_one();
        }
    }
}

pub fn update_background(img: RgbaImage) {
    if let Some(shared) = app_handle().try_state::<Arc<SharedRenderer>>() {
        let mut state = shared.state.lock().unwrap();

        // Cache the image for restoration
        state.cached_image = Some(img);
        let img = state.cached_image.as_ref().unwrap();

        let new_texture_state = create_texture_from_image(&state.device, &state.queue, img);

        // Logic:
        // If no current, set current = new.
        // If current exists, set next = new, start transition.

        if state.current_texture.is_none() {
            state.current_texture = Some(new_texture_state);
        } else {
            state.next_texture = Some(new_texture_state);
            state.transition_start_time = Some(std::time::Instant::now());
            shared.cond.notify_one();
        }
    }
}

pub fn trigger_redraw() {
    if let Some(shared) = app_handle().try_state::<Arc<SharedRenderer>>() {
        let mut state = shared.state.lock().unwrap();
        state.needs_redraw = true;
        shared.cond.notify_one();
    }
}

pub fn start_render_loop(app_handle: tauri::AppHandle) {
    std::thread::spawn(move || {
        let shared = match app_handle.try_state::<Arc<SharedRenderer>>() {
            Some(s) => s,
            None => {
                crate::error!("start_render_loop: RendererState not found");
                return;
            }
        };

        let refresh_rate = app_handle
            .device_info()
            .get_display_info()
            .unwrap()
            .refresh_rate
            .unwrap();

        loop {
            let mut state = shared.state.lock().unwrap();

            // Wait until we have a transition active OR a redraw is requested
            while state.transition_start_time.is_none() && !state.needs_redraw {
                state = shared.cond.wait(state).unwrap();
            }

            // Consume redraw flag
            if state.needs_redraw {
                state.needs_redraw = false;
            }

            // Render and update logic
            // Update transition logic
            let mut mix_ratio = 0.0;
            let mut done_transition = false;

            if let Some(start_time) = state.transition_start_time {
                let elapsed = start_time.elapsed().as_secs_f32();
                mix_ratio = (elapsed / 0.75).min(1.0); // 750ms duration

                if mix_ratio >= 1.0 {
                    done_transition = true;
                }
            }

            if done_transition {
                state.current_texture = state.next_texture.take();
                state.transition_start_time = None;
                mix_ratio = 0.0;
            }

            // Update uniform
            let uniforms = Uniforms {
                mix_ratio,
                width: state.config.width as f32,
                height: state.config.height as f32,
                radius: CORNER_RADIUS,
            };
            state
                .queue
                .write_buffer(&state.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

            // Get generic view for missing next texture (reuse current)
            let current_view = &state.current_texture.as_ref().unwrap().view;
            let next_view = if let Some(next) = &state.next_texture {
                &next.view
            } else {
                current_view
            };

            let bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &state.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(current_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&state.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(next_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&state.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &state.uniform_buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                ],
                label: Some("Frame Bind Group"),
            });

            let frame = match state.surface.as_ref() {
                Some(surface) => match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        crate::warn!("Failed to get current texture: {e}");
                        // Drop lock before sleeping
                        drop(state);
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        continue;
                    }
                },
                None => {
                    // No surface, wait for resume
                    drop(state);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };

            let mut encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0, // Fully transparent
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rpass.set_pipeline(&state.render_pipeline);
                rpass.set_bind_group(0, &bind_group, &[]);
                rpass.draw(0..4, 0..1);
            }

            state.queue.submit(Some(encoder.finish()));
            frame.present();

            // Drop lock before sleeping to let other threads (resize/update) access it
            drop(state);

            std::thread::sleep(std::time::Duration::from_millis(
                (1000.0 / refresh_rate) as u64,
            ));
        }
    });
}
