// WGPU Renderer module for rendering behind webview

use std::borrow::Cow;
use std::sync::Mutex;
use tauri::{async_runtime::block_on, Manager};
use wgpu::{BackendOptions, Backends, InstanceDescriptor, InstanceFlags};

// Initialize WGPU rendering for a window
pub fn setup_wgpu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    crate::debug!("setup_wgpu: Starting WGPU initialization");

    let window = app.get_webview_window("main").unwrap();
    let size = window.inner_size()?;
    crate::debug!("setup_wgpu: Window size {}x{}", size.width, size.height);

    // Use OpenGL ES on Android (more stable with window handles), PRIMARY elsewhere
    #[cfg(target_os = "android")]
    let backends = Backends::GL;
    #[cfg(not(target_os = "android"))]
    let backends = Backends::PRIMARY;

    crate::debug!("setup_wgpu: Using backends {:?}", backends);

    let instance = wgpu::Instance::new(&InstanceDescriptor {
        backends,
        flags: InstanceFlags::default(),
        backend_options: BackendOptions::default(),
    });
    crate::debug!("setup_wgpu: Instance created");

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
        // Note: ctx.context() returns *mut c_void, which is a jobject reference to the Context/Activity
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

        // Wait loop for surface creation
        for _ in 0..50 {
            // Try to load the class
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
                // Convert JObject to JClass
                let fluyer_plugin_class: JClass = fluyer_plugin_class_obj.into();

                // Now get the static field from the class object we found
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
                        crate::debug!("setup_wgpu: Found valid surface object");
                        android_surface_obj = obj;
                        break;
                    }
                }
            }

            crate::debug!("setup_wgpu: Waiting for surface...");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if android_surface_obj.is_null() {
            return Err("Timed out waiting for Android Surface".into());
        }

        // Get the native window from the surface
        let native_window = unsafe {
            ndk::native_window::NativeWindow::from_surface(
                env.get_native_interface(),
                android_surface_obj.as_raw(),
            )
        }
        .ok_or("Failed to create native window from surface")?;

        let _native_window_ref = native_window.ptr().as_ptr();

        // Create raw window handle
        // FIXME: This is slightly risky as we're creating a handle for a window we don't own the lifecycle of perfectly
        // But for this purpose it should work if the surface stays valid.
        // We leak the native window to keep the reference valid for wgpu
        // Note: ndk::native_window::NativeWindow calls ANativeWindow_release on drop.
        // We need to keep it alive.
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
    let surface = instance.create_surface(window.clone())?;

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        // Request an adapter which can render to our surface
        compatible_surface: Some(&surface),
    }))
    .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
            r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#,
        )),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
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
            targets: &[Some(swapchain_format.into())],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    app.manage(WgpuSurface(surface));
    app.manage(render_pipeline);
    app.manage(device);
    app.manage(queue);
    app.manage(Mutex::new(config));

    // On Android we also need to keep the NativeWindow alive, possibly
    #[cfg(target_os = "android")]
    {
        // TODO: Store this in state if needed to prevent drop
        // For now, relies on the surface handle being valid
    }

    Ok(())
}

/// Wrapper for wgpu::Surface to make it Send + Sync
pub struct WgpuSurface<'a>(pub wgpu::Surface<'a>);

// SAFETY: The surface is only accessed from the main thread in Tauri
unsafe impl Send for WgpuSurface<'_> {}
unsafe impl Sync for WgpuSurface<'_> {}

/// Handle window resize for WGPU
pub fn handle_wgpu_resize(app_handle: &tauri::AppHandle, width: u32, height: u32) {
    let config = app_handle.state::<Mutex<wgpu::SurfaceConfiguration>>();
    let surface = app_handle.state::<WgpuSurface>();
    let device = app_handle.state::<wgpu::Device>();

    let mut config = config.lock().unwrap();
    config.width = if width > 0 { width } else { 1 };
    config.height = if height > 0 { height } else { 1 };
    surface.0.configure(&device, &config);
}

/// Render a frame using WGPU
pub fn render_frame(app_handle: &tauri::AppHandle) {
    let surface = app_handle.state::<WgpuSurface>();
    let render_pipeline = app_handle.state::<wgpu::RenderPipeline>();
    let device = app_handle.state::<wgpu::Device>();
    let queue = app_handle.state::<wgpu::Queue>();

    let frame = match surface.0.get_current_texture() {
        Ok(frame) => frame,
        Err(e) => {
            crate::debug!("Failed to acquire next swap chain texture: {:?}", e);
            return;
        }
    };

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass.set_pipeline(&render_pipeline);
        rpass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
}
